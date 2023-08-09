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

/// A ternary conditional expression, that is, `condition ? if_true : if_false`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TernaryExpression {
    /// The condition determining which branch to pick.
    pub condition: Box<Expression>,
    /// The branch the expression evaluates to if `condition` evaluates to true.
    pub if_true: Box<Expression>,
    /// The branch the expression evaluates to if `condition` evaluates to false.
    pub if_false: Box<Expression>,
    /// The span from `condition` to `if_false`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for TernaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ? {} : {})", self.condition, self.if_true, self.if_false)
    }
}

crate::simple_node_impl!(TernaryExpression);
