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

/// A unary instruction.
pub trait Unary {
    /// Returns the opcode of the instruction.
    fn opcode() -> &'static str;
    /// Returns a new instance of the instruction.
    fn new(source: Operand, destination: Identifier, span: Span) -> Self;
}

macro_rules! unary_instruction {
    ($name:ident, $opcode:expr) => {
        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
        pub struct $name {
            pub source: Operand,
            pub destination: Identifier,
            pub span: Span,
        }

        impl Unary for $name {
            fn opcode() -> &'static str { $opcode }

            fn new(source: Operand, destination: Identifier, span: Span) -> Self {
                Self { source, destination, span, }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {} into {};", Self::opcode(), self.source, self.destination)
            }
        }

        crate::simple_node_impl!($name);
    };
}

unary_instruction!(Abs, "abs");
unary_instruction!(AbsWrapped, "abs.w");
unary_instruction!(Double, "double");
unary_instruction!(HashBHP256, "hash.bhp256");
unary_instruction!(HashBHP512, "hash.bhp512");
unary_instruction!(HashBHP768, "hash.bhp768");
unary_instruction!(HashBHP1024, "hash.bhp1024");
unary_instruction!(HashPED64, "hash.ped64");
unary_instruction!(HashPED128, "hash.ped128");
unary_instruction!(HashPSD2, "hash.psd2");
unary_instruction!(HashPSD4, "hash.psd4");
unary_instruction!(HashPSD8, "hash.psd8");
unary_instruction!(Inv, "inv");
unary_instruction!(Neg, "neg");
unary_instruction!(Not, "not");
unary_instruction!(Square, "square");
unary_instruction!(SquareRoot, "sqrt");
