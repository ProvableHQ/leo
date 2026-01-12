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
/// E.g., `arr[2..5]`, `arr[0..=3]`, `arr[1..]`, `arr[..4]`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceExpression {
    /// The array being sliced.
    pub array: Expression,
    /// The optional starting index.
    pub start: Option<Expression>,
    /// The optional ending index and whether it's inclusive.
    pub end: Option<(bool, Expression)>,
    /// The span.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for SliceExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the start expression.
        let start = match &self.start {
            Some(expr) => expr.to_string(),
            None => "".to_string(),
        };
        // Format the end expression.
        let end = match &self.end {
            Some((true, expr)) => format!("={expr}"),
            Some((false, expr)) => expr.to_string(),
            None => "".to_string(),
        };

        write!(f, "{}[{start}..{end}]", self.array)
    }
}

impl From<SliceExpression> for Expression {
    fn from(value: SliceExpression) -> Self {
        Expression::Slice(Box::new(value))
    }
}

crate::simple_node_impl!(SliceExpression);
