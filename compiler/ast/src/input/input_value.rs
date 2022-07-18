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

use crate::{Expression, GroupLiteral, Literal, Node, Type, UnaryOperation};
use leo_errors::{InputError, LeoError, Result};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Field(String),
    Group(GroupLiteral),
    I8(String),
    I16(String),
    I32(String),
    I64(String),
    I128(String),
    U8(String),
    U16(String),
    U32(String),
    U64(String),
    U128(String),
}

impl TryFrom<(Type, Expression)> for InputValue {
    type Error = LeoError;
    fn try_from(value: (Type, Expression)) -> Result<Self> {
        Ok(match value {
            (type_, Expression::Literal(lit)) => match (type_, lit) {
                (Type::Address, Literal::Address(value, _)) => Self::Address(value),
                (Type::Boolean, Literal::Boolean(value, _)) => Self::Boolean(value),
                (Type::Field, Literal::Field(value, _)) => Self::Field(value),
                (Type::Group, Literal::Group(value)) => Self::Group(*value),
                (Type::I8, Literal::I8(value, _)) => Self::I8(value),
                (Type::I16, Literal::I16(value, _)) => Self::I16(value),
                (Type::I32, Literal::I32(value, _)) => Self::I32(value),
                (Type::I64, Literal::I64(value, _)) => Self::I64(value),
                (Type::I128, Literal::I128(value, _)) => Self::I128(value),
                (Type::U8, Literal::U8(value, _)) => Self::U8(value),
                (Type::U16, Literal::U16(value, _)) => Self::U16(value),
                (Type::U32, Literal::U32(value, _)) => Self::U32(value),
                (Type::U64, Literal::U64(value, _)) => Self::U64(value),
                (Type::U128, Literal::U128(value, _)) => Self::U128(value),
                (x, y) => {
                    return Err(InputError::unexpected_type(x, &y, y.span()).into());
                }
            },
            (type_, Expression::Unary(unary)) if unary.op == UnaryOperation::Negate => {
                InputValue::try_from((type_, *unary.receiver))?
            }
            (_type_, expr) => return Err(InputError::illegal_expression(&expr, expr.span()).into()),
        })
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Address(ref address) => write!(f, "{}", address),
            InputValue::Boolean(ref boolean) => write!(f, "{}", boolean),
            InputValue::Group(ref group) => write!(f, "{}", group),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::I8(ref integer) => write!(f, "{}", integer),
            InputValue::I16(ref integer) => write!(f, "{}", integer),
            InputValue::I32(ref integer) => write!(f, "{}", integer),
            InputValue::I64(ref integer) => write!(f, "{}", integer),
            InputValue::I128(ref integer) => write!(f, "{}", integer),
            InputValue::U8(ref integer) => write!(f, "{}", integer),
            InputValue::U16(ref integer) => write!(f, "{}", integer),
            InputValue::U32(ref integer) => write!(f, "{}", integer),
            InputValue::U64(ref integer) => write!(f, "{}", integer),
            InputValue::U128(ref integer) => write!(f, "{}", integer),
        }
    }
}
