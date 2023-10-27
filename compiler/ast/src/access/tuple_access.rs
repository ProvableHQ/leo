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

use crate::{Expression, Node, NodeID, NonNegativeNumber};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A tuple access expression, e.g., `tuple.index`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TupleAccess {
    /// An expression evaluating to some tuple type, e.g., `(5, 2)`.
    pub tuple: Box<Expression>,
    /// The index to access in the tuple expression. E.g., `0` for `(5, 2)` would yield `5`.
    pub index: NonNegativeNumber,
    /// The span for the entire expression `tuple.index`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for TupleAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.tuple, self.index)
    }
}

crate::simple_node_impl!(TupleAccess);
