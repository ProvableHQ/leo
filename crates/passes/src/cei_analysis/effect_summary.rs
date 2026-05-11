// Copyright (C) 2019-2026 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{CompilerState, SymbolTable, VariableType};

use leo_ast::*;
use leo_span::Symbol;

use indexmap::IndexMap;

/// The CEI operation category for finalize-time analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CeiCategory {
    Check,
    Effect,
    Interaction,
}

/// Automaton state for the finalize-time CEI walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomatonState {
    BeforeInteraction,
    AfterInteraction,
}

/// Summary of what CEI operations a function (transitively) performs.
/// Computed bottom-up over the call graph.
#[derive(Debug, Clone, Default)]
pub struct EffectSummary {
    pub has_checks: bool,
    pub has_effects: bool,
    pub has_interactions: bool,
}

impl EffectSummary {
    /// Merge another summary into this one (union).
    pub fn merge(&mut self, other: &EffectSummary) {
        self.has_checks |= other.has_checks;
        self.has_effects |= other.has_effects;
        self.has_interactions |= other.has_interactions;
    }
}

/// Peel through `MemberAccess`, `TupleAccess`, and `ArrayAccess` wrappers on an
/// assignment LHS to find the root `Path`.  Returns `None` if the root is not a `Path`.
pub fn peel_assign_root(expr: &Expression) -> Option<&Path> {
    match expr {
        Expression::Path(path) => Some(path),
        Expression::MemberAccess(access) => peel_assign_root(&access.inner),
        Expression::TupleAccess(access) => peel_assign_root(&access.tuple),
        Expression::ArrayAccess(access) => peel_assign_root(&access.array),
        _ => None,
    }
}

/// Check if a path expression refers to a plain storage variable (not a mapping or vector).
/// Mappings and vectors are always accessed through intrinsics (which are classified separately),
/// so referencing them by name is not a state read/write in the CEI sense.
pub fn is_storage_variable(symbol_table: &SymbolTable, current_program: Symbol, path: &Path) -> bool {
    if let Some(loc) = path.try_global_location()
        && let Some(var) = symbol_table.lookup_global(current_program, loc)
        && var.declaration == VariableType::Storage
    {
        // Exclude mappings and vectors — they are only identifiers for intrinsic calls.
        if let Some(ty) = &var.type_ {
            return !ty.is_mapping() && !ty.is_vector();
        }
        return true;
    }
    false
}

/// Context for effect summary collection, providing access to the symbol table
/// so that storage variable reads/writes can be detected.
pub struct SummaryContext<'a> {
    pub symbol_table: &'a SymbolTable,
    pub current_program: Symbol,
}

/// Classify an intrinsic into a CEI category.
pub fn classify_intrinsic(intrinsic: &Intrinsic) -> Option<CeiCategory> {
    match intrinsic {
        // Checks (reads / environment queries)
        Intrinsic::MappingGet
        | Intrinsic::MappingGetOrUse
        | Intrinsic::MappingContains
        | Intrinsic::VectorGet
        | Intrinsic::VectorLen
        | Intrinsic::BlockHeight
        | Intrinsic::BlockTimestamp
        | Intrinsic::NetworkId
        | Intrinsic::ProgramOwner
        | Intrinsic::ProgramChecksum
        | Intrinsic::ProgramEdition
        | Intrinsic::SelfProgramOwner
        | Intrinsic::SelfAddress
        | Intrinsic::SelfCaller
        | Intrinsic::SelfChecksum
        | Intrinsic::SelfEdition
        | Intrinsic::SelfId
        | Intrinsic::SelfSigner
        | Intrinsic::DynamicContains
        | Intrinsic::DynamicGet
        | Intrinsic::DynamicGetOrUse
        | Intrinsic::SnarkVerify
        | Intrinsic::SnarkVerifyBatch => Some(CeiCategory::Check),

        // Effects (state mutations)
        Intrinsic::MappingSet
        | Intrinsic::MappingRemove
        | Intrinsic::VectorSet
        | Intrinsic::VectorPush
        | Intrinsic::VectorPop
        | Intrinsic::VectorClear
        | Intrinsic::VectorSwapRemove => Some(CeiCategory::Effect),

        // Interactions (external finalize execution)
        Intrinsic::FinalRun => Some(CeiCategory::Interaction),

        // Pure computations no on-chain state interaction.
        Intrinsic::ChaChaRand(_)
        | Intrinsic::Commit(_, _)
        | Intrinsic::ECDSAVerify(_)
        | Intrinsic::Hash(_, _)
        | Intrinsic::OptionalUnwrap
        | Intrinsic::OptionalUnwrapOr
        | Intrinsic::GroupToXCoordinate
        | Intrinsic::GroupToYCoordinate
        | Intrinsic::GroupGen
        | Intrinsic::AleoGenerator
        | Intrinsic::AleoGeneratorPowers
        | Intrinsic::SignatureVerify
        | Intrinsic::Serialize(_)
        | Intrinsic::Deserialize(_, _) => None,

        // Transition-only, cannot appear in finalize. Not a CEI concern.
        Intrinsic::DynamicCall => None,
    }
}

/// Compute effect summaries for all on-chain functions, bottom-up over the call graph.
pub fn compute_effect_summaries(state: &CompilerState) -> IndexMap<Location, EffectSummary> {
    let mut summaries = IndexMap::<Location, EffectSummary>::new();

    // Get bottom-up ordering of the call graph.
    let post_order = match state.call_graph.post_order() {
        Ok(order) => order,
        Err(_) => return summaries, // Cycle, earlier passes should have caught this.
    };

    for location in &post_order {
        let Some(func_symbol) = state.symbol_table.lookup_function(location.program, location) else {
            continue;
        };

        if !func_symbol.function.variant.is_onchain() {
            continue;
        }

        let ctx = SummaryContext { symbol_table: &state.symbol_table, current_program: location.program };
        let mut summary = EffectSummary::default();
        collect_block_effects_with_summaries(&func_symbol.function.block, &summaries, &mut summary, &ctx);
        summaries.insert(location.clone(), summary);
    }

    summaries
}

/// Recursively collect CEI effects from a block into the given summary.
/// This is public so the finalize visitor can compute local summaries for loop bodies.
pub fn collect_block_effects_with_summaries(
    block: &Block,
    computed: &IndexMap<Location, EffectSummary>,
    summary: &mut EffectSummary,
    ctx: &SummaryContext,
) {
    for stmt in &block.statements {
        collect_statement_effects(stmt, computed, summary, ctx);
    }
}

/// Collect CEI effects from a single statement.
fn collect_statement_effects(
    stmt: &Statement,
    computed: &IndexMap<Location, EffectSummary>,
    summary: &mut EffectSummary,
    ctx: &SummaryContext,
) {
    match stmt {
        Statement::Assert(assert) => {
            // this is a check by definition
            summary.has_checks = true;

            // Recurse into sub-expressions so nested efffects/interactions still propagate into the summary.
            match &assert.variant {
                AssertVariant::Assert(expr) => {
                    collect_expression_effects(expr, computed, summary, ctx);
                }
                AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                    collect_expression_effects(left, computed, summary, ctx);
                    collect_expression_effects(right, computed, summary, ctx);
                }
            }
        }
        Statement::Assign(assign) => {
            // Check if LHS root is a storage variable write.
            if let Some(root) = peel_assign_root(&assign.place)
                && is_storage_variable(ctx.symbol_table, ctx.current_program, root)
            {
                summary.has_effects = true;
            }
            // Visit index sub-expressions in the LHS (e.g., `i` in `arr[i] = x`) for read effects,
            // but skip the root path to avoid misclassifying the write target as a check.
            collect_assign_lhs_effects(&assign.place, computed, summary, ctx);
            collect_expression_effects(&assign.value, computed, summary, ctx);
        }
        Statement::Block(block) => {
            collect_block_effects_with_summaries(block, computed, summary, ctx);
        }
        Statement::Conditional(cond) => {
            collect_expression_effects(&cond.condition, computed, summary, ctx);
            collect_block_effects_with_summaries(&cond.then, computed, summary, ctx);
            if let Some(otherwise) = &cond.otherwise {
                collect_statement_effects(otherwise, computed, summary, ctx);
            }
        }
        Statement::Const(decl) => {
            collect_expression_effects(&decl.value, computed, summary, ctx);
        }
        Statement::Definition(def) => {
            collect_expression_effects(&def.value, computed, summary, ctx);
        }
        Statement::Expression(expr_stmt) => {
            collect_expression_effects(&expr_stmt.expression, computed, summary, ctx);
        }
        Statement::Iteration(iter) => {
            collect_expression_effects(&iter.start, computed, summary, ctx);
            collect_expression_effects(&iter.stop, computed, summary, ctx);
            collect_block_effects_with_summaries(&iter.block, computed, summary, ctx);
        }
        Statement::Return(ret) => {
            collect_expression_effects(&ret.expression, computed, summary, ctx);
        }
    }
}

/// Visit only the index/key sub-expressions of an assignment LHS for read effects.
/// Skips the root path (the write target) to avoid misclassifying it as a check.
fn collect_assign_lhs_effects(
    expr: &Expression,
    computed: &IndexMap<Location, EffectSummary>,
    summary: &mut EffectSummary,
    ctx: &SummaryContext,
) {
    match expr {
        // Root path — skip (this is the write target, not a read).
        Expression::Path(_) => {}
        Expression::MemberAccess(access) => collect_assign_lhs_effects(&access.inner, computed, summary, ctx),
        Expression::TupleAccess(access) => collect_assign_lhs_effects(&access.tuple, computed, summary, ctx),
        Expression::ArrayAccess(access) => {
            // The array root is the write target — recurse to skip it.
            collect_assign_lhs_effects(&access.array, computed, summary, ctx);
            // The index is a read — collect its effects.
            collect_expression_effects(&access.index, computed, summary, ctx);
        }
        _ => {}
    }
}

/// Collect CEI effects from an expression.
fn collect_expression_effects(
    expr: &Expression,
    computed: &IndexMap<Location, EffectSummary>,
    summary: &mut EffectSummary,
    ctx: &SummaryContext,
) {
    match expr {
        Expression::Intrinsic(intr) => {
            if let Some(intrinsic) = Intrinsic::from_symbol(intr.name, &intr.type_parameters)
                && let Some(category) = classify_intrinsic(&intrinsic)
            {
                match category {
                    CeiCategory::Check => summary.has_checks = true,
                    CeiCategory::Effect => summary.has_effects = true,
                    CeiCategory::Interaction => summary.has_interactions = true,
                }
            }
            for arg in &intr.arguments {
                collect_expression_effects(arg, computed, summary, ctx);
            }
        }
        Expression::Call(call) => {
            if let Some(loc) = call.function.try_global_location()
                && let Some(callee_summary) = computed.get(loc)
            {
                summary.merge(callee_summary);
            }
            for arg in &call.arguments {
                collect_expression_effects(arg, computed, summary, ctx);
            }
        }
        Expression::Binary(bin) => {
            collect_expression_effects(&bin.left, computed, summary, ctx);
            collect_expression_effects(&bin.right, computed, summary, ctx);
        }
        Expression::Unary(un) => {
            collect_expression_effects(&un.receiver, computed, summary, ctx);
        }
        Expression::Ternary(tern) => {
            collect_expression_effects(&tern.condition, computed, summary, ctx);
            collect_expression_effects(&tern.if_true, computed, summary, ctx);
            collect_expression_effects(&tern.if_false, computed, summary, ctx);
        }
        Expression::Cast(cast) => {
            collect_expression_effects(&cast.expression, computed, summary, ctx);
        }
        Expression::Tuple(tuple) => {
            for elem in &tuple.elements {
                collect_expression_effects(elem, computed, summary, ctx);
            }
        }
        Expression::Array(arr) => {
            for elem in &arr.elements {
                collect_expression_effects(elem, computed, summary, ctx);
            }
        }
        Expression::ArrayAccess(access) => {
            collect_expression_effects(&access.array, computed, summary, ctx);
            collect_expression_effects(&access.index, computed, summary, ctx);
        }
        Expression::MemberAccess(access) => {
            collect_expression_effects(&access.inner, computed, summary, ctx);
        }
        Expression::TupleAccess(access) => {
            collect_expression_effects(&access.tuple, computed, summary, ctx);
        }
        Expression::Composite(comp) => {
            for member in &comp.members {
                if let Some(expr) = &member.expression {
                    collect_expression_effects(expr, computed, summary, ctx);
                }
            }
        }
        Expression::Repeat(rep) => {
            collect_expression_effects(&rep.expr, computed, summary, ctx);
            collect_expression_effects(&rep.count, computed, summary, ctx);
        }
        Expression::Async(async_expr) => {
            collect_block_effects_with_summaries(&async_expr.block, computed, summary, ctx);
        }
        Expression::DynamicOp(dop) => {
            collect_expression_effects(&dop.target_program, computed, summary, ctx);
            if let Some(ref network) = dop.network {
                collect_expression_effects(network, computed, summary, ctx);
            }
            match &dop.kind {
                DynamicOpKind::Call { arguments, .. } => {
                    // Transition-only; won't appear in on-chain functions, but visit args for completeness.
                    for arg in arguments {
                        collect_expression_effects(arg, computed, summary, ctx);
                    }
                }
                DynamicOpKind::Read { .. } => {
                    // Reading external storage is a Check.
                    summary.has_checks = true;
                }
                DynamicOpKind::Op { arguments, .. } => {
                    // Dynamic storage ops (get, get_or_use, contains, len) are all reads/checks.
                    summary.has_checks = true;
                    for arg in arguments {
                        collect_expression_effects(arg, computed, summary, ctx);
                    }
                }
            }
        }
        Expression::Path(path) => {
            // Storage variable read.
            if is_storage_variable(ctx.symbol_table, ctx.current_program, path) {
                summary.has_checks = true;
            }
        }
        // Leaf expressions: no sub-effects.
        Expression::Literal(_) | Expression::Unit(_) | Expression::Err(_) => {}
    }
}
