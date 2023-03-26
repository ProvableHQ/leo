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

use crate::*;

/// A Reconstructor trait for instructions in the AST.
pub trait InstructionReconstructor: ExpressionReconstructor {
    fn reconstruct_instruction(&mut self, input: Instruction) -> (Instruction, Self::AdditionalOutput) {
        (
            Instruction {
                opcode: input.opcode,
                operands: input
                    .operands
                    .into_iter()
                    .map(|expr| self.reconstruct_expression(expr).0)
                    .collect(),
                destinations: input.destinations,
                span: Default::default(),
            },
            Default::default(),
        )
    }
}
