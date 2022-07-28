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

use crate::{BinaryOperation, Expression, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// The assignment operator.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum AssignOperation {
    /// Plain assignment, `=`.
    Assign,
    /// Adding assignment, `+=`.
    Add,
    /// Subtracting assignment, `-=`.
    Sub,
    /// Multiplying assignment, `*=`.
    Mul,
    /// Dividing-assignment, `/=`.
    Div,
    /// Exponentiating assignment `**=`.
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
    // /// Signed shift right assignment.
    // ShrSigned,
    /// Shift left assignment.
    Shl,
    // /// Modulus / remainder assignment.
    // Mod,
}

impl AssignOperation {
    pub fn into_binary_operation(assign_op: AssignOperation) -> Option<BinaryOperation> {
        match assign_op {
            AssignOperation::Assign => None,
            AssignOperation::Add => Some(BinaryOperation::Add),
            AssignOperation::Sub => Some(BinaryOperation::Sub),
            AssignOperation::Mul => Some(BinaryOperation::Mul),
            AssignOperation::Div => Some(BinaryOperation::Div),
            AssignOperation::Pow => Some(BinaryOperation::Pow),
            AssignOperation::Or => Some(BinaryOperation::Or),
            AssignOperation::And => Some(BinaryOperation::And),
            AssignOperation::BitOr => Some(BinaryOperation::BitwiseOr),
            AssignOperation::BitAnd => Some(BinaryOperation::BitwiseAnd),
            AssignOperation::BitXor => Some(BinaryOperation::Xor),
            AssignOperation::Shr => Some(BinaryOperation::Shr),
            // AssignOperation::ShrSigned => Some(BinaryOperation::ShrSigned),
            AssignOperation::Shl => Some(BinaryOperation::Shl),
            // AssignOperation::Mod => Some(BinaryOperation::Mod),
        }
    }
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
            // AssignOperation::ShrSigned => ">>>=",
            AssignOperation::Shl => "<<=",
            // AssignOperation::Mod => "%=",
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
    pub place: Expression,
    /// The value to assign to the `assignee`.
    pub value: Expression,
    /// The span, excluding the semicolon.
    pub span: Span,
}

impl fmt::Display for AssignStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {};", self.place, self.operation.as_ref(), self.value)
    }
}

crate::simple_node_impl!(AssignStatement);
