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

/// A ternary conditional expression, that is, `condition ? if_true : if_false`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TernaryExpression {
    /// The condition determining which branch to pick.
    pub condition: Expression,
    /// The branch the expression evaluates to if `condition` evaluates to true.
    pub if_true: Expression,
    /// The branch the expression evaluates to if `condition` evaluates to false.
    pub if_false: Expression,
    /// The span from `condition` to `if_false`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for TernaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let maybe_paren = |f: &mut fmt::Formatter, expr: &Expression| {
            if expr.precedence() > 0 { write!(f, "{expr}") } else { write!(f, "({expr})") }
        };

        maybe_paren(f, &self.condition)?;
        write!(f, " ? ")?;
        maybe_paren(f, &self.if_true)?;
        write!(f, " : ")?;
        maybe_paren(f, &self.if_false)
    }
}

impl From<TernaryExpression> for Expression {
    fn from(value: TernaryExpression) -> Self {
        Expression::Ternary(Box::new(value))
    }
}

crate::simple_node_impl!(TernaryExpression);
