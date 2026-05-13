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

use crate::{
    CompilerState,
    cei_analysis::effect_summary::{AutomatonState, CeiCategory, EffectSummary, classify_intrinsic, peel_assign_root},
    errors::cei_analyzer,
};

use leo_ast::*;
use leo_errors::Formatted;
use leo_span::{Span, Symbol};

use indexmap::IndexMap;

pub struct FinalizeCeiVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The current program name.
    pub current_program: Symbol,
    /// Pre-computed effect summaries for all on-chain functions.
    pub effect_summaries: IndexMap<Location, EffectSummary>,
    /// The current automaton state during a function walk.
    pub automaton_state: AutomatonState,
    /// Span of the interaction that caused the transition to AfterInteraction.
    pub interaction_span: Option<Span>,
    /// The variant of the function currently being walked.
    pub current_variant: Option<Variant>,
    /// Whether we are currently inside finalize-level code (on-chain function or inline async block).
    /// CEI checks are only active when this is true.
    pub in_finalize: bool,
}

impl FinalizeCeiVisitor<'_> {
    /// Emits a CEI analyzer warning.
    pub fn emit_warning(&self, warning: Formatted) {
        self.state.handler.emit_warning(warning);
    }

    fn is_storage_variable(&self, path: &Path) -> bool {
        crate::cei_analysis::effect_summary::is_storage_variable(&self.state.symbol_table, self.current_program, path)
    }

    /// Transition to AfterInteraction state.
    fn transition_to_after_interaction(&mut self, span: Span) {
        self.automaton_state = AutomatonState::AfterInteraction;
        self.interaction_span = Some(span);
    }

    /// Visit only the index/key sub-expressions of an assignment LHS for read effects.
    /// Skips the root path (the write target) to avoid misclassifying it as a read.
    fn visit_assign_lhs_reads(&mut self, expr: &Expression) {
        match expr {
            // Root path — skip (this is the write target, not a read).
            Expression::Path(_) => {}
            Expression::MemberAccess(access) => self.visit_assign_lhs_reads(&access.inner),
            Expression::TupleAccess(access) => self.visit_assign_lhs_reads(&access.tuple),
            Expression::ArrayAccess(access) => {
                // The array root is the write target — recurse to skip it.
                self.visit_assign_lhs_reads(&access.array);
                // The index is a read — visit it normally.
                self.visit_expression(&access.index, &Default::default());
            }
            _ => {}
        }
    }

    /// Warn if a check or effect occurs after an interaction.
    fn warn_if_after_interaction(&self, category: CeiCategory, description: &str, span: Span) {
        if self.automaton_state == AutomatonState::AfterInteraction {
            match category {
                CeiCategory::Check => {
                    self.emit_warning(cei_analyzer::check_after_interaction(description, span));
                }
                CeiCategory::Effect => {
                    self.emit_warning(cei_analyzer::effect_after_interaction(description, span));
                }
                CeiCategory::Interaction => {} // Interactions after interactions are fine.
            }
        }
    }
}

impl AstVisitor for FinalizeCeiVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_intrinsic(&mut self, input: &IntrinsicExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Visit arguments first (they are evaluated before the intrinsic executes).
        for arg in &input.arguments {
            self.visit_expression(arg, &Default::default());
        }

        if self.in_finalize
            && let Some(intrinsic) = Intrinsic::from_symbol(input.name, &input.type_parameters)
            && let Some(category) = classify_intrinsic(&intrinsic)
        {
            match category {
                CeiCategory::Interaction => {
                    self.transition_to_after_interaction(input.span());
                }
                CeiCategory::Check | CeiCategory::Effect => {
                    self.warn_if_after_interaction(category, &input.name.to_string(), input.span());
                }
            }
        }
    }

    fn visit_call(&mut self, input: &CallExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Visit arguments first (they are evaluated before the call).
        for arg in &input.arguments {
            self.visit_expression(arg, &Default::default());
        }

        if self.in_finalize
            && let Some(loc) = input.function.try_global_location()
            && let Some(callee_summary) = self.effect_summaries.get(loc)
        {
            // If we're already after an interaction and the callee has checks or effects, warn.
            if self.automaton_state == AutomatonState::AfterInteraction
                && (callee_summary.has_checks || callee_summary.has_effects)
            {
                self.emit_warning(cei_analyzer::callee_has_effects_after_interaction(&input.function, input.span()));
            }

            // If the callee has interactions, transition to AfterInteraction.
            if callee_summary.has_interactions {
                self.transition_to_after_interaction(input.span());
            }
        }
    }

    fn visit_dynamic_op(&mut self, input: &DynamicOpExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Visit sub-expressions first.
        self.visit_expression(&input.target_program, &Default::default());
        if let Some(ref network) = input.network {
            self.visit_expression(network, &Default::default());
        }
        match &input.kind {
            DynamicOpKind::Call { arguments, .. } => {
                // Transition-only; shouldn't appear in finalize. Visit args for completeness.
                for arg in arguments {
                    self.visit_expression(arg, &Default::default());
                }
            }
            DynamicOpKind::Read { storage } => {
                // Reading external storage is a Check.
                if self.in_finalize {
                    self.warn_if_after_interaction(
                        CeiCategory::Check,
                        &format!("dynamic storage read `{storage}`"),
                        input.span(),
                    );
                }
            }
            DynamicOpKind::Op { op, arguments, .. } => {
                for arg in arguments {
                    self.visit_expression(arg, &Default::default());
                }
                // Dynamic storage ops (get, get_or_use, contains, len) are all reads/checks.
                if self.in_finalize {
                    self.warn_if_after_interaction(
                        CeiCategory::Check,
                        &format!("dynamic storage op `{op}`"),
                        input.span(),
                    );
                }
            }
        }
    }

    fn visit_assert(&mut self, input: &AssertStatement) {
        match &input.variant {
            AssertVariant::Assert(expr) => {
                self.visit_expression(expr, &Default::default());
            }
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                self.visit_expression(left, &Default::default());
                self.visit_expression(right, &Default::default());
            }
        }

        if self.in_finalize {
            self.warn_if_after_interaction(CeiCategory::Check, "assert", input.span);
        }
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        self.visit_assign_lhs_reads(&input.place);
        self.visit_expression(&input.value, &Default::default());

        if self.in_finalize {
            // Check if the LHS root is a storage variable write.
            if let Some(root) = peel_assign_root(&input.place)
                && self.is_storage_variable(root)
            {
                self.warn_if_after_interaction(CeiCategory::Effect, "storage variable write", input.span);
            }
        }
    }

    fn visit_path(&mut self, input: &Path, _additional: &Self::AdditionalInput) -> Self::Output {
        // If reading a storage variable after an interaction, warn.
        if self.in_finalize && self.is_storage_variable(input) {
            self.warn_if_after_interaction(CeiCategory::Check, "storage variable read", input.span());
        }
    }

    fn visit_conditional(&mut self, input: &ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());

        // Save state before branches.
        let saved_state = self.automaton_state;
        let saved_span = self.interaction_span;

        // Visit then branch.
        self.visit_block(&input.then);
        let then_state = self.automaton_state;
        let then_span = self.interaction_span;

        // Restore and visit else branch.
        self.automaton_state = saved_state;
        self.interaction_span = saved_span;

        if let Some(otherwise) = &input.otherwise {
            match &**otherwise {
                Statement::Block(block) => self.visit_block(block),
                Statement::Conditional(cond) => self.visit_conditional(cond),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }
        }
        let else_state = self.automaton_state;

        // Merge: if either branch reached AfterInteraction, subsequent code is AfterInteraction.
        if then_state == AutomatonState::AfterInteraction || else_state == AutomatonState::AfterInteraction {
            self.automaton_state = AutomatonState::AfterInteraction;
            // Prefer the then-branch span (first in source order), falling back to else-branch.
            self.interaction_span = then_span.or(self.interaction_span);
        }
    }

    fn visit_iteration(&mut self, input: &IterationStatement) {
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());

        // Compute a local effect summary for the loop body to check for violations.
        let mut loop_summary = EffectSummary::default();
        let ctx = crate::cei_analysis::effect_summary::SummaryContext {
            symbol_table: &self.state.symbol_table,
            current_program: self.current_program,
        };
        crate::cei_analysis::effect_summary::collect_block_effects_with_summaries(
            &input.block,
            &self.effect_summaries,
            &mut loop_summary,
            &ctx,
        );

        // If the loop body contains interactions alongside checks or effects, warn.
        if loop_summary.has_interactions && (loop_summary.has_checks || loop_summary.has_effects) {
            self.emit_warning(cei_analyzer::cei_violation_in_loop(input.variable.span()));
        }

        // Walk the loop body for detailed per-statement warnings.
        self.visit_block(&input.block);

        // After the loop: if the body had interactions, we're in AfterInteraction.
        if loop_summary.has_interactions {
            self.transition_to_after_interaction(input.span);
        }
    }

    fn visit_async(&mut self, input: &AsyncExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Inline async blocks (`return final { ... }`) contain finalize code
        // inside EntryPoint functions. Analyze them for CEI violations.
        let saved_automaton = self.automaton_state;
        let saved_span = self.interaction_span;
        let saved_in_finalize = self.in_finalize;

        self.automaton_state = AutomatonState::BeforeInteraction;
        self.interaction_span = None;
        self.in_finalize = true;

        self.visit_block(&input.block);

        self.automaton_state = saved_automaton;
        self.interaction_span = saved_span;
        self.in_finalize = saved_in_finalize;
    }
}

impl UnitVisitor for FinalizeCeiVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        self.current_program = input.program_id.as_symbol();
        input.functions.iter().for_each(|(_, c)| self.visit_function(c));
    }

    fn visit_function(&mut self, input: &Function) {
        self.current_variant = Some(input.variant);

        if input.variant.is_onchain() {
            // Dedicated on-chain function (final fn / finalize).
            self.automaton_state = AutomatonState::BeforeInteraction;
            self.interaction_span = None;
            self.in_finalize = true;
            self.visit_block(&input.block);
            self.in_finalize = false;
        } else {
            // For non-onchain functions (e.g., EntryPoint), still walk the body
            // so we can encounter inline async blocks via visit_async.
            self.in_finalize = false;
            self.visit_block(&input.block);
        }
    }
}
