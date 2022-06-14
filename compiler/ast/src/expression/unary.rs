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
use leo_span::Symbol;

/// A unary operator for a unary expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperation {
    /// Absolute value checking for overflow, i.e. `.abs()`.
    Abs,
    /// Absolute value wrapping around at the boundary of the type, i.e. `.abs_wrapped()`.
    AbsWrapped,
    /// Double operation, i.e. `.double()`.
    Double,
    /// Multiplicative inverse, i.e. `.inv()`.
    Inverse,
    /// Negate operation, i.e. `.neg()`.
    Negate,
    /// Bitwise NOT, i.e. `!`, `.not()`.
    Not,
    /// Square operation, i.e. `.square()`.
    Square,
    /// Square root operation, i.e. `.sqrt()`.
    SquareRoot,
}

impl UnaryOperation {
    /// Returns a `UnaryOperation` from the given `Symbol`.
    pub fn from_symbol(symbol: &Symbol) -> Option<UnaryOperation> {
        Some(match symbol.as_u32() {
            0 => UnaryOperation::Abs,
            1 => UnaryOperation::AbsWrapped,
            2 => UnaryOperation::Double,
            3 => UnaryOperation::Inverse,
            4 => UnaryOperation::Negate,
            5 => UnaryOperation::Not,
            6 => UnaryOperation::Square,
            7 => UnaryOperation::SquareRoot,
            _ => return None,
        })
    }
}

impl AsRef<str> for UnaryOperation {
    fn as_ref(&self) -> &'static str {
        match self {
            UnaryOperation::Abs => "abs",
            UnaryOperation::AbsWrapped => "abs_wrapped",
            UnaryOperation::Double => "double",
            UnaryOperation::Inverse => "inv",
            UnaryOperation::Negate => "-",
            UnaryOperation::Not => "!",
            UnaryOperation::Square => ".square",
            UnaryOperation::SquareRoot => "sqrt"
        }
    }
}

/// An unary expression applying an operator to an inner expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnaryExpression {
    /// The inner expression `op` is applied to.
    pub receiver: Box<Expression>,
    /// The unary operator to apply to `inner`.
    pub op: UnaryOperation,
    /// The span covering `op inner`.
    pub span: Span,
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.op.as_ref(), self.receiver)
    }
}

crate::simple_node_impl!(UnaryExpression);
