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
use leo_ast::{Function, ProgramReconstructor, ProgramScope, StatementReconstructor};
use leo_span::Symbol;

use indexmap::IndexMap;

impl ProgramReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program name from the input.
        self.program = input.program_id.name.name;

        // Compute a post-order traversal of the call graph. This ensures that functions are processed after all their
        // callees. This unwrap is safe because the type checker guarantees an acyclic graph.
        let order = self.state.call_graph.post_order().unwrap();

        // Create a map of function names to their definitions for fast access.
        let mut function_map: IndexMap<Symbol, Function> = input.functions.into_iter().collect();

        // Reconstruct functions in post-order.
        for function_name in &order {
            // Skip external functions (i.e., not in the input map).
            if let Some(function) = function_map.shift_remove(function_name) {
                // Perform monomorphization or other reconstruction logic.
                let reconstructed_function = self.reconstruct_function(function);
                // Store the reconstructed function for inclusion in the output scope.
                self.reconstructed_functions.push((*function_name, reconstructed_function));
            }
        }

        // Identify functions that are monomorphized and no longer referenced by any calls.
        let functions_to_remove: Vec<_> = self
            .reconstructed_functions
            .iter()
            .filter_map(|(f, _)| {
                let is_monomorphized = self.monomorphized_functions.contains(f);

                let is_still_called = self.unresolved_calls.iter().any(|c| match &c.function {
                    leo_ast::Expression::Identifier(ident) => ident.name == *f,
                    _ => panic!("Parser guarantees `function` is always an identifier."),
                });

                // Mark for removal if the function is monomorphized and no unresolved calls reference it.
                if is_monomorphized && !is_still_called { Some(*f) } else { None }
            })
            .collect();

        // Remove functions that are no longer needed.
        self.reconstructed_functions.retain(|(f, _)| !functions_to_remove.contains(f));

        // Also remove these functions from the call graph.
        for f in functions_to_remove {
            self.state.call_graph.remove_node(&f);
        }

        // Move reconstructed functions into the final `ProgramScope`, clearing the temporary storage for the next scope.
        let functions = core::mem::take(&mut self.reconstructed_functions).into_iter().collect();

        // Return the fully reconstructed scope with updated functions.
        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            functions,
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
}
