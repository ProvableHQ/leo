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

use crate::Type;

/// A cast expression, e.g. `42u8 as u16`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CastExpression {
    /// The expression to be casted, e.g.`42u8` in `42u8 as u16`.
    pub expression: Box<Expression>,
    /// The type to be casted to, e.g. `u16` in `42u8 as u16`.
    pub type_: Type,
    /// Span of the entire cast `42u8 as u16`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for CastExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} as {})", self.expression, self.type_)
    }
}

crate::simple_node_impl!(CastExpression);
