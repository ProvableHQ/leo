// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::Inliner;

use leo_ast::{Program, ProgramReconstructor};

use indexmap::IndexMap;

impl ProgramReconstructor for Inliner<'_> {
    fn reconstruct_program(&mut self, program: Program) -> Program {
        // Get the original order of the functions. This is necessary to preserve the original order of the functions in the program.
        let function_names = program.functions.keys().cloned().collect::<Vec<_>>();
        // Store the functions for lookup.
        self.functions = program.functions;

        // Get the topological order of the call graph.
        // TODO: This function has already been called in type checking. Reorganize once passes are toggleable.
        let topological_order = self.call_graph.topological_sort().unwrap();

        // Inline the functions in the reverse topological order.
        // Inlining in reverse topological order ensures that all callee functions have been inlined before the caller function is inlined.
        for function_name in topological_order.iter().rev() {
            // Note that this unwrap is safe since type checking guarantees that the function exists.
            let function = self.functions.remove(function_name).unwrap();
            // Inline the function calls.
            let inlined_function = self.reconstruct_function(function);
            // Insert the function back in the map.
            self.functions.insert(*function_name, inlined_function);
        }

        // Rearrange the functions in the original order.
        let mut functions = IndexMap::new();
        for name in function_names {
            // Note that the unwrap is safe since `self.functions` contains all functions in the program.
            functions.insert(name, self.functions.remove(&name).unwrap());
        }

        Program {
            name: program.name,
            network: program.network,
            expected_input: program.expected_input,
            imports: program.imports,
            circuits: program.circuits,
            functions,
        }
    }
}
