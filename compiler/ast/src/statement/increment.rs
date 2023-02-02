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

use crate::{Expression, Identifier, Node};

use leo_span::Span;

use core::fmt;
use serde::{Deserialize, Serialize};

/// An increment statement `increment(foo, bar, 1);`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IncrementStatement {
    /// The mapping to be modified.
    pub mapping: Identifier,
    /// The index of the element to be incremented.
    pub index: Expression,
    /// The amount to increment the element by.
    pub amount: Expression,
    /// The span of `increment(foo, bar, 1)` excluding the semicolon.
    pub span: Span,
}

impl fmt::Display for IncrementStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "increment({}, {}, {});", self.mapping, self.index, self.amount)
    }
}

crate::simple_node_impl!(IncrementStatement);
