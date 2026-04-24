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

use super::MonomorphizationVisitor;
use crate::common::{items_at_path, program_composites, program_functions, stub_composites, stub_functions};
use leo_ast::{AstReconstructor, Library, Program, ProgramScope, Statement, Stub, UnitReconstructor, Variant};
use leo_span::sym;

impl UnitReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_library(&mut self, input: Library) -> Library {
        // Unreached library functions are dropped: code generation skips library stubs
        // entirely, and nothing downstream reads un-inlined library bodies. Only functions
        // the DFS actually monomorphized (i.e. the ones callers reach) make it through.
        Library {
            name: input.name,
            modules: input.modules.into_iter().map(|(id, m)| (id, self.assemble_module(m))).collect(),
            structs: items_at_path(&self.reconstructed_composites, input.name, &[]).collect(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            functions: items_at_path(&self.reconstructed_functions, input.name, &[]).collect(),
            interfaces: input.interfaces,
            stubs: input.stubs,
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        let top_level_program = input.program_id.as_symbol();
        self.program = top_level_program;

        // Composites first: a composite field may instantiate another generic composite, so
        // post-order makes sure dependencies are monomorphized before their users.
        let composite_order = self.state.composite_graph.post_order().unwrap();

        for loc in &composite_order {
            if let Some(composite) = self.composite_map.swap_remove(loc) {
                let reconstructed_composite = self.reconstruct_composite(composite);
                self.reconstructed_composites.insert(loc.clone(), reconstructed_composite);
            }
        }

        // Any current-program composite left in `composite_map` is dead code; note that and
        // drop it so downstream passes re-run. External leftovers are fine — they just aren't
        // referenced by the composite graph and will flow through unchanged into stubs.
        if self.composite_map.keys().any(|loc| loc.program == self.program) {
            self.changed = true;
        }

        // DFS the call graph from every program's entry points, constructors, and non-generic
        // fns. External roots are included so that generic closures called only from within an
        // imported program (e.g. child's `main` calling child's own `foo::[42u32]()`) still get
        // monomorphized during the current compilation; otherwise their call sites would escape
        // unrewritten and later passes would see dangling const-parameter references.
        // `post_order_with_filter` only limits roots, so the outer filter is still needed to
        // drop transitive non-candidates. The unwrap is safe — type checking proves acyclicity.
        let order = self
            .state
            .call_graph
            .post_order_with_filter(|location| {
                if location.path.as_slice() == [sym::constructor] {
                    return true;
                }
                self.function_map
                    .get(location)
                    .map(|f| {
                        matches!(f.variant, Variant::EntryPoint)
                            || (f.variant == Variant::Fn && f.const_parameters.is_empty())
                    })
                    .unwrap_or(false)
            })
            .unwrap()
            .into_iter()
            .filter(|location| self.function_map.contains_key(location))
            .collect::<Vec<_>>();

        for function_name in &order {
            // `self.program` is set per-function so `reconstruct_call` can see cross-program
            // calls from the callee's perspective.
            self.program = function_name.program;
            if let Some(function) = self.function_map.swap_remove(function_name) {
                let reconstructed_function = self.reconstruct_function(function);
                self.reconstructed_functions.insert(function_name.clone(), reconstructed_function);
            }
        }

        self.program = top_level_program;

        // External items not reached by the DFS are carried through unchanged so stubs can
        // pick them up; current-program leftovers are dead code and intentionally dropped.
        for (loc, f) in std::mem::take(&mut self.function_map) {
            if loc.program != self.program {
                self.reconstructed_functions.entry(loc).or_insert(f);
            }
        }
        for (loc, c) in std::mem::take(&mut self.composite_map) {
            if loc.program != self.program {
                self.reconstructed_composites.entry(loc).or_insert(c);
            }
        }

        let mappings =
            input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect();
        let storage_variables = input
            .storage_variables
            .into_iter()
            .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
            .collect();

        let consts = input
            .consts
            .into_iter()
            .map(|(i, c)| match self.reconstruct_const(c) {
                (Statement::Const(declaration), _) => (i, declaration),
                _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
            })
            .collect();

        // The constructor is reconstructed last because nothing can call it.
        let constructor = input.constructor.map(|c| self.reconstruct_constructor(c));

        // Drop original generic functions whose monomorphized instances have replaced them,
        // unless they are still referenced by unresolved calls that later passes will retry.
        self.reconstructed_functions.retain(|l, _| {
            let is_monomorphized = self.monomorphized_functions.contains(l);
            let is_still_called = self.unresolved_calls.iter().any(|c| c.function.expect_global_location() == l);
            !is_monomorphized || is_still_called
        });

        // Collect only current-program top-level functions for this scope, then reorder so
        // entry points precede finalize functions — the type checker expects that order.
        let (entry_points, non_entry_points): (Vec<_>, Vec<_>) =
            items_at_path(&self.reconstructed_functions, self.program, &[]).partition(|(_, f)| f.variant.is_entry());
        let functions: Vec<_> = entry_points.into_iter().chain(non_entry_points).collect();

        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.into_iter().map(|(s, t)| (s, self.reconstruct_type(t).0)).collect(),
            // Exclude generic composites that have been monomorphized — only their concrete
            // specializations should appear in the output.
            composites: items_at_path(&self.reconstructed_composites, self.program, &[])
                .filter(|(_, c)| c.const_parameters.is_empty())
                .collect(),
            mappings,
            storage_variables,
            functions,
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor,
            consts,
            span: input.span,
        }
    }

    fn reconstruct_program(&mut self, input: Program) -> Program {
        // Seed `function_map` and `composite_map` with every definition reachable from this
        // program (stubs, libraries, current program). A single DFS from the current program's
        // entry points then monomorphizes all of them in one pass; cross-program edges in the
        // call graph make recursive per-stub passes unnecessary. Current-program inserts come
        // last so they override any stub placeholders for overlapping keys.
        self.program =
            *input.program_scopes.first().expect("a program must have a single program scope at this stage").0;

        for (_, stub) in &input.stubs {
            for (loc, f) in stub_functions(stub) {
                self.function_map.entry(loc).or_insert_with(|| f.clone());
            }
            for (loc, c) in stub_composites(stub) {
                self.composite_map.entry(loc).or_insert_with(|| c.clone());
            }
        }
        for (loc, f) in program_functions(&input) {
            self.function_map.insert(loc, f.clone());
        }
        for (loc, c) in program_composites(&input) {
            self.composite_map.insert(loc, c.clone());
        }

        // Type checking depends on stubs coming out in the original insertion order, so
        // snapshot the keys before partitioning.
        let stub_key_order: Vec<_> = input.stubs.keys().cloned().collect();
        let (from_leo_stubs, other_stubs): (Vec<_>, Vec<_>) =
            input.stubs.into_iter().partition(|(_, stub)| matches!(stub, Stub::FromLeo { .. }));

        let program_scopes =
            input.program_scopes.into_iter().map(|(id, scope)| (id, self.reconstruct_program_scope(scope))).collect();

        // `FromLeo` stubs are reassembled from the already-populated `reconstructed_*` maps.
        let from_leo_stubs: indexmap::IndexMap<_, _> = from_leo_stubs
            .into_iter()
            .map(|(id, stub)| {
                let stub = match stub {
                    Stub::FromLeo { program, parents } => {
                        Stub::FromLeo { program: self.assemble_from_leo_program(program), parents }
                    }
                    other => other,
                };
                (id, stub)
            })
            .collect();

        // Library/Aleo stubs run after program scopes because `reconstruct_library` pulls
        // monomorphized composites out of `reconstructed_composites`.
        let other_stubs: indexmap::IndexMap<_, _> =
            other_stubs.into_iter().map(|(id, stub)| (id, self.reconstruct_stub(stub))).collect();

        let mut from_leo_map = from_leo_stubs;
        let mut other_map = other_stubs;
        let stubs = stub_key_order
            .into_iter()
            .map(|id| {
                let stub = from_leo_map
                    .swap_remove(&id)
                    .or_else(|| other_map.swap_remove(&id))
                    .expect("every stub key must appear in exactly one partition");
                (id, stub)
            })
            .collect();

        Program {
            stubs,
            program_scopes,
            modules: input.modules.into_iter().map(|(id, module)| (id, self.assemble_module(module))).collect(),
            ..input
        }
    }
}
