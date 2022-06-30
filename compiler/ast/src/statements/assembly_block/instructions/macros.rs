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

#[macro_export]
macro_rules! impl_binary_instruction {
    ($name:ident, $opcode:literal) => {
        /// The `$name` instruction.
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            /// The first operand of the instruction.
            pub first: Operand,
            /// The second operand of the instruction.
            pub second: Operand,
            /// The variable to which the result of the instruction is assigned.
            pub destination: Identifier,
            /// The span excluding the `;` at the end of the instruction.
            pub span: Span,
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{} {} {} into {};",
                    $opcode, self.first, self.second, self.destination
                )
            }
        }

        crate::simple_node_impl!($name);
    };
}

#[macro_export]
macro_rules! impl_ternary_instruction {
    ($name:ident, $opcode:literal) => {
        /// The `$name` instruction.
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            /// The first operand of the instruction.
            pub first: Operand,
            /// The second operand of the instruction.
            pub second: Operand,
            /// The third operand of the instruction.
            pub third: Operand,
            /// The variable to which the result of the instruction is assigned.
            pub destination: Identifier,
            /// The span excluding the `;` at the end of the instruction.
            pub span: Span,
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{} {} {} {} into {};",
                    $opcode, self.first, self.second, self.third, self.destination
                )
            }
        }

        crate::simple_node_impl!($name);
    };
}

#[macro_export]
macro_rules! impl_unary_instruction {
    ($name:ident, $opcode:literal) => {
        /// The `$name` instruction.
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name {
            /// The operand to the instruction.
            pub operand: Operand,
            /// The variable to which the result of the instruction is assigned.
            pub destination: Identifier,
            /// The span excluding the `;` at the end of the instruction.
            pub span: Span,
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{} {} into {};", $opcode, self.operand, self.destination)
            }
        }

        crate::simple_node_impl!($name);
    };
}
