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

// TODO: See if we can use snarkVM instructions directly, once they are stabilized.

use crate::{simple_node_impl, Identifier, Node, Opcode, Operand};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An Aleo instruction found in an assembly block. For example, `add a b into c;`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Instruction {
    /// The operation being performed by the instruction.
    pub opcode: Opcode,
    /// The arguments to the instruction.
    pub operands: Vec<Operand>,
    /// The identifiers to which the results of the instruction are assigned.
    pub destinations: Vec<Identifier>,
    /// The span excluding the `;` at the end of the instruction.
    pub span: Span,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.opcode)?;

        // Write the operands separated by a space.
        let mut operands = self.operands.iter();
        if let Some(first) = operands.next() {
            write!(f, "{}", first)?;

            for operand in operands {
                write!(f, ", {}", operand)?;
            }
        }

        if !self.destinations.is_empty() {
            write!(f, " into ")?;
            // Write the destinations separated by a space.
            let mut destinations = self.destinations.iter();
            if let Some(first) = destinations.next() {
                write!(f, "{}", first)?;

                for destination in destinations {
                    write!(f, ", {}", destination)?;
                }
            }
        }
        write!(f, ";")
    }
}

simple_node_impl!(Instruction);
