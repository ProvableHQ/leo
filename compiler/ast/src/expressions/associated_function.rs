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

use crate::{Expression, Identifier, Node, NodeID};
use leo_span::Span;

use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use std::fmt;

/// An access expression to an associated function in a struct, e.g.`Pedersen64::hash()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssociatedFunctionExpression {
    /// The inner struct variant.
    pub variant: Identifier,
    /// The static struct member function that is being accessed.
    pub name: Identifier,
    /// The arguments passed to the function `name`.
    pub arguments: Vec<Expression>,
    /// The span for the entire expression `Foo::bar()`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for AssociatedFunctionExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}({})", self.variant, self.name, self.arguments.iter().format(", "))
    }
}

impl From<AssociatedFunctionExpression> for Expression {
    fn from(value: AssociatedFunctionExpression) -> Self {
        Expression::AssociatedFunction(value)
    }
}

crate::simple_node_impl!(AssociatedFunctionExpression);
