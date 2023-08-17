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

/// A function call expression, e.g.`foo(args)` or `Foo::bar(args)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallExpression {
    /// An expression evaluating to a callable function,
    /// either a member of a structure or a free function.
    pub function: Box<Expression>, // todo: make this identifier?
    /// Expressions for the arguments passed to the functions parameters.
    pub arguments: Vec<Expression>,
    /// The name of the external program call, e.g.`bar` in `bar.leo`.
    pub external: Option<Box<Expression>>,
    /// Span of the entire call `function(arguments)`.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.external {
            Some(external) => {
                write!(f, "{external}.leo/{}(", self.function)?;
            }
            None => {
                write!(f, "{}(", self.function)?;
            }
        }

        for (i, param) in self.arguments.iter().enumerate() {
            write!(f, "{param}")?;
            if i < self.arguments.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

crate::simple_node_impl!(CallExpression);
