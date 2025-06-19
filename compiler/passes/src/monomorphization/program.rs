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
use leo_ast::{Function, Location, ProgramReconstructor, ProgramScope};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;

impl ProgramReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program name from the input.
        self.program = input.program_id.name.name;

        // Create a map of function names to their definitions for fast access.
        let mut function_map: IndexMap<Symbol, Function> = input.functions.into_iter().collect();

        // Compute a post-order traversal of the call graph.
        // This ensures that functions are processed after all their callees.
        let order = self.state.call_graph.post_order().unwrap(); // This unwrap is safe because the type checker guarantees an acyclic graph.

        // Reconstruct functions in post-order.
        for location in &order {
            // Skip external functions.
            if location.program != self.program {
                continue;
            }
            // Reconstruct the function which is guaranteed to be in the map.
            let Some(function) = function_map.swap_remove(&location.name) else {
                panic!("Function {} not found in function map", location.name);
            };
            // Perform monomorphization or other reconstruction logic.
            self.function = function.identifier.name;
            let reconstructed_function = self.reconstruct_function(function);
            // Store the reconstructed function for inclusion in the output scope.
            self.reconstructed_functions.insert(location.name, reconstructed_function);
        }

        // Reconstruct the constructor last, as it cannot be called by any other function.
        let constructor = input.constructor.map(|c| {
            self.function = sym::constructor;
            self.reconstruct_constructor(c)
        });

        // Retain only functions that are either not yet monomorphized or are still referenced by calls.
        self.reconstructed_functions.retain(|f, _| {
            let is_monomorphized = self.monomorphized_functions.contains(f);

            let is_still_called = self.unresolved_calls.iter().any(|c| c.function.name == *f);

            if is_monomorphized && !is_still_called {
                // It's monomorphized and there are no unresolved calls to it - remove it.
                self.state.call_graph.remove_node(&Location::new(self.program, *f));
                false
            } else {
                true
            }
        });

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
            consts: input.consts,
            structs: input.structs,
            mappings: input.mappings,
            functions: all_functions,
            constructor,
            span: input.span,
        }
    }
}
