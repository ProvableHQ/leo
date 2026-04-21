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

/// A dynamic operation against an interface at a runtime target address.
///
/// Covers three surface forms:
///
/// - **Function call** (`DynamicOpKind::Call`):
///   `Interface@(target[, network])::func(args)` — invokes an interface function.
///
/// - **Bare storage read** (`DynamicOpKind::Read`):
///   `Interface@(target[, network])::name` — reads a singleton storage variable
///   declared in the interface, producing `Option<T>`.
///
/// - **Storage member operation** (`DynamicOpKind::Op`):
///   `Interface@(target[, network])::member.op(args)` — performs an operation on
///   a mapping (`get`, `get_or_use`, `contains`) or vector (`get`, `len`) storage
///   variable declared in the interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DynamicOpExpression {
    /// The interface type.
    pub interface: Type,
    /// The target program expression (`field` or `identifier`).
    pub target_program: Expression,
    /// The optional network expression; defaults to `'aleo'` when `None`.
    pub network: Option<Expression>,
    /// Which dynamic operation this expression represents.
    pub kind: DynamicOpKind,
    /// The span of the entire expression.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

/// Distinguishes the three surface forms of [`DynamicOpExpression`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicOpKind {
    /// `Interface@(p)::func(args)` — call a function declared in the interface.
    Call { function: Identifier, arguments: Vec<Expression> },
    /// `Interface@(p)::name` — bare read of a singleton storage variable.
    Read { storage: Identifier },
    /// `Interface@(p)::member.op(args)` — op on a mapping or vector storage variable.
    ///
    /// Mappings support `get`, `get_or_use`, `contains`. Vectors support `get`, `len`.
    Op { member: Identifier, op: Identifier, arguments: Vec<Expression> },
}

impl fmt::Display for DynamicOpExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let target = if let Some(network) = &self.network {
            format!("({}, {})", self.target_program, network)
        } else {
            format!("({})", self.target_program)
        };

        match &self.kind {
            DynamicOpKind::Call { function, arguments } => {
                write!(f, "{}@{}::{}({})", self.interface, target, function, arguments.iter().format(", "))
            }
            DynamicOpKind::Read { storage } => {
                write!(f, "{}@{}::{}", self.interface, target, storage)
            }
            DynamicOpKind::Op { member, op, arguments } => {
                write!(f, "{}@{}::{}.{}({})", self.interface, target, member, op, arguments.iter().format(", "))
            }
        }
    }
}

impl From<DynamicOpExpression> for Expression {
    fn from(value: DynamicOpExpression) -> Self {
        Expression::DynamicOp(Box::new(value))
    }
}

crate::simple_node_impl!(DynamicOpExpression);
