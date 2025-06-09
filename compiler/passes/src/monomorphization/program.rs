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
use leo_ast::{Function, ProgramReconstructor, ProgramScope, StatementReconstructor, TypeReconstructor, Variant};
use leo_span::Symbol;

use indexmap::IndexMap;

impl TypeReconstructor for MonomorphizationVisitor<'_> {}

impl ProgramReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program name from the input.
        self.program = input.program_id.name.name;

        // Create a map of function names to their definitions for fast access.
        let mut function_map: IndexMap<Symbol, Function> = input.functions.clone().into_iter().collect();

        // Compute a post-order traversal of the call graph. This ensures that functions are processed after all their
        // callees. Make sure to only to compute the post order by considering the entry points of the program, which
        // are `async transition`, `transition` and `function`.
        let order = self
            .state
            .call_graph
            .post_order_from_entry_points(|node| {
                function_map
                    .get(node)
                    .map(|f| matches!(f.variant, Variant::AsyncTransition | Variant::Transition | Variant::Function))
                    .unwrap_or(false)
            })
            .unwrap(); // This unwrap is safe because the type checker guarantees an acyclic graph.

        // Reconstruct functions in post-order.
        for function_name in &order {
            // Skip external functions (i.e., not in the input map).
            if let Some(function) = function_map.swap_remove(function_name) {
                // Perform monomorphization or other reconstruction logic.
                let reconstructed_function = self.reconstruct_function(function);
                // Store the reconstructed function for inclusion in the output scope.
                self.reconstructed_functions.insert(*function_name, (reconstructed_function, *function_name));
            }
        }

        // Retain only functions that are either not yet monomorphized or are still referenced by calls.
        self.reconstructed_functions.retain(|f, _| {
            let is_monomorphized = self.monomorphized_functions.contains(f);

            let is_still_called = self.unresolved_calls.iter().any(|c| match &c.function {
                leo_ast::Expression::Identifier(ident) => ident.name == *f,
                _ => panic!("Parser guarantees `function` is always an identifier."),
            });

            if is_monomorphized && !is_still_called {
                // It's monomorphized and there are no unresolved calls to it - remove it.
                self.state.call_graph.remove_node(f);
                false
            } else {
                true
            }
        });

        // Move reconstructed functions into the final `ProgramScope`, clearing the temporary storage for the next scope.
        /*let functions = core::mem::take(&mut self.reconstructed_functions)
        .into_iter()
        .map(|(s, (f, _))| (s, f))
        .collect::<Vec<_>>();*/

        // Step 1: Group reconstructed functions by `old_name`
        let mut grouped: std::collections::HashMap<Symbol, Vec<(Symbol, Function)>> = std::collections::HashMap::new();

        for (new_name, (function, old_name)) in &self.reconstructed_functions {
            grouped.entry(*old_name).or_default().push((*new_name, function.clone()));
        }

        // Step 2: Emit all reconstructed functions in the original order
        let mut ordered_reconstructed = Vec::new();

        for (original_name, _) in &input.functions {
            if let Some(entries) = grouped.remove(original_name) {
                ordered_reconstructed.extend(entries);
            }
        }

        self.reconstructed_functions.clear();

        // Return the fully reconstructed scope with updated functions.
        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            functions: ordered_reconstructed,
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
