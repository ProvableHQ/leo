// Copyright (C) 2019-2025 Provable Inc.
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

/// An array expression constructed from one repeated element.
///
/// E.g., `[1u32; 5]`. Expression `expr` is the element to be repeated; Expression `count` is the number of times to
/// repeat it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepeatExpression {
    /// The element to repeat.
    pub expr: Expression,
    /// The number of times to repeat it.
    pub count: Expression,
    /// The span from `[` to `]`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for RepeatExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self.expr, self.count)
    }
}

impl From<RepeatExpression> for Expression {
    fn from(value: RepeatExpression) -> Self {
        Expression::Repeat(Box::new(value))
    }
}

crate::simple_node_impl!(RepeatExpression);
