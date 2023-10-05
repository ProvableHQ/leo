// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::FunctionInliner;

use leo_ast::{Function, ProgramReconstructor, ProgramScope};
use leo_span::Symbol;

use indexmap::IndexMap;

impl ProgramReconstructor for FunctionInliner<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Get the post-order ordering of the call graph.
        // Note that the post-order always contains all nodes in the call graph.
        // Note that the unwrap is safe since type checking guarantees that the call graph is acyclic.
        let order = self.call_graph.post_order().unwrap();

        // Construct map to provide faster lookup of functions
        let mut function_map: IndexMap<Symbol, Function> = input.functions.into_iter().collect();

        // Reconstruct and accumulate each of the functions in post-order.
        for function_name in &order {
            // None: If `function_name` is not in `input.functions`, then it must be an external function.
            // TODO: Check that this is indeed an external function. Requires a redesign of the symbol table.
            if let Some(function) = function_map.remove(function_name) {
                // Reconstruct the function.
                let reconstructed_function = self.reconstruct_function(function);
                // Add the reconstructed function to the mapping.
                self.reconstructed_functions.push((*function_name, reconstructed_function));
            }
        }
        // This is a sanity check to ensure that functions in the program scope have been processed.
        assert!(function_map.is_empty(), "All functions in the program scope should have been processed.");

        // Note that this intentionally clears `self.reconstructed_functions` for the next program scope.
        let functions = core::mem::take(&mut self.reconstructed_functions).into_iter().collect();

        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            functions,
            consts: input.consts,
            span: input.span,
        }
    }
}
