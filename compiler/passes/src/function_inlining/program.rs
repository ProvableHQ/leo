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

use super::FunctionInliningVisitor;
use leo_ast::{AstReconstructor, Constructor, Function, ProgramReconstructor, ProgramScope};
use leo_span::Symbol;

use indexmap::IndexMap;
use snarkvm::prelude::Itertools;

impl ProgramReconstructor for FunctionInliningVisitor<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the program name.
        self.program = input.program_id.name.name;

        // Get the post-order ordering of the call graph.
        // Note that the post-order always contains all nodes in the call graph.
        // Note that the unwrap is safe since type checking guarantees that the call graph is acyclic.
        let order = self
            .state
            .call_graph
            .post_order()
            .unwrap()
            .into_iter()
            .filter_map(|location| (location.program == self.program).then_some(location.name))
            .collect_vec();

        // Construct map to provide faster lookup of functions.
        let mut function_map: IndexMap<Symbol, Function> = input.functions.into_iter().collect();

        // Reconstruct and accumulate each of the functions in post-order.
        for function_name in &order {
            // None: If `function_name` is not in `input.functions`, then it must be an external function.
            // TODO: Check that this is indeed an external function. Requires a redesign of the symbol table.
            if let Some(function) = function_map.shift_remove(function_name) {
                // Reconstruct the function.
                let reconstructed_function = self.reconstruct_function(function);
                // Add the reconstructed function to the mapping.
                self.reconstructed_functions.push((*function_name, reconstructed_function));
            }
        }
        // This is a sanity check to ensure that functions in the program scope have been processed.
        assert!(function_map.is_empty(), "All functions in the program scope should have been processed.");

        // Reconstruct the constructor.
        // Note: This must be done after the functions have been reconstructed to ensure that every callee function has been inlined.
        let constructor = input.constructor.map(|constructor| self.reconstruct_constructor(constructor));

        // Note that this intentionally clears `self.reconstructed_functions` for the next program scope.
        let functions = core::mem::take(&mut self.reconstructed_functions).into_iter().collect();

        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            constructor,
            functions,
            consts: input.consts,
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input.const_parameters,
            input: input.input,
            output: input.output,
            output_type: input.output_type,
            block: {
                // Set the `is_async` flag before reconstructing the block.
                self.is_async = input.variant.is_async_function();
                // Reconstruct the block.
                let block = self.reconstruct_block(input.block).0;
                // Reset the `is_async` flag.
                self.is_async = false;
                block
            },
            span: input.span,
            id: input.id,
        }
    }

    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        Constructor {
            annotations: input.annotations,
            block: {
                // Set the `is_async` flag before reconstructing the block.
                self.is_async = true;
                // Reconstruct the block.
                let block = self.reconstruct_block(input.block).0;
                // Reset the `is_async` flag.
                self.is_async = false;
                block
            },
            span: input.span,
            id: input.id,
        }
    }
}
