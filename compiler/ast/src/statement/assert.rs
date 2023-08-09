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

use crate::{Expression, Node, NodeID};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A variant of an assert statement.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum AssertVariant {
    /// A `assert(expr)` variant, asserting that the expression evaluates to true.
    Assert(Expression),
    /// A `assert_eq(expr1, expr2)` variant, asserting that the operands are equal.
    AssertEq(Expression, Expression),
    /// A `assert_neq(expr1, expr2)` variant, asserting that the operands are not equal.
    AssertNeq(Expression, Expression),
}

/// An assert statement, `assert(<expr>)`, `assert_eq(<expr>)` or `assert_neq(<expr>)`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AssertStatement {
    /// The variant of the assert statement.
    pub variant: AssertVariant,
    /// The span, excluding the semicolon.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for AssertStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.variant {
            AssertVariant::Assert(ref expr) => write!(f, "assert({expr});"),
            AssertVariant::AssertEq(ref expr1, ref expr2) => write!(f, "assert_eq({expr1}, {expr2});"),
            AssertVariant::AssertNeq(ref expr1, ref expr2) => write!(f, "assert_neq({expr1}, {expr2});"),
        }
    }
}

crate::simple_node_impl!(AssertStatement);
