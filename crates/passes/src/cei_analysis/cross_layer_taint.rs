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

use crate::CompilerState;

use leo_ast::*;
use leo_errors::CeiAnalyzerWarning;
use leo_span::Symbol;

use indexmap::{IndexMap, IndexSet};

/// Identifies a future by the variable name it was bound to and its source location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FutureId {
    /// The local variable name binding the future.
    pub variable: Symbol,
    /// The external call that produced this future (for diagnostics).
    pub source: Location,
}

/// Taint information for a single variable: which futures its value is coupled with.
#[derive(Debug, Clone, Default)]
pub struct TaintInfo {
    pub coupled_futures: IndexSet<FutureId>,
}

impl TaintInfo {
    /// Merge another taint into this one (union of coupled futures).
    pub fn merge(&mut self, other: &TaintInfo) {
        self.coupled_futures.extend(other.coupled_futures.iter().cloned());
    }

    pub fn is_tainted(&self) -> bool {
        !self.coupled_futures.is_empty()
    }
}

/// Union-merge two branch taint maps. A variable is tainted in the result
/// if it is tainted in either branch.
fn merge_taint_maps(mut a: IndexMap<Symbol, TaintInfo>, b: IndexMap<Symbol, TaintInfo>) -> IndexMap<Symbol, TaintInfo> {
    for (sym, info) in b {
        a.entry(sym).or_default().merge(&info);
    }
    a
}

pub struct CrossLayerTaintVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The current program name.
    pub current_program: Symbol,
    /// Taint map for the current transition body walk.
    pub taint_map: IndexMap<Symbol, TaintInfo>,
    /// Whether we are currently walking a transition body.
    pub in_transition: bool,
    /// Implicit taint from enclosing branch conditions.
    /// When a conditional's condition depends on a tainted value, any variable
    /// assigned inside the branch inherits this taint (control-flow dependence).
    pub implicit_taint: TaintInfo,
}

impl CrossLayerTaintVisitor<'_> {
    /// Emits a CEI analyzer warning.
    pub fn emit_warning(&self, warning: CeiAnalyzerWarning) {
        self.state.handler.emit_warning(warning);
    }

    /// Collect taint from an expression by looking up all referenced variables.
    pub fn collect_taint(&self, expr: &Expression) -> TaintInfo {
        let mut taint = TaintInfo::default();
        self.collect_taint_recursive(expr, &mut taint);
        taint
    }

    /// Collect taint from an expression, including implicit taint from enclosing
    /// tainted branch conditions (control-flow dependence).
    pub fn collect_taint_with_implicit(&self, expr: &Expression) -> TaintInfo {
        let mut taint = self.collect_taint(expr);
        taint.merge(&self.implicit_taint);
        taint
    }

    fn collect_taint_recursive(&self, expr: &Expression, taint: &mut TaintInfo) {
        match expr {
            Expression::Path(path) => {
                if let Some(sym) = path.try_local_symbol()
                    && let Some(info) = self.taint_map.get(&sym)
                {
                    taint.merge(info);
                }
            }
            Expression::Binary(bin) => {
                self.collect_taint_recursive(&bin.left, taint);
                self.collect_taint_recursive(&bin.right, taint);
            }
            Expression::Unary(un) => {
                self.collect_taint_recursive(&un.receiver, taint);
            }
            Expression::Call(call) => {
                // Conservative: if any argument is tainted, the return is tainted.
                for arg in &call.arguments {
                    self.collect_taint_recursive(arg, taint);
                }
            }
            Expression::Ternary(tern) => {
                self.collect_taint_recursive(&tern.condition, taint);
                self.collect_taint_recursive(&tern.if_true, taint);
                self.collect_taint_recursive(&tern.if_false, taint);
            }
            Expression::Cast(cast) => {
                self.collect_taint_recursive(&cast.expression, taint);
            }
            Expression::MemberAccess(access) => {
                self.collect_taint_recursive(&access.inner, taint);
            }
            Expression::TupleAccess(access) => {
                self.collect_taint_recursive(&access.tuple, taint);
            }
            Expression::Tuple(tuple) => {
                for elem in &tuple.elements {
                    self.collect_taint_recursive(elem, taint);
                }
            }
            Expression::Array(arr) => {
                for elem in &arr.elements {
                    self.collect_taint_recursive(elem, taint);
                }
            }
            Expression::ArrayAccess(access) => {
                self.collect_taint_recursive(&access.array, taint);
                self.collect_taint_recursive(&access.index, taint);
            }
            Expression::Repeat(rep) => {
                self.collect_taint_recursive(&rep.expr, taint);
            }
            Expression::Intrinsic(intr) => {
                for arg in &intr.arguments {
                    self.collect_taint_recursive(arg, taint);
                }
            }
            Expression::Composite(comp) => {
                for member in &comp.members {
                    if let Some(expr) = &member.expression {
                        self.collect_taint_recursive(expr, taint);
                    }
                }
            }
            Expression::DynamicOp(dop) => {
                self.collect_taint_recursive(&dop.target_program, taint);
                if let Some(ref network) = dop.network {
                    self.collect_taint_recursive(network, taint);
                }
                match &dop.kind {
                    DynamicOpKind::Call { arguments, .. } | DynamicOpKind::Op { arguments, .. } => {
                        for arg in arguments {
                            self.collect_taint_recursive(arg, taint);
                        }
                    }
                    DynamicOpKind::Read { .. } => {
                        // No sub-expressions beyond target/network (already visited above).
                    }
                }
            }
            // Leaf / irrelevant nodes.
            Expression::Literal(_) | Expression::Unit(_) | Expression::Err(_) | Expression::Async(_) => {}
        }
    }

    /// Check if an expression is a call to an external function.
    fn is_external_call(&self, call: &CallExpression) -> bool {
        if let Some(loc) = call.function.try_global_location() { loc.program != self.current_program } else { false }
    }

    /// Peel through wrappers (TupleAccess, MemberAccess, Cast) to find an underlying call.
    fn peel_to_call(expr: &Expression) -> Option<&Expression> {
        match expr {
            // Found the call.
            Expression::Call(_) => Some(expr),
            Expression::DynamicOp(dop) if matches!(dop.kind, DynamicOpKind::Call { .. }) => Some(expr),

            // Wrappers that access a sub-part of a call result, peel through.
            Expression::TupleAccess(ta) => Self::peel_to_call(&ta.tuple),
            Expression::MemberAccess(ma) => Self::peel_to_call(&ma.inner),
            Expression::Cast(c) => Self::peel_to_call(&c.expression),

            // Not a call or wrapper around one.
            Expression::Array(_)
            | Expression::ArrayAccess(_)
            | Expression::Async(_)
            | Expression::Binary(_)
            | Expression::Composite(_)
            | Expression::Err(_)
            | Expression::Intrinsic(_)
            | Expression::Literal(_)
            | Expression::Path(_)
            | Expression::Repeat(_)
            | Expression::Ternary(_)
            | Expression::Tuple(_)
            | Expression::DynamicOp(_)
            | Expression::Unary(_)
            | Expression::Unit(_) => None,
        }
    }

    /// Check if a type contains a Future.
    fn type_contains_future(ty: &Type) -> bool {
        match ty {
            Type::Future(_) => true,
            Type::Tuple(tuple) => tuple.elements().iter().any(Self::type_contains_future),
            _ => false,
        }
    }

    /// Walk the async block (finalize body) looking for uses of tainted variables.
    /// Also propagates taint through definitions and assignments within the block
    /// so that derived variables are tracked.
    fn check_async_block_for_taint(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.check_statement_for_taint(stmt);
        }
    }

    fn check_statement_for_taint(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Assert(assert) => match &assert.variant {
                AssertVariant::Assert(expr) => self.check_expression_for_taint(expr),
                AssertVariant::AssertEq(l, r) | AssertVariant::AssertNeq(l, r) => {
                    self.check_expression_for_taint(l);
                    self.check_expression_for_taint(r);
                }
            },
            Statement::Assign(assign) => {
                self.check_expression_for_taint(&assign.place);
                self.check_expression_for_taint(&assign.value);
                // Propagate taint within the finalize block,
                // including implicit taint from enclosing tainted branch conditions.
                if let Expression::Path(path) = &assign.place
                    && let Some(sym) = path.try_local_symbol()
                {
                    let rhs_taint = self.collect_taint_with_implicit(&assign.value);
                    if rhs_taint.is_tainted() {
                        self.taint_map.insert(sym, rhs_taint);
                    } else {
                        self.taint_map.swap_remove(&sym);
                    }
                }
            }
            Statement::Block(block) => self.check_async_block_for_taint(block),
            Statement::Conditional(cond) => {
                self.check_expression_for_taint(&cond.condition);

                // Propagate implicit taint from tainted branch conditions.
                let condition_taint = self.collect_taint(&cond.condition);
                let saved_implicit = self.implicit_taint.clone();
                self.implicit_taint.merge(&condition_taint);

                // Save taint map before branches.
                let saved_taint = self.taint_map.clone();

                self.check_async_block_for_taint(&cond.then);
                let then_taint = self.taint_map.clone();

                self.taint_map = saved_taint.clone();
                if let Some(otherwise) = &cond.otherwise {
                    self.check_statement_for_taint(otherwise);
                }
                let else_taint = self.taint_map.clone();

                // Union-merge: if taint exists in either branch, it persists.
                self.taint_map = merge_taint_maps(then_taint, else_taint);

                // Restore implicit taint to pre-conditional state.
                self.implicit_taint = saved_implicit;
            }
            Statement::Const(decl) => self.check_expression_for_taint(&decl.value),
            Statement::Definition(def) => {
                self.check_expression_for_taint(&def.value);
                // Propagate taint within the finalize block so derived variables are tracked,
                // including implicit taint from enclosing tainted branch conditions.
                let rhs_taint = self.collect_taint_with_implicit(&def.value);
                if rhs_taint.is_tainted() {
                    match &def.place {
                        DefinitionPlace::Single(id) => {
                            self.taint_map.insert(id.name, rhs_taint);
                        }
                        DefinitionPlace::Multiple(ids) => {
                            for id in ids {
                                self.taint_map.insert(id.name, rhs_taint.clone());
                            }
                        }
                    }
                }
            }
            Statement::Expression(expr_stmt) => self.check_expression_for_taint(&expr_stmt.expression),
            Statement::Iteration(iter) => {
                self.check_expression_for_taint(&iter.start);
                self.check_expression_for_taint(&iter.stop);
                self.check_async_block_for_taint(&iter.block);
            }
            Statement::Return(ret) => self.check_expression_for_taint(&ret.expression),
        }
    }

    fn check_expression_for_taint(&self, expr: &Expression) {
        match expr {
            Expression::Path(path) => {
                if let Some(sym) = path.try_local_symbol()
                    && let Some(info) = self.taint_map.get(&sym)
                    && info.is_tainted()
                {
                    // Emit warning for each coupled future source.
                    for future_id in &info.coupled_futures {
                        self.emit_warning(CeiAnalyzerWarning::tainted_value_in_finalize(
                            sym,
                            &future_id.source,
                            path.span(),
                        ));
                    }
                }
            }
            Expression::Binary(bin) => {
                self.check_expression_for_taint(&bin.left);
                self.check_expression_for_taint(&bin.right);
            }
            Expression::Unary(un) => {
                self.check_expression_for_taint(&un.receiver);
            }
            Expression::Call(call) => {
                for arg in &call.arguments {
                    self.check_expression_for_taint(arg);
                }
            }
            Expression::Intrinsic(intr) => {
                for arg in &intr.arguments {
                    self.check_expression_for_taint(arg);
                }
            }
            Expression::Ternary(tern) => {
                self.check_expression_for_taint(&tern.condition);
                self.check_expression_for_taint(&tern.if_true);
                self.check_expression_for_taint(&tern.if_false);
            }
            Expression::Cast(cast) => {
                self.check_expression_for_taint(&cast.expression);
            }
            Expression::MemberAccess(access) => {
                self.check_expression_for_taint(&access.inner);
            }
            Expression::TupleAccess(access) => {
                self.check_expression_for_taint(&access.tuple);
            }
            Expression::Tuple(tuple) => {
                for elem in &tuple.elements {
                    self.check_expression_for_taint(elem);
                }
            }
            Expression::Array(arr) => {
                for elem in &arr.elements {
                    self.check_expression_for_taint(elem);
                }
            }
            Expression::ArrayAccess(access) => {
                self.check_expression_for_taint(&access.array);
                self.check_expression_for_taint(&access.index);
            }
            Expression::Composite(comp) => {
                for member in &comp.members {
                    if let Some(expr) = &member.expression {
                        self.check_expression_for_taint(expr);
                    }
                }
            }
            Expression::Repeat(rep) => {
                self.check_expression_for_taint(&rep.expr);
            }
            Expression::DynamicOp(dop) => {
                self.check_expression_for_taint(&dop.target_program);
                if let Some(ref network) = dop.network {
                    self.check_expression_for_taint(network);
                }
                match &dop.kind {
                    DynamicOpKind::Call { arguments, .. } | DynamicOpKind::Op { arguments, .. } => {
                        for arg in arguments {
                            self.check_expression_for_taint(arg);
                        }
                    }
                    DynamicOpKind::Read { .. } => {}
                }
            }
            Expression::Literal(_) | Expression::Unit(_) | Expression::Err(_) | Expression::Async(_) => {}
        }
    }
}

impl AstVisitor for CrossLayerTaintVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_definition(&mut self, input: &DefinitionStatement) {
        if !self.in_transition {
            return;
        }

        // Check if the RHS contains an external call returning a future.
        // Peel through wrappers (TupleAccess, MemberAccess, Cast) to find the call.
        if let Some(call_expr) = Self::peel_to_call(&input.value) {
            // If wrappers were peeled, look up the bound type (the type after wrappers).
            // This lets us detect when only the Future component is accessed.
            let bound_type = self.state.type_table.get(&input.value.id());
            match call_expr {
                Expression::Call(call) if self.is_external_call(call) => {
                    let call_loc = call.function.try_global_location().cloned();
                    if let Some(ref loc) = call_loc
                        && let Some(ret_type) = self.state.type_table.get(&call.id)
                        && Self::type_contains_future(&ret_type)
                    {
                        // Callee has a finalize block, so arguments become finalize args.
                        // Warn if any argument carries taint from a prior external call.
                        let callee_desc = call.function.to_string();
                        self.warn_tainted_call_arguments(&call.arguments, &callee_desc, call.span);

                        self.handle_external_call_with_future(input, loc, &ret_type, bound_type.as_ref());
                        return;
                    }
                }
                Expression::DynamicOp(dop) => {
                    if let DynamicOpKind::Call { arguments, .. } = &dop.kind
                        && let Some(ret_type) = self.state.type_table.get(&dop.id)
                        && Self::type_contains_future(&ret_type)
                    {
                        // Callee has a finalize block, so arguments become finalize args.
                        // Warn if any argument carries taint from a prior external call.
                        self.warn_tainted_call_arguments(arguments, "<dynamic call>", dop.span);

                        let loc = Location::dynamic();
                        self.handle_external_call_with_future(input, &loc, &ret_type, bound_type.as_ref());
                        return;
                    }
                }
                _ => {}
            }
        }

        // For non-external-call definitions, propagate taint from the RHS,
        // including implicit taint from enclosing tainted branch conditions.
        let rhs_taint = self.collect_taint_with_implicit(&input.value);
        if rhs_taint.is_tainted() {
            match &input.place {
                DefinitionPlace::Single(id) => {
                    self.taint_map.insert(id.name, rhs_taint);
                }
                DefinitionPlace::Multiple(ids) => {
                    // Conservatively: all variables in the destructuring get the same taint.
                    for id in ids {
                        self.taint_map.insert(id.name, rhs_taint.clone());
                    }
                }
            }
        }
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        if !self.in_transition {
            return;
        }

        if let Expression::Path(path) = &input.place
            && let Some(sym) = path.try_local_symbol()
        {
            let rhs_taint = self.collect_taint_with_implicit(&input.value);
            if rhs_taint.is_tainted() {
                self.taint_map.insert(sym, rhs_taint);
            } else {
                // Clear taint when the variable is reassigned to a clean value
                // outside any tainted branch.
                self.taint_map.swap_remove(&sym);
            }
        }
    }

    fn visit_conditional(&mut self, input: &ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());

        // If the condition depends on a tainted value, assignments inside either
        // branch are implicitly tainted (control-flow dependence).  Merge into
        // the existing implicit_taint so nested conditionals accumulate.
        let condition_taint = self.collect_taint(&input.condition);
        let saved_implicit = self.implicit_taint.clone();
        self.implicit_taint.merge(&condition_taint);

        // Save taint map before branches.
        let saved_taint = self.taint_map.clone();

        // Visit then-branch.
        self.visit_block(&input.then);
        let then_taint = self.taint_map.clone();

        // Restore and visit else-branch.
        self.taint_map = saved_taint.clone();
        if let Some(otherwise) = &input.otherwise {
            match &**otherwise {
                Statement::Block(block) => self.visit_block(block),
                Statement::Conditional(cond) => self.visit_conditional(cond),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }
        }
        let else_taint = self.taint_map.clone();

        // Union-merge: if taint exists in either branch, it persists.
        self.taint_map = merge_taint_maps(then_taint, else_taint);

        // Restore implicit taint to pre-conditional state.
        self.implicit_taint = saved_implicit;
    }

    fn visit_async(&mut self, input: &AsyncExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        if !self.in_transition {
            return;
        }

        // The async block is the finalize body inline.
        // Check for uses of tainted variables from the transition scope.
        self.check_async_block_for_taint(&input.block);
    }
}

impl CrossLayerTaintVisitor<'_> {
    /// Check each argument of an external call for taint and emit warnings.
    fn warn_tainted_call_arguments(&self, arguments: &[Expression], callee_desc: &str, span: leo_span::Span) {
        for arg in arguments {
            let taint = self.collect_taint(arg);
            if taint.is_tainted() {
                for future_id in &taint.coupled_futures {
                    self.emit_warning(CeiAnalyzerWarning::tainted_argument_to_external_call(
                        arg,
                        callee_desc,
                        &future_id.source,
                        span,
                    ));
                }
            }
        }
    }

    /// Handle an external call that returns a value paired with a Future.
    /// Mark non-Future components as tainted, coupled to the Future's identity.
    /// `ret_type` is the full return type of the call; `bound_type` is the type
    /// actually bound by the definition (may differ when wrappers like TupleAccess are present).
    fn handle_external_call_with_future(
        &mut self,
        def: &DefinitionStatement,
        call_loc: &Location,
        ret_type: &Type,
        bound_type: Option<&Type>,
    ) {
        match &def.place {
            DefinitionPlace::Multiple(ids) => {
                // Match each destructured identifier to its tuple element type.
                if let Type::Tuple(tuple_ty) = ret_type {
                    let elements = tuple_ty.elements();

                    // First, find which identifiers bind Futures.
                    let mut future_ids = Vec::new();
                    for (i, elem_ty) in elements.iter().enumerate() {
                        if matches!(elem_ty, Type::Future(_))
                            && let Some(id) = ids.get(i)
                        {
                            future_ids.push(FutureId { variable: id.name, source: call_loc.clone() });
                        }
                    }

                    // Then, taint non-Future identifiers, coupled to the Futures found.
                    for (i, elem_ty) in elements.iter().enumerate() {
                        if !matches!(elem_ty, Type::Future(_))
                            && let Some(id) = ids.get(i)
                        {
                            let mut taint = TaintInfo::default();
                            taint.coupled_futures.extend(future_ids.iter().cloned());
                            self.taint_map.insert(id.name, taint);
                        }
                    }
                } else {
                    // Conservative fallback: return type is not a tuple (unexpected for
                    // a multi-binding destructure), taint all bound identifiers.
                    let future_id = FutureId { variable: ids[0].name, source: call_loc.clone() };
                    for id in ids {
                        let mut taint = TaintInfo::default();
                        taint.coupled_futures.insert(future_id.clone());
                        self.taint_map.insert(id.name, taint);
                    }
                }
            }
            DefinitionPlace::Single(id) => {
                // If the bound type is purely a Future, there is no proof-time value
                // to become stale — the variable is just the finalize handle.
                // Check the bound type first (accounts for wrappers like TupleAccess),
                // then fall back to the call's return type.
                let effective_type = bound_type.unwrap_or(ret_type);
                if matches!(effective_type, Type::Future(_)) {
                    return;
                }
                // The entire tuple is one variable, taint it coupled to any Future in the return.
                let mut taint = TaintInfo::default();
                taint.coupled_futures.insert(FutureId { variable: id.name, source: call_loc.clone() });
                self.taint_map.insert(id.name, taint);
            }
        }
    }
}

impl UnitVisitor for CrossLayerTaintVisitor<'_> {
    fn visit_program_scope(&mut self, input: &ProgramScope) {
        self.current_program = input.program_id.as_symbol();
        input.functions.iter().for_each(|(_, c)| self.visit_function(c));
    }

    fn visit_function(&mut self, input: &Function) {
        // Only analyze transitions that produce a Future (have a finalize block).
        if input.variant != Variant::EntryPoint || !input.has_final_output() {
            return;
        }

        self.in_transition = true;
        self.taint_map.clear();
        self.implicit_taint = TaintInfo::default();

        self.visit_block(&input.block);

        self.in_transition = false;
        self.taint_map.clear();
        self.implicit_taint = TaintInfo::default();
    }
}
