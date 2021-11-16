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

use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Or,
    And,
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
    BitOr,
    BitAnd,
    BitXor,
    Shr,
    ShrSigned,
    Shl,
    Mod,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperationClass {
    Numeric,
    Boolean,
}

impl AsRef<str> for BinaryOperation {
    fn as_ref(&self) -> &'static str {
        match self {
            BinaryOperation::Add => "+",
            BinaryOperation::Sub => "-",
            BinaryOperation::Mul => "*",
            BinaryOperation::Div => "/",
            BinaryOperation::Pow => "**",
            BinaryOperation::Or => "||",
            BinaryOperation::And => "&&",
            BinaryOperation::Eq => "==",
            BinaryOperation::Ne => "!=",
            BinaryOperation::Ge => ">=",
            BinaryOperation::Gt => ">",
            BinaryOperation::Le => "<=",
            BinaryOperation::Lt => "<",
            BinaryOperation::BitOr => "|",
            BinaryOperation::BitAnd => "&",
            BinaryOperation::BitXor => "^",
            BinaryOperation::Shr => ">>",
            BinaryOperation::ShrSigned => ">>>",
            BinaryOperation::Shl => "<<",
            BinaryOperation::Mod => "%",
        }
    }
}

impl BinaryOperation {
    pub fn class(&self) -> BinaryOperationClass {
        match self {
            BinaryOperation::Add
            | BinaryOperation::Sub
            | BinaryOperation::Mul
            | BinaryOperation::Div
            | BinaryOperation::BitOr
            | BinaryOperation::BitAnd
            | BinaryOperation::BitXor
            | BinaryOperation::Shr
            | BinaryOperation::ShrSigned
            | BinaryOperation::Shl
            | BinaryOperation::Mod
            | BinaryOperation::Pow => BinaryOperationClass::Numeric,
            BinaryOperation::Or
            | BinaryOperation::And
            | BinaryOperation::Eq
            | BinaryOperation::Ne
            | BinaryOperation::Ge
            | BinaryOperation::Gt
            | BinaryOperation::Le
            | BinaryOperation::Lt => BinaryOperationClass::Boolean,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: BinaryOperation,
    pub span: Span,
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op.as_ref(), self.right)
    }
}

impl Node for BinaryExpression {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
