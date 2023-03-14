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

pub mod instruction;
pub use instruction::*;

use crate::{Node};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An assembly block `{ [instruction]* }` consisting of a list of instructions to execute in order.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AssemblyBlock {
    /// The list of instructions to execute.
    pub instructions: Vec<Instruction>,
    /// The span from `{` to `}`.
    pub span: Span,
}

impl fmt::Display for AssemblyBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "asm {{")?;
        if self.instructions.is_empty() {
            writeln!(f, "\t")?;
        } else {
            self.instructions
                .iter()
                .try_for_each(|instruction| writeln!(f, "\t{instruction}"))?;
        }
        write!(f, "}};")
    }
}

crate::simple_node_impl!(AssemblyBlock);
