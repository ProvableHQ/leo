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

/// A function call expression, e.g.`foo(args)` or `Foo::bar(args)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallExpression {
    /// A path to a callable function, either a member of a structure or a free function.
    pub function: Path,
    /// Expressions for the const arguments passed to the function's const parameters.
    pub const_arguments: Vec<Expression>,
    /// Expressions for the arguments passed to the function's parameters.
    pub arguments: Vec<Expression>,
    /// The name of the parent program call, e.g.`bar` in `bar.aleo`.
    pub program: Option<Symbol>,
    /// Span of the entire call `function(arguments)`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.function)?;
        if !self.const_arguments.is_empty() {
            write!(f, "::[{}]", self.const_arguments.iter().format(", "))?;
        }
        write!(f, "({})", self.arguments.iter().format(", "))
    }
}

impl From<CallExpression> for Expression {
    fn from(value: CallExpression) -> Self {
        Expression::Call(Box::new(value))
    }
}

crate::simple_node_impl!(CallExpression);
