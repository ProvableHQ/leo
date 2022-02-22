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

use super::*;

/// A binary operator.
///
/// Precedence is defined in the parser.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperation {
    /// Addition, i.e. `+`.
    Add,
    /// Subtraction, i.e. `-`.
    Sub,
    /// Multiplication, i.e. `*`.
    Mul,
    /// Division, i.e. `/`.
    Div,
    /// Exponentiation, i.e. `**` in `a ** b`.
    Pow,
    /// Logical-or, i.e., `||`.
    Or,
    /// Logical-and, i.e., `&&`.
    And,
    /// Equality relation, i.e., `==`.
    Eq,
    /// In-equality relation, i.e. `!=`.
    Ne,
    /// Greater-or-equal relation, i.e. `>=`.
    Ge,
    /// Greater-than relation, i.e. `>=`.
    Gt,
    /// Lesser-or-equal relation, i.e. `<=`.
    Le,
    /// Lesser-than relation, i.e. `<`.
    Lt,
    /// Bitwise-or inclusive, i.e., `|`.
    BitOr,
    /// Bitwise-and, i.e., `&`.
    BitAnd,
    /// Bitwise-or exclusive, i.e., `^`.
    BitXor,
    /// Shift-right, i.e `>>`.
    Shr,
    /// Unsigned shift-right, i.e `>>>`.
    ShrSigned,
    /// Shift-left, i.e `<<`.
    Shl,
    /// Modulus or remainder operation, i.e., `%`.
    Mod,
}

/// The category a binary operation belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperationClass {
    /// A numeric one, that is, the result is numeric.
    Numeric,
    /// A boolean one, meaning the result is of type `bool`.
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
    /// The class ("category") that the binary operation belongs to.
    /// For example, the `+` operator is numeric and `==` results in a boolean value.
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

/// A binary expression `left op right` of two operands separated by some operator.
/// For example, `foo + bar`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryExpression {
    /// The left operand of the expression.
    pub left: Box<Expression>,
    /// The right operand of the expression.
    pub right: Box<Expression>,
    /// The operand defining the meaning of the resulting binary expression.
    pub op: BinaryOperation,
    /// The span from `left` to `right`.
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
