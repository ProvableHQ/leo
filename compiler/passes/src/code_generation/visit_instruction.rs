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

use crate::CodeGenerator;

use leo_ast::Instruction;

use itertools::Itertools;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_instruction(&mut self, instruction: &'a Instruction) -> String {
        // Visit the instruction operands.
        // Note that parsing guarantees that the operands are either literals, identifiers, or member accesses.
        let operands = instruction
            .operands
            .iter()
            .map(|operand| self.visit_expression(operand).0)
            .join(" ");
        // Construct the new destination registers and add them to mapping.
        let destinations = instruction
            .destinations
            .iter()
            .map(|identifier| {
                let destination_register = format!("r{}", self.next_register);
                // Increment the register counter.
                self.next_register += 1;
                // Add the destination to the mapping.
                self.variable_mapping
                    .insert(&identifier.name, destination_register.clone());
                destination_register
            })
            .join(" ");
        format!("{} {} into {};", instruction.opcode, operands, destinations)
    }
}
