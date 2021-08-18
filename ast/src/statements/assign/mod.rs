// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use leo_errors::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

mod assignee;
pub use assignee::*;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum AssignOperation {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Or,
    And,
    BitOr,
    BitAnd,
    BitXor,
    Shr,
    ShrSigned,
    Shl,
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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AssignStatement {
    pub operation: AssignOperation,
    pub assignee: Assignee,
    pub value: Expression,
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
