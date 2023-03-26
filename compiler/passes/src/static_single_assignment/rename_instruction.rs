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

use crate::StaticSingleAssigner;

use leo_ast::{Expression, ExpressionConsumer, Instruction, InstructionConsumer, Statement};

impl InstructionConsumer for StaticSingleAssigner<'_> {
    type Output = (Instruction, Vec<Statement>);

    fn consume_instruction(&mut self, input: Instruction) -> Self::Output {
        let mut statements = Vec::new();
        // First consume the operands of the instruction.
        let operands = input
            .operands
            .into_iter()
            .map(|operand| {
                let (value, mut stmts) = self.consume_expression(operand);
                statements.append(&mut stmts);
                value
            })
            .collect();

        // Then assign a new unique name to the left-hand-side of the assignment.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
        self.is_lhs = true;
        let destinations = input
            .destinations
            .into_iter()
            .map(|destination| {
                if let (Expression::Identifier(destination), mut stmts) = self.consume_identifier(destination) {
                    statements.append(&mut stmts);
                    destination
                } else {
                    unreachable!("`consume_identifier` always returns an identifier")
                }
            })
            .collect();
        self.is_lhs = false;

        // Finally, reconstruct the instruction.
        (
            Instruction {
                opcode: input.opcode,
                operands,
                destinations,
                span: input.span,
            },
            statements,
        )
    }
}
