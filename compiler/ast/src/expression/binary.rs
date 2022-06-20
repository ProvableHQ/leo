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
use leo_span::{sym, Symbol};

/// A binary operator.
///
/// Precedence is defined in the parser.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperation {
    /// Addition, i.e. `+`, `.add()`.
    Add,
    /// Wrapping addition, i.e. `.add_wrapped()`.
    AddWrapped,
    /// Logical AND, i.e. `&&`.
    And,
    /// Bitwise AND, i.e. `&`, `.and()`.
    BitwiseAnd,
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
    /// Logical OR, i.e. `||`.
    Or,
    /// Bitwise OR, i.e. `|`, `.or()`.
    BitwiseOr,
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
    pub fn from_symbol(symbol: Symbol) -> Option<Self> {
        Some(match symbol {
            sym::add => Self::Add,
            sym::add_wrapped => Self::AddWrapped,
            sym::and => Self::BitwiseAnd,
            sym::div => Self::Div,
            sym::div_wrapped => Self::DivWrapped,
            sym::eq => Self::Eq,
            sym::ge => Self::Ge,
            sym::gt => Self::Gt,
            sym::le => Self::Le,
            sym::lt => Self::Lt,
            sym::mul => Self::Mul,
            sym::mul_wrapped => Self::MulWrapped,
            sym::nand => Self::Nand,
            sym::neq => Self::Neq,
            sym::nor => Self::Nor,
            sym::or => Self::BitwiseOr,
            sym::pow => Self::Pow,
            sym::pow_wrapped => Self::PowWrapped,
            sym::shl => Self::Shl,
            sym::shl_wrapped => Self::ShlWrapped,
            sym::shr => Self::Shr,
            sym::shr_wrapped => Self::ShrWrapped,
            sym::sub => Self::Sub,
            sym::sub_wrapped => Self::SubWrapped,
            sym::xor => Self::Xor,
            _ => return None,
        })
    }

    /// Represents the operator as a string.
    fn as_str(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::AddWrapped => "add_wrapped",
            Self::And => "&&",
            Self::BitwiseAnd => "and",
            Self::Div => "div",
            Self::DivWrapped => "div_wrapped",
            Self::Eq => "eq",
            Self::Ge => "ge",
            Self::Gt => "gt",
            Self::Le => "le",
            Self::Lt => "lt",
            Self::Mul => "mul",
            Self::MulWrapped => "mul_wrapped",
            Self::Nand => "nand",
            Self::Neq => "neq",
            Self::Nor => "nor",
            Self::Or => "||",
            Self::BitwiseOr => "or",
            Self::Pow => "pow",
            Self::PowWrapped => "pow_wrapped",
            Self::Shl => "shl",
            Self::ShlWrapped => "shl_wrapped",
            Self::Shr => "shr",
            Self::ShrWrapped => "shr_wrapped",
            Self::Sub => "sub",
            Self::SubWrapped => "sub_wrapped",
            Self::Xor => "xor",
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
        write!(f, "{}.{}({})", self.left, self.op.as_str(), self.right)
    }
}

crate::simple_node_impl!(BinaryExpression);
