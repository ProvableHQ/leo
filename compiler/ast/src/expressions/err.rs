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

/// Represents a syntactically invalid expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrExpression {
    /// The span of the invalid expression.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for ErrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("error")
    }
}

crate::simple_node_impl!(ErrExpression);
