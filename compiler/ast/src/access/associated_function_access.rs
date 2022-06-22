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

use crate::{Expression, Identifier, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An access expression to an associated function in a circuit, e.g.`Pedersen64::hash()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssociatedFunctionCall {
    /// The inner circuit type.
    pub inner: Box<Expression>,
    /// The static circuit member function that is being accessed.
    pub name: Identifier,
    /// The arguments passed to the function `name`.
    pub args: Vec<Expression>,
    /// The span for the entire expression `Foo::bar()`.
    pub span: Span,
}

impl fmt::Display for AssociatedFunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.inner, self.name)
    }
}

crate::simple_node_impl!(AssociatedFunctionCall);
