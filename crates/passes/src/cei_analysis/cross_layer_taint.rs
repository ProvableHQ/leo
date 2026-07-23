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

use crate::{CompilerState, errors::cei_analyzer};

use leo_ast::*;
use leo_errors::Formatted;
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
    pub fn emit_warning(&self, warning: Formatted) {
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
                    } else if let Some(info) = self.taint_map.get(&member.identifier.name) {
                        // Shorthand initializer `S { x }` equivalent to `S { x: x }`.
                        // Desugaring happens in later passes, so we resolve it here.
                        taint.merge(info);
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

    /// Walk an expression tree, finding every external (or dynamic) call that returns
    /// a Future — whether at the root, behind projection wrappers (`.0`, `.field`,
    /// `as T`), or buried in a combinator (`(call(), x)`, `if c { call() } else { ... }`).
    /// For each, emit tainted-argument warnings and insert its source location into
    /// `sources`.  The bound variable inherits taint coupled to all collected sources.
    fn collect_future_call_sources(&self, expr: &Expression, sources: &mut IndexSet<Location>) {
        match expr {
            Expression::Call(call) => {
                if self.is_external_call(call)
                    && let Some(loc) = call.function.try_global_location()
                    && let Some(ret_type) = self.state.type_table.get(&call.id)
                    && Self::type_contains_future(&ret_type)
                {
                    let callee_desc = call.function.to_string();
                    self.warn_tainted_call_arguments(&call.arguments, &callee_desc, call.span);
                    sources.insert(loc.clone());
                }
                for arg in &call.arguments {
                    self.collect_future_call_sources(arg, sources);
                }
            }
            Expression::DynamicOp(dop) => {
                if let DynamicOpKind::Call { arguments, .. } = &dop.kind
                    && let Some(ret_type) = self.state.type_table.get(&dop.id)
                    && Self::type_contains_future(&ret_type)
                {
                    self.warn_tainted_call_arguments(arguments, "<dynamic call>", dop.span);
                    sources.insert(Location::dynamic());
                }
                self.collect_future_call_sources(&dop.target_program, sources);
                if let Some(network) = &dop.network {
                    self.collect_future_call_sources(network, sources);
                }
                match &dop.kind {
                    DynamicOpKind::Call { arguments, .. } | DynamicOpKind::Op { arguments, .. } => {
                        for arg in arguments {
                            self.collect_future_call_sources(arg, sources);
                        }
                    }
                    DynamicOpKind::Read { .. } => {}
                }
            }
            Expression::Tuple(t) => {
                for e in &t.elements {
                    self.collect_future_call_sources(e, sources);
                }
            }
            Expression::Array(a) => {
                for e in &a.elements {
                    self.collect_future_call_sources(e, sources);
                }
            }
            Expression::ArrayAccess(a) => {
                self.collect_future_call_sources(&a.array, sources);
                self.collect_future_call_sources(&a.index, sources);
            }
            Expression::Composite(c) => {
                for m in &c.members {
                    if let Some(e) = &m.expression {
                        self.collect_future_call_sources(e, sources);
                    }
                }
            }
            Expression::Ternary(t) => {
                self.collect_future_call_sources(&t.condition, sources);
                self.collect_future_call_sources(&t.if_true, sources);
                self.collect_future_call_sources(&t.if_false, sources);
            }
            Expression::Binary(b) => {
                self.collect_future_call_sources(&b.left, sources);
                self.collect_future_call_sources(&b.right, sources);
            }
            Expression::Unary(u) => self.collect_future_call_sources(&u.receiver, sources),
            Expression::Cast(c) => self.collect_future_call_sources(&c.expression, sources),
            Expression::TupleAccess(a) => self.collect_future_call_sources(&a.tuple, sources),
            Expression::MemberAccess(a) => self.collect_future_call_sources(&a.inner, sources),
            Expression::Repeat(r) => self.collect_future_call_sources(&r.expr, sources),
            Expression::Intrinsic(intr) => {
                for arg in &intr.arguments {
                    self.collect_future_call_sources(arg, sources);
                }
            }
            Expression::Path(_)
            | Expression::Literal(_)
            | Expression::Unit(_)
            | Expression::Err(_)
            | Expression::Async(_) => {}
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
            // `const` bindings are compile-time constants, so their RHS
            // cannot contain an external call and the bound value can never
            // be tainted. We still walk the RHS for taint uses but skip the
            // taint-map propagation.
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
                        self.emit_warning(cei_analyzer::tainted_value_in_finalize(sym, &future_id.source, path.span()));
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
                    } else if let Some(info) = self.taint_map.get(&member.identifier.name)
                        && info.is_tainted()
                    {
                        // Shorthand initializer `S { x }`, emit the warning against
                        // the field identifier's span since there is no separate value expression.
                        for future_id in &info.coupled_futures {
                            self.emit_warning(cei_analyzer::tainted_value_in_finalize(
                                member.identifier.name,
                                &future_id.source,
                                member.identifier.span(),
                            ));
                        }
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

        // Scan the entire RHS for external future-returning calls — at the root,
        // behind projection wrappers, or embedded in combinators alike. Tainted-arg
        // warnings are emitted as a side effect of this walk.
        let mut call_sources = IndexSet::new();
        self.collect_future_call_sources(&input.value, &mut call_sources);

        // Taint derived from references to previously-tainted variables (plus the
        // implicit taint of any enclosing tainted branch condition).
        let propagated = self.collect_taint_with_implicit(&input.value);

        // Taint freshly originated by call sources in the RHS.  Keyed by the bound
        // variable's name; `variable` only participates in `IndexSet` dedup, never
        // in warning text.
        let mut originated = TaintInfo::default();
        if !call_sources.is_empty() {
            let bound_name = match &input.place {
                DefinitionPlace::Single(id) => id.name,
                DefinitionPlace::Multiple(ids) => ids.first().map(|i| i.name).unwrap_or_else(|| Symbol::intern("_")),
            };
            for source in call_sources {
                originated.coupled_futures.insert(FutureId { variable: bound_name, source });
            }
        }

        match &input.place {
            DefinitionPlace::Single(id) => {
                // A pure-Future Single binding that *only* receives taint from a call
                // source (e.g. `let f: Final = ext_call();` or its embedded-combinator
                // equivalent) is just the finalize handle, no proof-time payload to
                // become stale.  If the binding also inherits propagated taint from a
                // reference (e.g. `let f: Final = start.1;` where `start` is tainted),
                // the conservative variable-granularity rule still applies and we
                // taint normally.
                let bound_is_future = matches!(self.state.type_table.get(&input.value.id()), Some(Type::Future(_)));
                if bound_is_future && propagated.coupled_futures.is_empty() {
                    return;
                }
                let mut merged = propagated;
                merged.coupled_futures.extend(originated.coupled_futures);
                if merged.is_tainted() {
                    self.taint_map.insert(id.name, merged);
                }
            }
            DefinitionPlace::Multiple(ids) => {
                let mut merged = propagated;
                merged.coupled_futures.extend(originated.coupled_futures);
                if !merged.is_tainted() {
                    return;
                }
                // Per-position: skip destructure slots whose element type is Future.
                if let Some(Type::Tuple(tuple_ty)) = self.state.type_table.get(&input.value.id()) {
                    let elements = tuple_ty.elements();
                    for (i, id) in ids.iter().enumerate() {
                        let is_future = elements.get(i).is_some_and(|t| matches!(t, Type::Future(_)));
                        if !is_future {
                            self.taint_map.insert(id.name, merged.clone());
                        }
                    }
                } else {
                    for id in ids {
                        self.taint_map.insert(id.name, merged.clone());
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
                    self.emit_warning(cei_analyzer::tainted_argument_to_external_call(
                        arg,
                        callee_desc,
                        &future_id.source,
                        span,
                    ));
                }
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
        // Skip `@test` transitions: they are off-chain fixtures where cross-layer
        // taint has no runtime meaning.
        if input.variant != Variant::EntryPoint || !input.has_final_output() || input.is_test() {
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
