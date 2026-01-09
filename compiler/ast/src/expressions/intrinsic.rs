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
use leo_span::Symbol;

use itertools::Itertools as _;

/// An intrinsic call, e.g.`_foo(args)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntrinsicExpression {
    /// Which intrinsic is being called
    pub name: Symbol,
    /// Type parameters for generic intrinsics.
    pub type_parameters: Vec<(Type, Span)>,
    /// Expressions for the arguments passed to the function's parameters.
    pub arguments: Vec<Expression>,
    /// Span of the entire call `function(arguments)`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for IntrinsicExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format type parameters if they exist.
        let type_parameters = if !self.type_parameters.is_empty() {
            format!("::[{}]", self.type_parameters.iter().map(|(t, _)| t.to_string()).format(", "))
        } else {
            String::new()
        };
        write!(f, "{}{type_parameters}({})", self.name, self.arguments.iter().format(", "))
    }
}

impl From<IntrinsicExpression> for Expression {
    fn from(value: IntrinsicExpression) -> Self {
        Expression::Intrinsic(Box::new(value))
    }
}

crate::simple_node_impl!(IntrinsicExpression);
