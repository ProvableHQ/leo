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

use crate::{Expression, Node, NodeID};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An assignment statement, `assignee = value`.
/// Note that there is no operation associated with the assignment.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AssignStatement {
    /// The place to assign to.
    /// Note that `place` can either be an identifier or tuple.
    pub place: Expression,
    /// The value to assign to the `assignee`.
    pub value: Expression,
    /// The span, excluding the semicolon.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for AssignStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {};", self.place, self.value)
    }
}

crate::simple_node_impl!(AssignStatement);
