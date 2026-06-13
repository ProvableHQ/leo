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

//! Phase 1 of the unused-items pass: a pure analysis walk that gathers
//! reference information (used imports, used globals, composite-dependency
//! edges) and records unused-local findings as each lexical scope drains. It
//! produces only data — no diagnostics are built or emitted here; phase 2 does
//! both.

use super::{CollectedUses, LocalFinding, LocalFindingKind, name_starts_with_underscore};

use leo_ast::*;
use leo_span::{Span, Symbol, sym};

use indexmap::{IndexMap, IndexSet};

#[derive(Copy, Clone)]
enum BindingKind {
    /// A `let` binding, function/iter-var parameter, or `for` iteration variable.
    Variable,
    /// A local `const` declaration. Emits `constant X is never used` instead of the
    /// `unused variable` text used for the other kinds.
    Const,
}

struct Binding {
    name: Symbol,
    span: Span,
    kind: BindingKind,
    referenced: bool,
    /// Cached `name_starts_with_underscore(name)`, computed once at declaration so the
    /// per-read and per-drain checks don't re-resolve the interned string each time.
    starts_with_underscore: bool,
}

pub(super) struct ReferenceCollector<'a> {
    /// Read-only compiler state; consulted for `call_count` only.
    state: &'a crate::CompilerState,

    // --- Traversal state: a cursor and scope bookkeeping, mutated as the walk proceeds. ---
    /// Current compilation unit while walking. Pushed/popped by `visit_program_scope`,
    /// `visit_module`, and `visit_library`.
    current_unit: Symbol,
    /// Current module path prefix (empty at program/library top level).
    current_module: Vec<Symbol>,
    /// True while walking a library compilation unit. Libraries never report their functions
    /// as unused, so body-level warnings must always fire inside them.
    is_library: bool,
    /// True while walking the body of a program function the checker phase will report as
    /// unused. Suppresses per-binding warnings to avoid double-reporting on a dead function.
    suppress_body_warnings: bool,
    /// Locals currently in scope, in declaration order. `exit_scope` drains the tail.
    bindings: Vec<Binding>,
    /// `bindings.len()` snapshot at each scope entry; `exit_scope` truncates back to it.
    scope_starts: Vec<usize>,
    /// `name → stack of indices into `bindings``; the top is the innermost binding, so a
    /// reference resolves in O(1). Kept in sync with `bindings` by `declare` and `exit_scope`.
    name_index: IndexMap<Symbol, Vec<usize>>,

    // --- Collected results: accumulated during the walk and drained by `into_data`. ---
    /// Imports referenced by some path.
    used_imports: IndexSet<Symbol>,
    /// Globals (functions, structs, consts) referenced by some path.
    used_globals: IndexSet<Location>,
    /// Member-type edges: `composite_deps[A]` are the composites referenced by `A`'s members.
    /// Folded into `live_composites` by the reachability scan in `into_data`.
    composite_deps: IndexMap<Location, IndexSet<Location>>,
    /// Always-live composite roots (records + library top-level structs); a scan input too.
    composite_roots: Vec<Location>,
    /// Unused-local findings, recorded in emission order and handed to phase 2.
    local_findings: Vec<LocalFinding>,
}

impl<'a> ReferenceCollector<'a> {
    pub(super) fn new(state: &'a crate::CompilerState) -> Self {
        Self {
            state,
            current_unit: Symbol::intern(""),
            current_module: Vec::new(),
            is_library: false,
            suppress_body_warnings: false,
            bindings: Vec::new(),
            scope_starts: Vec::new(),
            name_index: IndexMap::new(),
            used_imports: IndexSet::new(),
            used_globals: IndexSet::new(),
            composite_deps: IndexMap::new(),
            composite_roots: Vec::new(),
            local_findings: Vec::new(),
        }
    }

    pub(super) fn into_data(self) -> CollectedUses {
        let live_composites = self.compute_live_composites();
        CollectedUses {
            used_imports: self.used_imports,
            used_globals: self.used_globals,
            live_composites,
            local_findings: self.local_findings,
        }
    }

    /// Depth-first reachability scan over the composite-dependency graph from the live-root
    /// set: `composite_roots` plus any composite directly referenced from user code
    /// (`used_globals`).
    fn compute_live_composites(&self) -> IndexSet<Location> {
        let mut live: IndexSet<Location> = IndexSet::new();
        let mut queue: Vec<Location> = Vec::with_capacity(self.composite_roots.len() + self.used_globals.len());
        queue.extend(self.composite_roots.iter().chain(&self.used_globals).cloned());
        while let Some(loc) = queue.pop() {
            if !live.insert(loc.clone()) {
                continue;
            }
            if let Some(children) = self.composite_deps.get(&loc) {
                for child in children {
                    queue.push(child.clone());
                }
            }
        }
        live
    }

    fn current_location(&self, name: Symbol) -> Location {
        Location::new(self.current_unit, self.current_module.iter().copied().chain(std::iter::once(name)).collect())
    }

    /// Run `body` with the unit/module cursor set to `(unit, module)`, restoring the previous
    /// values afterward. Centralizes the save/restore so the cursor can never leak across units.
    fn in_unit_scope(&mut self, unit: Symbol, module: Vec<Symbol>, body: impl FnOnce(&mut Self)) {
        let prev_unit = std::mem::replace(&mut self.current_unit, unit);
        let prev_module = std::mem::replace(&mut self.current_module, module);
        body(self);
        self.current_unit = prev_unit;
        self.current_module = prev_module;
    }

    fn declare(&mut self, name: Symbol, span: Span, kind: BindingKind) {
        let index = self.bindings.len();
        let starts_with_underscore = name_starts_with_underscore(name);
        self.bindings.push(Binding { name, span, kind, referenced: false, starts_with_underscore });
        self.name_index.entry(name).or_default().push(index);
    }

    fn enter_scope(&mut self) {
        self.scope_starts.push(self.bindings.len());
    }

    /// Drains every binding that was introduced inside this scope, recording a finding for
    /// each that was never read (and isn't `_`-prefixed).
    fn exit_scope(&mut self) {
        let start = self.scope_starts.pop().expect("enter/exit_scope must balance");
        let suppress = self.suppress_body_warnings;
        for b in self.bindings.drain(start..) {
            // Remove this binding's index from its name stack; it is always the top entry.
            if let Some(stack) = self.name_index.get_mut(&b.name) {
                let popped = stack.pop();
                debug_assert!(popped.map(|i| i >= start).unwrap_or(false), "name_index out of sync with bindings");
            }
            // Inside a dead program function the `function is never used` warning already
            // covers everything here; suppress per-binding warnings to avoid double-reporting.
            if suppress {
                continue;
            }
            // A leading `_` signals intentionally-unused; safe locally since local names
            // never reach the VM.
            if b.referenced || b.starts_with_underscore {
                continue;
            }
            let kind = match b.kind {
                BindingKind::Variable => LocalFindingKind::UnusedVariable,
                BindingKind::Const => LocalFindingKind::UnusedConst,
            };
            self.local_findings.push(LocalFinding { kind, name: b.name, span: b.span });
        }
    }

    /// Mark the innermost in-scope binding for `name` as referenced. Returns `true` if a
    /// binding matched, `false` otherwise (the path resolved to a global, handled via
    /// `used_globals`, or to nothing, in which case earlier passes have already errored).
    fn note_local_use(&mut self, name: Symbol) -> bool {
        let mut warn_underscore: Option<(Symbol, Span)> = None;
        let matched = if let Some(&idx) = self.name_index.get(&name).and_then(|stack| stack.last()) {
            let b = &mut self.bindings[idx];
            // Reading a `_`-prefixed binding defeats the silencing marker; warn once on the
            // first read (subsequent reads see `referenced == true` and stay silent).
            if !b.referenced && b.starts_with_underscore {
                warn_underscore = Some((b.name, b.span));
            }
            b.referenced = true;
            true
        } else {
            false
        };
        if let Some((name, span)) = warn_underscore
            && !self.suppress_body_warnings
        {
            self.local_findings.push(LocalFinding { kind: LocalFindingKind::UsedUnderscore, name, span });
        }
        matched
    }

    /// Whether this function's parameters should be checked. Parameters of externally-callable
    /// functions (entry points and `view fn`s) and `@test` functions are public surface and
    /// never warned.
    fn track_parameters(function: &Function) -> bool {
        !function.variant.is_externally_callable()
            && !function.annotations.iter().any(|a| a.identifier.name == sym::test)
    }

    /// Track a path's contribution to import usage and local references. Called from
    /// `visit_path` (expression paths) and from the explicit overrides for paths that the
    /// default walk skips (call function paths, composite-init paths, composite-type paths).
    fn note_path(&mut self, path: &Path) {
        if let Some(pid) = path.user_program() {
            self.used_imports.insert(pid.as_symbol());
        }
        if path.is_local() {
            self.note_local_use(path.identifier().name);
        }
        if let Some(loc) = path.try_global_location() {
            self.used_globals.insert(loc.clone());
        }
        if !path.is_resolved() {
            // An unresolved path is an intrinsic or an already-errored name; treat the bare
            // identifier as a potential local reference to stay robust.
            self.note_local_use(path.identifier().name);
        }
    }

    /// Walk the LHS of an assignment for read-effects only: the assignment root is a write,
    /// not a use, while indices and navigation into the target are reads. Mirrors
    /// `cei_analysis::visit_assign_lhs_reads`, so a write-only-never-read local is flagged
    /// unused (as `rustc` does for `let mut x = 0; x = 5;`).
    fn walk_assign_place(&mut self, expr: &Expression) {
        match expr {
            // Root path is a write target, not a use.
            Expression::Path(_) => {}
            // Outermost field is the write target; recurse into the inner navigation.
            Expression::MemberAccess(access) => self.walk_assign_place(&access.inner),
            Expression::TupleAccess(access) => self.walk_assign_place(&access.tuple),
            Expression::ArrayAccess(access) => {
                self.walk_assign_place(&access.array);
                self.visit_expression(&access.index, &Default::default());
            }
            // Any other shape on the LHS shouldn't reach this stage (type checker rejects),
            // but fall back to a normal read just in case.
            other => self.visit_expression(other, &Default::default()),
        }
    }

    /// Record the composite-dependency edges (`loc → composites referenced in members`)
    /// for `loc`'s composite. Records / library top-level structs additionally seed the
    /// reachability roots via `mark_root`.
    fn record_composite_member_deps(&mut self, loc: Location, composite: &Composite) {
        let mut refs = IndexSet::new();
        for member in &composite.members {
            collect_type_composite_refs(&member.type_, &mut refs);
        }
        self.composite_deps.insert(loc, refs);
    }

    /// Mark the consts referenced inside a type — array lengths and composite const arguments —
    /// as used. Used for non-record struct members, where visiting the member composites directly
    /// would keep transitively-dead structs alive but the const references must still be counted.
    fn note_type_const_refs(&mut self, ty: &Type) {
        match ty {
            Type::Array(a) => {
                self.visit_expression(&a.length, &Default::default());
                self.note_type_const_refs(a.element_type());
            }
            Type::Optional(OptionalType { inner }) => self.note_type_const_refs(inner),
            Type::Tuple(t) => {
                for elem in t.elements() {
                    self.note_type_const_refs(elem);
                }
            }
            Type::Vector(v) => self.note_type_const_refs(v.element_type()),
            Type::Mapping(m) => {
                self.note_type_const_refs(&m.key);
                self.note_type_const_refs(&m.value);
            }
            Type::Future(f) => {
                for inp in &f.inputs {
                    self.note_type_const_refs(inp);
                }
            }
            Type::Composite(c) => {
                for expr in &c.const_arguments {
                    self.visit_expression(expr, &Default::default());
                }
            }
            _ => {}
        }
    }
}

impl AstVisitor for ReferenceCollector<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_path(&mut self, input: &Path, _additional: &Self::AdditionalInput) -> Self::Output {
        self.note_path(input);
    }

    fn visit_block(&mut self, input: &Block) {
        // Blocks are the scope boundary for `let` / local `const` / iter-var shadowing, so a
        // later same-name reference cannot retroactively silence an earlier unused binding.
        self.enter_scope();
        for stmt in &input.statements {
            self.visit_statement(stmt);
        }
        self.exit_scope();
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) {
        // Visit the RHS first so a `let x = x + 1` style shadow resolves the RHS `x` to the
        // outer binding rather than to itself.
        if let Some(ty) = input.type_.as_ref() {
            self.visit_type(ty);
        }
        self.visit_expression(&input.value, &Default::default());
        match &input.place {
            DefinitionPlace::Single(id) => {
                self.declare(id.name, id.span, BindingKind::Variable);
            }
            DefinitionPlace::Multiple(ids) => {
                for id in ids {
                    self.declare(id.name, id.span, BindingKind::Variable);
                }
            }
        }
    }

    fn visit_const(&mut self, input: &ConstDeclaration) {
        self.visit_type(&input.type_);
        self.visit_expression(&input.value, &Default::default());
        // Only track local `const`s as bindings; top-level/module-scope consts (no scope open)
        // are checked separately via `used_globals` in the checker.
        if !self.scope_starts.is_empty() {
            self.declare(input.place.name, input.place.span, BindingKind::Const);
        }
    }

    fn visit_iteration(&mut self, input: &IterationStatement) {
        if let Some(ty) = input.type_.as_ref() {
            self.visit_type(ty);
        }
        self.visit_expression(&input.start, &Default::default());
        self.visit_expression(&input.stop, &Default::default());
        // Iter var is in scope only during the loop body.
        self.enter_scope();
        self.declare(input.variable.name, input.variable.span, BindingKind::Variable);
        self.visit_block(&input.block);
        self.exit_scope();
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        self.walk_assign_place(&input.place);
        self.visit_expression(&input.value, &Default::default());
    }

    fn visit_call(&mut self, input: &CallExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        self.note_path(&input.function);
        for expr in &input.const_arguments {
            self.visit_expression(expr, &Default::default());
        }
        for expr in &input.arguments {
            self.visit_expression(expr, &Default::default());
        }
    }

    fn visit_composite_init(
        &mut self,
        input: &CompositeExpression,
        _additional: &Self::AdditionalInput,
    ) -> Self::Output {
        self.note_path(&input.path);
        for expr in &input.const_arguments {
            self.visit_expression(expr, &Default::default());
        }
        for member in &input.members {
            // `PathResolution` has desugared every resolvable shorthand `Foo { a }` into
            // `Foo { a: <resolved path>}`, routing through `note_path` like an explicit `a: a`.
            // Entries still `None` failed to resolve; the type checker reports those.
            if let Some(expression) = &member.expression {
                self.visit_expression(expression, &Default::default());
            }
        }
    }

    fn visit_composite_type(&mut self, input: &CompositeType) {
        self.note_path(&input.path);
        for expr in &input.const_arguments {
            self.visit_expression(expr, &Default::default());
        }
    }
}

impl UnitVisitor for ReferenceCollector<'_> {
    fn visit_program(&mut self, input: &Program) {
        for scope in input.program_scopes.values() {
            self.visit_program_scope(scope);
        }
        for module in input.modules.values() {
            self.visit_module(module);
        }
        // `stubs` (imported programs) are not user code; skip.
    }

    fn visit_library(&mut self, input: &Library) {
        // A library is visited once as the root of its walk; programs never visit here, so
        // setting the flag without save/restore is safe.
        self.is_library = true;
        self.in_unit_scope(input.name, Vec::new(), |this| {
            for (_, c) in &input.consts {
                this.visit_const(c);
            }
            // Top-level library structs are public surface — always live roots, regardless of
            // whether anything in this compilation unit references them.
            for (name, composite) in &input.structs {
                this.composite_roots.push(Location::new(this.current_unit, vec![*name]));
                this.visit_composite(composite);
            }
            for (_, f) in &input.functions {
                this.visit_function(f);
            }
            for (_, i) in &input.interfaces {
                this.visit_interface(i);
            }
            for module in input.modules.values() {
                this.visit_module(module);
            }
            // `stubs` are not user code; skip.
        });
    }

    fn visit_program_scope(&mut self, input: &ProgramScope) {
        self.in_unit_scope(input.program_id.as_symbol(), Vec::new(), |this| {
            for (_, c) in &input.consts {
                this.visit_const(c);
            }
            for (_, c) in &input.composites {
                this.visit_composite(c);
            }
            for (_, i) in &input.interfaces {
                this.visit_interface(i);
            }
            for (_, m) in &input.mappings {
                this.visit_mapping(m);
            }
            for (_, s) in &input.storage_variables {
                this.visit_storage_variable(s);
            }
            for (_, f) in &input.functions {
                this.visit_function(f);
            }
            if let Some(c) = input.constructor.as_ref() {
                this.visit_constructor(c);
            }
        });
    }

    fn visit_module(&mut self, input: &Module) {
        self.in_unit_scope(input.unit_name, input.path.clone(), |this| {
            for (_, c) in &input.consts {
                this.visit_const(c);
            }
            for (_, c) in &input.composites {
                this.visit_composite(c);
            }
            for (_, i) in &input.interfaces {
                this.visit_interface(i);
            }
            for (_, f) in &input.functions {
                this.visit_function(f);
            }
        });
    }

    fn visit_composite(&mut self, input: &Composite) {
        let loc = self.current_location(input.identifier.name);
        for cp in &input.const_parameters {
            self.visit_type(&cp.type_);
        }
        self.record_composite_member_deps(loc.clone(), input);
        // Records are always-live program surface, so walk their member types to mark referenced
        // composites used. Struct members are skipped: visiting them would keep a transitively-
        // dead struct's inner composites alive.
        if input.is_record {
            self.composite_roots.push(loc);
            for member in &input.members {
                self.visit_type(&member.type_);
            }
        } else {
            // For structs, the composite edges above already track member composites (kept alive
            // only transitively). But const references embedded in member types — array lengths
            // and composite const arguments — must still be marked used, or a const used solely
            // in a struct field type would be falsely reported unused.
            for member in &input.members {
                self.note_type_const_refs(&member.type_);
            }
        }
    }

    fn visit_function(&mut self, input: &Function) {
        // Suppress body-level warnings exactly when the checker will report the function itself
        // as unused, to avoid double-reporting. This mirrors the checker's conditions; libraries
        // never report their functions, so `in_library` short-circuits.
        let is_dead_program_fn = !self.is_library
            && !input.variant.is_externally_callable()
            && !input.annotations.iter().any(|a| a.identifier.name == sym::test)
            && !name_starts_with_underscore(input.identifier.name)
            && self.state.call_count.get(&self.current_location(input.identifier.name)).copied() == Some(0);
        let prev_suppress = std::mem::replace(&mut self.suppress_body_warnings, is_dead_program_fn);

        // Function-level scope holds the params (in scope for the whole body). The body's
        // own block then opens a nested scope. On exit, both scopes drain and emit warnings.
        self.enter_scope();
        if Self::track_parameters(input) {
            for cp in &input.const_parameters {
                self.declare(cp.identifier.name, cp.identifier.span, BindingKind::Variable);
            }
            for inp in &input.input {
                self.declare(inp.identifier.name, inp.identifier.span, BindingKind::Variable);
            }
        }
        for cp in &input.const_parameters {
            self.visit_type(&cp.type_);
        }
        for inp in &input.input {
            self.visit_type(&inp.type_);
        }
        for out in &input.output {
            self.visit_type(&out.type_);
        }
        self.visit_type(&input.output_type);
        self.visit_block(&input.block);
        self.exit_scope();

        self.suppress_body_warnings = prev_suppress;
    }

    // `visit_constructor` and `visit_interface` use the default `UnitVisitor` impls:
    // the constructor's body block already opens its own scope via `visit_block`, and the
    // interface walk only visits types.

    fn visit_stub(&mut self, _input: &Stub) {
        // Imported programs/libraries are not the user's code; skip.
    }

    fn visit_function_stub(&mut self, _input: &leo_ast::FunctionStub) {}

    fn visit_composite_stub(&mut self, _input: &Composite) {}
}

/// Collect the composite `Location`s reachable from `ty` through type wrappers
/// (Array, Optional, Tuple, Vector, Mapping, Future). Mirrors
/// `type_checking::add_composite_dependencies`.
fn collect_type_composite_refs(ty: &Type, refs: &mut IndexSet<Location>) {
    match ty {
        Type::Composite(c) => {
            if let Some(loc) = c.path.try_global_location() {
                refs.insert(loc.clone());
            }
        }
        Type::Array(a) => collect_type_composite_refs(a.element_type(), refs),
        Type::Optional(OptionalType { inner }) => collect_type_composite_refs(inner, refs),
        Type::Tuple(t) => {
            for elem in t.elements() {
                collect_type_composite_refs(elem, refs);
            }
        }
        Type::Vector(v) => collect_type_composite_refs(v.element_type(), refs),
        Type::Mapping(m) => {
            collect_type_composite_refs(&m.key, refs);
            collect_type_composite_refs(&m.value, refs);
        }
        Type::Future(f) => {
            for inp in &f.inputs {
                collect_type_composite_refs(inp, refs);
            }
        }
        _ => {}
    }
}
