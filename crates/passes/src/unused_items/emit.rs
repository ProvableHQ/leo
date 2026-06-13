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

//! Phase 2 of the unused-items pass: emits every warning. First the unused-local
//! warnings recorded by phase 1, then — by walking the AST top-down without
//! descending into function bodies — the unused top-level items and imports.

use super::{CollectedUses, LocalFindingKind, name_starts_with_underscore};

use leo_ast::*;
use leo_span::{Symbol, sym};

pub(super) struct TopLevelItemChecker<'a> {
    /// Mutable compiler state; this is the sole emitter of warnings.
    state: &'a mut crate::CompilerState,
    /// Everything phase 1 collected, consumed here.
    data: CollectedUses,

    // --- Traversal state: a cursor over the top-level walk. ---
    /// Current compilation unit while walking. Pushed/popped by `visit_program_scope`,
    /// `visit_module`, and `visit_library`.
    current_unit: Symbol,
    /// Current module path prefix (empty at program/library top level).
    current_module: Vec<Symbol>,
}

impl<'a> TopLevelItemChecker<'a> {
    pub(super) fn new(state: &'a mut crate::CompilerState, data: CollectedUses) -> Self {
        Self { state, data, current_unit: Symbol::intern(""), current_module: Vec::new() }
    }

    /// Turn the unused-local findings from phase 1 into diagnostics and emit them. Called before
    /// the top-level walk so locals are reported ahead of items, matching the original order.
    pub(super) fn emit_local_warnings(&mut self) {
        use crate::errors::unused_items::*;
        for finding in self.data.local_findings.drain(..) {
            let warning = match finding.kind {
                LocalFindingKind::UnusedVariable => unused_variable(finding.name, finding.span),
                LocalFindingKind::UnusedConst => unused_const(finding.name, finding.span),
                LocalFindingKind::UsedUnderscore => used_underscore_binding(finding.name, finding.span),
            };
            self.state.handler.emit_warning(warning);
        }
    }

    fn current_location(&self, name: Symbol) -> Location {
        Location::new(self.current_unit, self.current_module.iter().copied().chain(std::iter::once(name)).collect())
    }
}

impl AstVisitor for TopLevelItemChecker<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_const(&mut self, input: &ConstDeclaration) {
        // The checker never descends into bodies, so every const reached here is top-level.
        if name_starts_with_underscore(input.place.name) {
            return;
        }
        let location = self.current_location(input.place.name);
        if self.data.used_globals.contains(&location) {
            return;
        }
        self.state.handler.emit_warning(crate::errors::unused_items::unused_const(input.place.name, input.place.span));
    }
}

impl UnitVisitor for TopLevelItemChecker<'_> {
    fn visit_program(&mut self, input: &Program) {
        for scope in input.program_scopes.values() {
            self.visit_program_scope(scope);
        }
        for module in input.modules.values() {
            self.visit_module(module);
        }
        for (import_name, program_id) in &input.imports {
            if self.data.used_imports.contains(import_name) {
                continue;
            }
            self.state.handler.emit_warning(crate::errors::unused_items::unused_import(import_name, program_id.span()));
        }
    }

    fn visit_library(&mut self, _input: &Library) {
        // Every top-level library item is public surface reachable cross-unit, so none are
        // flagged here. Unused locals in their bodies are still reported by the collect phase.
    }

    fn visit_program_scope(&mut self, input: &ProgramScope) {
        let prev_unit = std::mem::replace(&mut self.current_unit, input.program_id.as_symbol());
        let prev_prefix = std::mem::take(&mut self.current_module);
        // Iterate functions → composites → consts so warnings group by item kind in the
        // original output order.
        for (_, f) in &input.functions {
            self.visit_function(f);
        }
        for (_, c) in &input.composites {
            self.visit_composite(c);
        }
        for (_, c) in &input.consts {
            self.visit_const(c);
        }
        self.current_unit = prev_unit;
        self.current_module = prev_prefix;
    }

    fn visit_module(&mut self, input: &Module) {
        let prev_unit = std::mem::replace(&mut self.current_unit, input.unit_name);
        let prev_prefix = std::mem::replace(&mut self.current_module, input.path.clone());
        for (_, f) in &input.functions {
            self.visit_function(f);
        }
        for (_, c) in &input.composites {
            self.visit_composite(c);
        }
        for (_, c) in &input.consts {
            self.visit_const(c);
        }
        self.current_unit = prev_unit;
        self.current_module = prev_prefix;
    }

    fn visit_function(&mut self, input: &Function) {
        // Always live: externally-callable functions (entry points, `view fn`s) and `@test`
        // functions, whose callers live in other compilation units.
        if input.variant.is_externally_callable() {
            return;
        }
        if input.annotations.iter().any(|a| a.identifier.name == sym::test) {
            return;
        }
        // A leading `_` signals intentionally-unused. Safe here: such functions are always
        // inlined and never reach the VM as a named identifier.
        if name_starts_with_underscore(input.identifier.name) {
            return;
        }
        let location = self.current_location(input.identifier.name);
        let call_count =
            *self.state.call_count.get(&location).expect("call_count is populated for every function by TypeChecking");
        if call_count > 0 {
            return;
        }
        self.state
            .handler
            .emit_warning(crate::errors::unused_items::unused_function(input.identifier.name, input.identifier.span));
    }

    fn visit_composite(&mut self, input: &Composite) {
        // Records are public surface constrained by interface conformance; never warn on them.
        if input.is_record {
            return;
        }
        let location = self.current_location(input.identifier.name);
        if !self.data.live_composites.contains(&location) {
            self.state
                .handler
                .emit_warning(crate::errors::unused_items::unused_struct(input.identifier.name, input.identifier.span));
        }
        // Dead struct fields are intentionally not warned: see the module docstring.
    }

    // The checker only warns on top-level items, so suppress descents into everything else
    // the default impls would recurse into (prototype types, mapping types, etc.).
    fn visit_constructor(&mut self, _input: &Constructor) {}

    fn visit_interface(&mut self, _input: &Interface) {}

    fn visit_mapping(&mut self, _input: &Mapping) {}

    fn visit_storage_variable(&mut self, _input: &StorageVariable) {}

    fn visit_stub(&mut self, _input: &Stub) {}

    fn visit_function_stub(&mut self, _input: &leo_ast::FunctionStub) {}

    fn visit_composite_stub(&mut self, _input: &Composite) {}
}
