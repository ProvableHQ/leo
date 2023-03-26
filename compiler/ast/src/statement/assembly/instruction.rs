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

use crate::{Expression, Identifier, Node, Opcode};

use core::fmt;
use itertools::Itertools;
use leo_span::Span;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// An AVM instruction, e.g. `add foo bar into baz;`.
/// Note that `call` and `cast` are excluded.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Expression>,
    pub destinations: Vec<Identifier>,
    pub span: Span,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} into {};",
            self.opcode,
            self.operands.iter().join(" "),
            self.destinations.iter().join(" ")
        )
    }
}

crate::simple_node_impl!(Instruction);
