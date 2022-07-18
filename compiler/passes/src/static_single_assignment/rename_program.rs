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

use crate::StaticSingleAssigner;

use leo_ast::{Function, FunctionInput, ProgramReconstructor, StatementReconstructor};

impl<'a> ProgramReconstructor for StaticSingleAssigner<'a> {
    /// Reduces the `Function`s in the `Program`, while allocating the appropriate `RenameTable`s.
    fn reconstruct_function(&mut self, function: Function) -> Function {
        // Allocate a `RenameTable` for the function.
        self.push();

        // There is no need to reconstruct `function.inputs`.
        // However, for each input, we must add each symbol to the rename table.
        for input in function.input.iter() {
            match input {
                FunctionInput::Variable(function_input_variable) => {
                    self.rename_table.update(
                        function_input_variable.identifier.name,
                        function_input_variable.identifier.name,
                    );
                }
            }
        }

        let block = self.reconstruct_block(function.block);

        // Remove the `RenameTable` for the function.
        self.pop();

        Function {
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            core_mapping: function.core_mapping,
            block,
            span: function.span,
        }
    }
}
