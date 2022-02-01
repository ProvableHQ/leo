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

/// The unary operator for an unary expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperation {
    /// The logical not operator, i.e., `!`.
    /// For example, it transforms `true` to `false`.
    Not,
    /// The negation operator, i.e., `-`.
    Negate,
    /// The bitwise not operator, i.e., `~`.
    /// For example, it transforms `1010` to `0101`.
    BitNot,
}

impl AsRef<str> for UnaryOperation {
    fn as_ref(&self) -> &'static str {
        match self {
            UnaryOperation::Not => "!",
            UnaryOperation::Negate => "-",
            UnaryOperation::BitNot => "~",
        }
    }
}

/// An unary expression applying an operator to an inner expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnaryExpression {
    /// The inner expression `op` is applied to.
    pub inner: Box<Expression>,
    /// The unary operator to apply to `inner`.
    pub op: UnaryOperation,
    /// The span covering `op inner`.
    pub span: Span,
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.op.as_ref(), self.inner)
    }
}

impl Node for UnaryExpression {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
