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

use crate::{Identifier, Operand};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A helper struct defining a unary instruction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnaryInstruction {
    /// The operand.
    pub operand: Operand,
    /// The variable to which the result of the instruction is assigned.
    pub destination: Identifier,
}

impl fmt::Display for UnaryInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} into {}", self.operand, self.destination)
    }
}

#[macro_export]
macro_rules! impl_unary_instruction {
    ($name:ident, $opcode:literal) => {
        /// The `$name` instruction.
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            /// The arity of the instruction.
            pub operation: UnaryInstruction,
            /// The span excluding the `;` at the end of the instruction.
            pub span: Span,
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {};", $opcode, self.operation)
            }
        }

        crate::simple_node_impl!($name);
    };
}
