// Copyright (C) 2019-2025 Provable Inc.
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
use leo_ast::{AstReconstructor, Module, Program, ProgramReconstructor, ProgramScope, Statement, Variant};
use leo_span::sym;

impl ProgramReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program name from the input.
        self.program = input.program_id.name.name;

        // We first reconstruct all structs. Struct fields can instantiate other generic structs that we need to handle
        // first. We'll then address struct expressions and other struct type instantiations.
        let struct_order = self.state.struct_graph.post_order().unwrap();

        // Reconstruct structs in post-order.
        for struct_name in &struct_order {
            if let Some(r#struct) = self.struct_map.swap_remove(struct_name) {
                // Perform monomorphization or other reconstruction logic.
                let reconstructed_struct = self.reconstruct_struct(r#struct);
                // Store the reconstructed struct for inclusion in the output scope.
                self.reconstructed_structs.insert(struct_name.clone(), reconstructed_struct);
            }
        }

        // If there are some structs left in `struct_map`, that means these are dead structs since they do not show up
        // in `struct_graph`. Therefore, they won't be reconstructed, implying a change in the reconstructed program.
        if !self.struct_map.is_empty() {
            self.changed = true;
        }

        // Next, handle generic functions
        //
        // Compute a post-order traversal of the call graph. This ensures that functions are processed after all their callees.
        // Make sure to only compute the post order by considering the entry points of the program, which are `async transition`, `transition` and `function`.
        // We must consider entry points to ignore const generic inlines that have already been monomorphized but never called.
        let order = self
            .state
            .call_graph
            .post_order_with_filter(|location| {
                // Filter out locations that are not from this program.
                if location.program != self.program {
                    return false;
                }
                // Allow constructors.
                if location.program == self.program && location.path == vec![sym::constructor] {
                    return true;
                }
                self.function_map
                    .get(&location.path)
                    .map(|f| {
                        matches!(
                            f.variant,
                            Variant::AsyncTransition | Variant::Transition | Variant::Function | Variant::Script
                        )
                    })
                    .unwrap_or(false)
            })
            .unwrap() // This unwrap is safe because the type checker guarantees an acyclic graph.
            .into_iter()
            .filter(|location| location.program == self.program).collect::<Vec<_>>();

        for function_name in &order {
            // Reconstruct functions in post-order.
            if let Some(function) = self.function_map.swap_remove(&function_name.path) {
                // Reconstruct the function.
                let reconstructed_function = self.reconstruct_function(function.clone());
                // Add the reconstructed function to the mapping.
                self.reconstructed_functions.insert(function_name.path.clone(), reconstructed_function);
            }
        }

        // Get any

        // Now reconstruct mappings
        let mappings =
            input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect();

        // Then consts
        let consts = input
            .consts
            .into_iter()
            .map(|(i, c)| match self.reconstruct_const(c) {
                (Statement::Const(declaration), _) => (i, declaration),
                _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
            })
            .collect();

        // Reconstruct the constructor last, as it cannot be called by any other function.
        let constructor = input.constructor.map(|c| self.reconstruct_constructor(c));

        // Now retain only functions that are either not yet monomorphized or are still referenced by calls.
        self.reconstructed_functions.retain(|f, _| {
            let is_monomorphized = self.monomorphized_functions.contains(f);
            let is_still_called = self.unresolved_calls.iter().any(|c| &c.function.absolute_path() == f);
            !is_monomorphized || is_still_called
        });

        // Move reconstructed functions into the final `ProgramScope`.
        // Make sure to place transitions before all the other functions.
        let (transitions, mut non_transitions): (Vec<_>, Vec<_>) =
            self.reconstructed_functions.clone().into_iter().partition(|(_, f)| f.variant.is_transition());

        let mut all_functions = transitions;
        all_functions.append(&mut non_transitions);

        // Return the fully reconstructed scope with updated functions.
        ProgramScope {
            program_id: input.program_id,
            structs: self
                .reconstructed_structs
                .iter()
                .filter_map(|(path, c)| {
                    // only consider structs defined at program scope. The rest will be added to their parent module.
                    path.split_last().filter(|(_, rest)| rest.is_empty()).map(|(last, _)| (*last, c.clone()))
                })
                .collect(),
            mappings,
            functions: all_functions
                .iter()
                .filter_map(|(path, f)| {
                    // only consider functions defined at program scope. The rest will be added to their parent module.
                    path.split_last().filter(|(_, rest)| rest.is_empty()).map(|(last, _)| (*last, f.clone()))
                })
                .collect(),
            constructor,
            consts,
            span: input.span,
        }
    }

    fn reconstruct_program(&mut self, input: Program) -> Program {
        // Populate `self.function_map` using the functions in the program scopes and the modules
        input
            .modules
            .iter()
            .flat_map(|(module_path, m)| {
                m.functions.iter().map(move |(name, f)| {
                    (module_path.iter().cloned().chain(std::iter::once(*name)).collect(), f.clone())
                })
            })
            .chain(
                input
                    .program_scopes
                    .iter()
                    .flat_map(|(_, scope)| scope.functions.iter().map(|(name, f)| (vec![*name], f.clone()))),
            )
            .for_each(|(full_name, f)| {
                self.function_map.insert(full_name, f);
            });

        // Populate `self.struct_map` using the structs in the program scopes and the modules
        input
            .modules
            .iter()
            .flat_map(|(module_path, m)| {
                m.structs.iter().map(move |(name, f)| {
                    (module_path.iter().cloned().chain(std::iter::once(*name)).collect(), f.clone())
                })
            })
            .chain(
                input
                    .program_scopes
                    .iter()
                    .flat_map(|(_, scope)| scope.structs.iter().map(|(name, f)| (vec![*name], f.clone()))),
            )
            .for_each(|(full_name, f)| {
                self.struct_map.insert(full_name, f);
            });

        // Reconstruct prrogram scopes first then reconstruct the modules after `self.reconstructed_structs`
        // and `self.reconstructed_functions` have been populated.
        Program {
            program_scopes: input
                .program_scopes
                .into_iter()
                .map(|(id, scope)| (id, self.reconstruct_program_scope(scope)))
                .collect(),
            modules: input.modules.into_iter().map(|(id, module)| (id, self.reconstruct_module(module))).collect(),
            ..input
        }
    }

    fn reconstruct_module(&mut self, input: Module) -> Module {
        // Here we're reconstructing structs and functions from `reconstructed_functions` and
        // `reconstructed_structs` based on their paths and whether they match the module path
        Module {
            structs: self
                .reconstructed_structs
                .iter()
                .filter_map(|(path, c)| path.split_last().map(|(last, rest)| (last, rest, c)))
                .filter(|&(_, rest, _)| input.path == rest)
                .map(|(last, _, c)| (*last, c.clone()))
                .collect(),

            functions: self
                .reconstructed_functions
                .iter()
                .filter_map(|(path, f)| path.split_last().map(|(last, rest)| (last, rest, f)))
                .filter(|&(_, rest, _)| input.path == rest)
                .map(|(last, _, f)| (*last, f.clone()))
                .collect(),
            ..input
        }
    }
}
