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

/// A method call expression, e.g., `1u8.add(2u8)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodCallExpression {
    /// The receiver of a method call, e.g. `1u8` in `1u8.add(2u8)`.
    pub receiver: Box<Expression>,
    /// The identifier of the called method.
    pub method: Identifier,
    /// Expressions for the arguments passed to the methods parameters.
    pub arguments: Vec<Expression>,
    /// Span of the entire call `receiver.method(arguments)`.
    pub span: Span,
}

impl fmt::Display for MethodCallExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.", self.receiver)?;
        write!(f, "{}(", self.method)?;
        for (i, param) in self.arguments.iter().enumerate() {
            write!(f, "{}", param)?;
            if i < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

crate::simple_node_impl!(MethodCallExpression);
