// Copyright (C) 2019-2026 Provable Inc.
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

/// A method call `receiver.method(arguments)`.
///
/// This is a transient node: the parser produces it for every non-operator `.method(..)` call
/// because the receiver's type (and hence which method is meant) is unknown at parse time. Type
/// checking resolves it by the receiver's type, and the Disambiguate pass rewrites it into either
/// an [`crate::IntrinsicExpression`] (built-in methods on `Vector`/`Mapping`/`Optional`/`signature`/
/// `Future`) or a [`crate::CallExpression`] (a user method on a struct/record). It never survives
/// past Disambiguate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodCall {
    /// The receiver the method is called on.
    pub receiver: Expression,
    /// The method name. Not a path — it is resolved against the receiver's type, not by name.
    pub method: Identifier,
    /// The explicit arguments (the receiver is not included here).
    pub arguments: Vec<Expression>,
    /// The span covering all of `receiver.method(arguments)`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for MethodCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}({})", self.receiver, self.method, self.arguments.iter().format(", "))
    }
}

impl From<MethodCall> for Expression {
    fn from(value: MethodCall) -> Self {
        Expression::MethodCall(Box::new(value))
    }
}

crate::simple_node_impl!(MethodCall);
