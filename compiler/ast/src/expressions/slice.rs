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

/// An array constructed from slicing another.
///
/// E.g., `arr[2:5]`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceExpression {
    /// The array being sliced.
    pub array: Expression,
    /// The starting index.
    pub start: Expression,
    /// The ending index.
    pub end: Expression,
    /// The span from `[` to `]`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for SliceExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}..{}{}]", self.array, self.start, if self.inclusive { "=" } else { "" }, self.end)
    }
}

impl From<SliceExpression> for Expression {
    fn from(value: SliceExpression) -> Self {
        Expression::Slice(Box::new(value))
    }
}

crate::simple_node_impl!(SliceExpression);
