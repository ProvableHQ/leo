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

use crate::{Expression, Node};

use leo_span::Span;

use core::fmt;
use serde::{Deserialize, Serialize};

/// A return statement `finalize(arg1, ..., argN);`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct FinalizeStatement {
    /// The arguments to pass to the finalize block.
    pub arguments: Vec<Expression>,
    /// The span of `finalize(arg1, ..., argN)` excluding the semicolon.
    pub span: Span,
}

impl fmt::Display for FinalizeStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "finalize(")?;
        for (i, param) in self.arguments.iter().enumerate() {
            write!(f, "{}", param)?;
            if i < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ");")
    }
}

crate::simple_node_impl!(FinalizeStatement);
