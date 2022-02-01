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

use super::*;

/// An array initializer expression, e.g., `[42; 5]`.
/// constructing an array of `element` repeated according to `dimensions`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayInitExpression {
    /// The expression that all elements in the array will evaluate to.
    pub element: Box<Expression>,
    /// The dimensions of the array.
    ///
    /// This could be a multi-dimensional array,
    /// e.g., `[42; (2, 2)]`, giving you a matrix `[[2, 2], [2, 2]]`.
    pub dimensions: ArrayDimensions,
    /// The span of the entire expression from `[` to `]`.
    pub span: Span,
}

impl fmt::Display for ArrayInitExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self.element, self.dimensions)
    }
}

impl Node for ArrayInitExpression {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
