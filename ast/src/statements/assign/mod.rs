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

use crate::{Expression, Node};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

mod assignee;
pub use assignee::*;

/// The assignment operator.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum AssignOperation {
    /// Plain assignment, `=`.
    Assign,
    /// Add-assignment, `+=`.
    Add,
    /// Subtracting assignment, `-=`.
    Sub,
    /// Multiplicating assignment, `*=`.
    Mul,
    /// Divising-assignment, `/=`.
    Div,
    /// Exponentating assignment `**=`.
    Pow,
    /// Logical or assignment.
    Or,
    /// Logical and assignment.
    And,
    /// Bitwise or assignment.
    BitOr,
    /// Bitwise and assignment.
    BitAnd,
    /// Bitwise xor assignment.
    BitXor,
    /// Shift right assignment.
    Shr,
    /// Signed shift right assignment.
    ShrSigned,
    /// Shift left assignment.
    Shl,
    /// Modulus / remainder assignment.
    Mod,
}

impl AsRef<str> for AssignOperation {
    fn as_ref(&self) -> &'static str {
        match self {
            AssignOperation::Assign => "=",
            AssignOperation::Add => "+=",
            AssignOperation::Sub => "-=",
            AssignOperation::Mul => "*=",
            AssignOperation::Div => "/=",
            AssignOperation::Pow => "**=",
            AssignOperation::Or => "||=",
            AssignOperation::And => "&&=",
            AssignOperation::BitOr => "|=",
            AssignOperation::BitAnd => "&=",
            AssignOperation::BitXor => "^=",
            AssignOperation::Shr => ">>=",
            AssignOperation::ShrSigned => ">>>=",
            AssignOperation::Shl => "<<=",
            AssignOperation::Mod => "%=",
        }
    }
}

/// An assignment statement, `assignee operation? = value`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AssignStatement {
    /// The assignment operation.
    /// For plain assignment, use `AssignOperation::Assign`.
    pub operation: AssignOperation,
    /// The place to assign to.
    pub assignee: Assignee,
    /// The value to assign to the `assignee`.
    pub value: Expression,
    /// The span, excluding the semicolon.
    pub span: Span,
}

impl fmt::Display for AssignStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {};", self.assignee, self.operation.as_ref(), self.value)
    }
}

impl Node for AssignStatement {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
