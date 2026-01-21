// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{Block, Expression, Indent, Node, NodeID};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An `async` block: e.g. `async { my_mapping.set(1, 2); }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncExpression {
    /// The block to run asynchronously.
    pub block: Block,
    /// The span for the entire expression `async { my_mapping.set(1, 2); }`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for AsyncExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "async {{")?;
        for stmt in self.block.statements.iter() {
            writeln!(f, "{}{}", Indent(stmt), stmt.semicolon())?;
        }
        write!(f, "}}")
    }
}

impl From<AsyncExpression> for Expression {
    fn from(value: AsyncExpression) -> Self {
        Expression::Async(value)
    }
}

crate::simple_node_impl!(AsyncExpression);
