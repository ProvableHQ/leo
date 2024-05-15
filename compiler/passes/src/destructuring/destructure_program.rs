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

use crate::Destructurer;

use leo_ast::{Function, ProgramReconstructor, StatementReconstructor};

impl ProgramReconstructor for Destructurer<'_> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
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
}
