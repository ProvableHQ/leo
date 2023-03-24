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

use crate::{Identifier, Node, Operand};

use leo_span::Span;

use core::fmt;
use serde::{Deserialize, Serialize};

/// A binary instruction.
pub trait Binary {
    /// Returns the opcode of the instruction.
    fn opcode() -> &'static str;
    /// Returns a new instance of the instruction.
    fn new(first: Operand, second: Operand, destination: Identifier, span: Span) -> Self;
}

macro_rules! binary_instruction {
    ($name:ident, $opcode:expr) => {
        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
        pub struct $name {
            pub first: Operand,
            pub second: Operand,
            pub destination: Identifier,
            pub span: Span,
        }

        impl Binary for $name {
            fn opcode() -> &'static str { $opcode }

            fn new(first: Operand, second: Operand, destination: Identifier, span: Span) -> Self {
                Self { first, second, destination, span }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

binary_instruction!(Add, "add");
binary_instruction!(AddWrapped, "add.w");
binary_instruction!(And, "and");
binary_instruction!(AssertEq, "assert.eq");
binary_instruction!(AssertNeq, "assert.neq");
binary_instruction!(CommitBHP256, "commit.bhp256");
binary_instruction!(CommitBHP512, "commit.bhp512");
binary_instruction!(CommitBHP768, "commit.bhp768");
binary_instruction!(CommitBHP1024, "commit.bhp1024");
binary_instruction!(CommitPED64, "commit.ped64");
binary_instruction!(CommitPED128, "commit.ped128");
binary_instruction!(Div, "div");
binary_instruction!(DivWrapped, "div.w");
binary_instruction!(GreaterThan, "gt");
binary_instruction!(GreaterThanOrEqual, "gte");
binary_instruction!(IsEq, "is.eq");
binary_instruction!(IsNeq, "is.neq");
binary_instruction!(LessThan, "lt");
binary_instruction!(LessThanOrEqual, "lte");
binary_instruction!(Modulo, "mod");
binary_instruction!(Mul, "mul");
binary_instruction!(MulWrapped, "mul.w");
binary_instruction!(Nand, "nand");
binary_instruction!(Nor, "nor");
binary_instruction!(Or, "or");
binary_instruction!(Pow, "pow");
binary_instruction!(PowWrapped, "pow.w");
binary_instruction!(Rem, "rem");
binary_instruction!(RemWrapped, "rem.w");
binary_instruction!(Shl, "shl");
binary_instruction!(ShlWrapped, "shl.w");
binary_instruction!(Shr, "shr");
binary_instruction!(ShrWrapped, "shr.w");
binary_instruction!(Sub, "sub");
binary_instruction!(SubWrapped, "sub.w");
binary_instruction!(Xor, "xor");
