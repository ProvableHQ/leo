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

use super::*;

use itertools::Itertools as _;

/// A dynamic call expression, e.g. `MyInterface@(target)/foobar(args)`.
///
/// This represents a dynamic call where:
/// - `interface` is the interface name (e.g. `MyInterface`)
/// - `target_program` is the expression containing the target program (a `field` or `identifier` value)
/// - `network` is the expression containing the target program's network (an optional `identifier` value)
/// - `function` is the function to call on the target (e.g. `foobar`)
/// - `arguments` are the arguments passed to the function
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicCallExpression {
    /// The interface path.
    pub interface: Type,
    /// The target expression.
    pub target_program: Expression,
    /// The optional network expression (defaults to 'aleo' if None).
    pub network: Option<Expression>,
    /// The function to call.
    pub function: Identifier,
    /// The arguments to the function.
    pub arguments: Vec<Expression>,
    /// The span of the entire expression.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for DynamicCallExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(network) = &self.network {
            write!(
                f,
                "{}@({}, {})/{}({})",
                self.interface,
                self.target_program,
                network,
                self.function,
                self.arguments.iter().format(", ")
            )
        } else {
            write!(
                f,
                "{}@({})/{}({})",
                self.interface,
                self.target_program,
                self.function,
                self.arguments.iter().format(", ")
            )
        }
    }
}

impl From<DynamicCallExpression> for Expression {
    fn from(value: DynamicCallExpression) -> Self {
        Expression::DynamicCall(Box::new(value))
    }
}

crate::simple_node_impl!(DynamicCallExpression);
