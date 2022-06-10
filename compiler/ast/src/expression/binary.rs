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
use leo_span::Symbol;

/// A binary operator.
///
/// Precedence is defined in the parser.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperation {
    /// Addition, i.e. `+`, `.add()`.
    Add,
    /// Wrapping addition, i.e. `.add_wrapped()`.
    AddWrapped,
    /// Bitwise AND, i.e. `&&`, `.and()`.
    And,
    /// Division, i.e. `/`, `.div()`.
    Div,
    /// Wrapping division, i.e. `.div_wrapped()`.
    DivWrapped,
    /// Equality relation, i.e. `==`, `.eq()`.
    Eq,
    /// Greater-or-equal relation, i.e. `>=`, `.ge()`.
    Ge,
    /// Greater-than relation, i.e. `>`, `.gt()`.
    Gt,
    /// Lesser-or-equal relation, i.e. `<=`, `.le()`.
    Le,
    /// Lesser-than relation, i.e. `<`, `.lt()`.
    Lt,
    /// Multiplication, i.e. `*`, `.mul()`.
    Mul,
    /// Wrapping multiplication, i.e. `.mul_wrapped()`.
    MulWrapped,
    /// Boolean NAND, i.e. `.nand()`.
    Nand,
    /// In-equality relation, i.e. `!=`, `.neq()`.
    Neq,
    /// Boolean NOR, i.e. `.nor()`.
    Nor,
    /// Logical-or, i.e. `||`.
    Or,
    /// Exponentiation, i.e. `**` in `a ** b`, `.pow()`.
    Pow,
    /// Wrapping exponentiation, i.e. `.pow_wrapped()`.
    PowWrapped,
    /// Shift left operation, i.e. `<<`, `.shl()`.
    Shl,
    /// Wrapping shift left operation, i.e. `<<`, `.shl_wrapped()`.
    ShlWrapped,
    /// Shift right operation, i.e. >>, `.shr()`.
    Shr,
    /// Wrapping shift right operation, i.e. >>, `.shr_wrapped()`.
    ShrWrapped,
    /// Subtraction, i.e. `-`, `.sub()`.
    Sub,
    /// Wrapped subtraction, i.e. `.sub_wrapped()`.
    SubWrapped,
    /// Bitwise XOR, i.e. `.xor()`.
    Xor,
}

impl BinaryOperation {
    /// Returns a `BinaryOperation` from the given `Symbol`.
    pub fn from_symbol(symbol: &Symbol) -> Option<BinaryOperation> {
        Some(match symbol.as_u32() {
            8 => BinaryOperation::Add,
            9 => BinaryOperation::AddWrapped,
            10 => BinaryOperation::And,
            11 => BinaryOperation::Div,
            12 => BinaryOperation::DivWrapped,
            13 => BinaryOperation::Eq,
            14 => BinaryOperation::Ge,
            15 => BinaryOperation::Gt,
            16 => BinaryOperation::Le,
            17 => BinaryOperation::Lt,
            18 => BinaryOperation::Mul,
            19 => BinaryOperation::MulWrapped,
            20 => BinaryOperation::Nand,
            21 => BinaryOperation::Neq,
            22 => BinaryOperation::Nor,
            23 => BinaryOperation::Or,
            24 => BinaryOperation::Pow,
            25 => BinaryOperation::PowWrapped,
            26 => BinaryOperation::Shl,
            27 => BinaryOperation::ShlWrapped,
            28 => BinaryOperation::Shr,
            29 => BinaryOperation::ShrWrapped,
            30 => BinaryOperation::Sub,
            31 => BinaryOperation::SubWrapped,
            32 => BinaryOperation::Xor,
            _ => return None,
        })
    }
}

impl AsRef<str> for BinaryOperation {
    fn as_ref(&self) -> &'static str {
        match self {
            BinaryOperation::Add => "add",
            BinaryOperation::AddWrapped => "add_wrapped",
            BinaryOperation::And => "and",
            BinaryOperation::Div => "div",
            BinaryOperation::DivWrapped => "div_wrapped",
            BinaryOperation::Eq => "eq",
            BinaryOperation::Ge => "ge",
            BinaryOperation::Gt => "gt",
            BinaryOperation::Le => "le",
            BinaryOperation::Lt => "lt",
            BinaryOperation::Mul => "mul",
            BinaryOperation::MulWrapped => "mul_wrapped",
            BinaryOperation::Nand => "nand",
            BinaryOperation::Neq => "neq",
            BinaryOperation::Nor => "nor",
            BinaryOperation::Or => "or",
            BinaryOperation::Pow => "pow",
            BinaryOperation::PowWrapped => "pow_wrapped",
            BinaryOperation::Shl => "shl",
            BinaryOperation::ShlWrapped => "shl_wrapped",
            BinaryOperation::Shr => "shr",
            BinaryOperation::ShrWrapped => "shr_wrapped",
            BinaryOperation::Sub => "sub",
            BinaryOperation::SubWrapped => "sub_wrapped",
            BinaryOperation::Xor => "xor",
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
        write!(f, "{}.{}({})", self.left, self.op.as_ref(), self.right)
    }
}

crate::simple_node_impl!(BinaryExpression);
