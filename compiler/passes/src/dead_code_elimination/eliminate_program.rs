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

use crate::DeadCodeEliminator;

use leo_ast::{Finalize, Function, ProgramReconstructor, StatementReconstructor};

impl ProgramReconstructor for DeadCodeEliminator<'_> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        // Reset the state of the dead code eliminator.
        self.used_variables.clear();
        self.is_necessary = false;

        // Traverse the function body.
        let block = self.reconstruct_block(input.block).0;

        // Reconstruct the finalize block, if it exists.
        let finalize = input.finalize.map(|finalize| {
            // Reset the state of the dead code eliminator.
            self.used_variables.clear();
            self.is_necessary = false;

            // Traverse the finalize block.
            let block = self.reconstruct_block(finalize.block).0;

            Finalize {
                identifier: finalize.identifier,
                input: finalize.input,
                output: finalize.output,
                output_type: finalize.output_type,
                block,
                span: finalize.span,
                id: finalize.id,
            }
        });

        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            input: input.input,
            output: input.output,
            output_type: input.output_type,
            block,
            finalize,
            span: input.span,
            id: input.id,
        }
    }
}
