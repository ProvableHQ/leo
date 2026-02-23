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

use super::*;

use itertools::Itertools as _;

/// A tuple expression, e.g., `(foo, false, 42)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TupleExpression {
    /// The elements of the tuple.
    /// In the example above, it would be `foo`, `false`, and `42`.
    pub elements: Vec<Expression>,
    /// The span from `(` to `)`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for TupleExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.elements.iter().format(", "))
    }
}

impl From<TupleExpression> for Expression {
    fn from(value: TupleExpression) -> Self {
        Expression::Tuple(value)
    }
}

crate::simple_node_impl!(TupleExpression);
