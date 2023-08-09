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

use super::*;
use leo_span::{sym, Symbol};

/// A unary operator for a unary expression.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    /// Converts a group element to its x-coordinate, i.e. `.to_x_coordinate()`.
    ToXCoordinate,
    /// Converts a group element to its y-coordinate, i.e. `.to_y_coordinate()`.
    ToYCoordinate,
}

impl UnaryOperation {
    /// Returns a `UnaryOperation` from the given `Symbol`.
    pub fn from_symbol(symbol: Symbol) -> Option<Self> {
        Some(match symbol {
            sym::abs => Self::Abs,
            sym::abs_wrapped => Self::AbsWrapped,
            sym::double => Self::Double,
            sym::inv => Self::Inverse,
            sym::neg => Self::Negate,
            sym::not => Self::Not,
            sym::square => Self::Square,
            sym::square_root => Self::SquareRoot,
            sym::to_x_coordinate => Self::ToXCoordinate,
            sym::to_y_coordinate => Self::ToYCoordinate,
            _ => return None,
        })
    }

    /// Represents the opera.tor as a string.
    fn as_str(self) -> &'static str {
        match self {
            Self::Abs => "abs",
            Self::AbsWrapped => "abs_wrapped",
            Self::Double => "double",
            Self::Inverse => "inv",
            Self::Negate => "neg",
            Self::Not => "not",
            Self::Square => "square",
            Self::SquareRoot => "square_root",
            Self::ToXCoordinate => "to_x_coordinate",
            Self::ToYCoordinate => "to_y_coordinate",
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
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.op.as_str(), self.receiver)
    }
}

crate::simple_node_impl!(UnaryExpression);
