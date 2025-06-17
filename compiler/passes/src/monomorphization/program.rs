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
use leo_ast::{Constructor, Function, Location, ProgramReconstructor, ProgramScope, StatementReconstructor};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;

impl ProgramReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program name from the input.
        self.program = input.program_id.name.name;

        // Create a map of function names to their definitions for fast access.
        let mut function_map: IndexMap<Symbol, Function> = input.functions.into_iter().collect();

        println!("Call graph: {:#?}", self.state.call_graph.nodes().collect::<Vec<_>>());

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
            let reconstructed_function = self.reconstruct_function(function);
            // Store the reconstructed function for inclusion in the output scope.
            self.reconstructed_functions.insert(location.name, reconstructed_function);
        }

        // Reconstruct the constructor last, as it cannot be called by any other function.
        let constructor = input.constructor.map(|c| self.reconstruct_constructor(c));

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
        let functions = core::mem::take(&mut self.reconstructed_functions).into_iter().collect::<Vec<_>>();

        // Return the fully reconstructed scope with updated functions.
        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            functions,
            constructor,
            consts: input.consts,
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        // Keep track of the current function name
        self.function = input.identifier.name;

        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input.const_parameters,
            input: input.input,
            output: input.output,
            output_type: input.output_type,
            block: self.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        // Use the symbol for the constructor as the current function name.
        self.function = sym::constructor;

        Constructor { block: self.reconstruct_block(input.block).0, span: input.span, id: input.id }
    }
}
