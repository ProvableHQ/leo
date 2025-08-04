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
use leo_ast::{AstReconstructor, Composite, Function, ProgramReconstructor, ProgramScope, Statement, Variant};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;

impl ProgramReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program name from the input.
        self.program = input.program_id.name.name;

        // We first reconstruct all structs. Struct fields can instantiate other generic structs that we need to handle
        // first. We'll then address struct expressions and other struct type instantiations.
        let mut struct_map: IndexMap<Symbol, Composite> = input.structs.clone().into_iter().collect();
        let struct_order = self.state.struct_graph.post_order().unwrap();

        // Reconstruct structs in post-order.
        for struct_name in &struct_order {
            if let Some(r#struct) = struct_map.swap_remove(struct_name) {
                // Perform monomorphization or other reconstruction logic.
                let reconstructed_struct = self.reconstruct_struct(r#struct);
                // Store the reconstructed struct for inclusion in the output scope.
                self.reconstructed_structs.insert(*struct_name, reconstructed_struct);
            }
        }

        // If there are some structs left in `struct_map`, that means these are dead structs since they do not show up
        // in `struct_graph`. Therefore, they won't be reconstructed, implying a change in the reconstructed program.
        if !struct_map.is_empty() {
            self.changed = true;
        }

        // Next, handle generic functions
        //
        // Create a map of function names to their definitions for fast access.
        let mut function_map: IndexMap<Symbol, Function> = input.functions.into_iter().collect();

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
                if location.program == self.program && location.name == sym::constructor {
                    return true;
                }
                function_map
                    .get(&location.name)
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

        // Determine any ca

        // Reconstruct functions in post-order.
        for location in &order {
            if let Some(function) = function_map.swap_remove(&location.name) {
                // Perform monomorphization or other reconstruction logic.
                let reconstructed_function = self.reconstruct_function(function);
                // Store the reconstructed function for inclusion in the output scope.
                self.reconstructed_functions.insert(location.name, reconstructed_function);
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
            let is_still_called = self.unresolved_calls.iter().any(|c| c.function.name == *f);
            !is_monomorphized || is_still_called
        });

        // Move reconstructed structs into the final `ProgramScope`, clearing the temporary storage for the next scope.
        let structs = core::mem::take(&mut self.reconstructed_structs).into_iter().collect::<Vec<_>>();

        // Move reconstructed functions into the final `ProgramScope`, clearing the temporary storage for the next scope.
        // Make sure to place transitions before all the other functions.
        let (transitions, mut non_transitions): (Vec<_>, Vec<_>) = core::mem::take(&mut self.reconstructed_functions)
            .into_iter()
            .partition(|(_, f)| f.variant.is_transition());

        let mut all_functions = transitions;
        all_functions.append(&mut non_transitions);

        // Return the fully reconstructed scope with updated functions.
        ProgramScope {
            program_id: input.program_id,
            structs,
            mappings,
            functions: all_functions,
            constructor,
            consts,
            span: input.span,
        }
    }
}
