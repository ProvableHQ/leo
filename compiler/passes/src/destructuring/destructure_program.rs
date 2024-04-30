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

use leo_ast::{Finalize, Function, ProgramReconstructor, StatementReconstructor};

impl ProgramReconstructor for Destructurer<'_> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            input: input.input,
            output: input.output,
            output_type: input.output_type,
            block: self.reconstruct_block(input.block).0,
            finalize: input.finalize.map(|finalize| {
                // Set the `is_finalize` flag before reconstructing the finalize block.
                self.is_finalize = true;
                // Reconstruct the finalize block.
                let finalize = Finalize {
                    identifier: finalize.identifier,
                    input: finalize.input,
                    output: finalize.output,
                    output_type: finalize.output_type,
                    block: self.reconstruct_block(finalize.block).0,
                    span: finalize.span,
                    id: finalize.id,
                };
                // Reset the `is_finalize` flag.
                self.is_finalize = false;
                finalize
            }),
            span: input.span,
            id: input.id,
        }
    }
}
