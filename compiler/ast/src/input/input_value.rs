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
// TODO: Add other input types.
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Field(String),
    Group(GroupLiteral),
    Scalar(String),
    String(String),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
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
                (Type::Scalar, Literal::Scalar(value, _)) => Self::Scalar(value),
                (Type::String, Literal::String(value, _)) => Self::String(value),
                (Type::I8, Literal::I8(value, span)) => InputValue::I8(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "i8", span))?,
                ),
                (Type::I16, Literal::I16(value, span)) => InputValue::I16(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "i16", span))?,
                ),
                (Type::I32, Literal::I32(value, span)) => InputValue::I32(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "i32", span))?,
                ),
                (Type::I64, Literal::I64(value, span)) => InputValue::I64(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "i64", span))?,
                ),
                (Type::I128, Literal::I128(value, span)) => InputValue::I128(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "i128", span))?,
                ),
                (Type::U8, Literal::U8(value, span)) => InputValue::U8(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "u8", span))?,
                ),
                (Type::U16, Literal::U16(value, span)) => InputValue::U16(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "u16", span))?,
                ),
                (Type::U32, Literal::U32(value, span)) => InputValue::U32(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "u32", span))?,
                ),
                (Type::U64, Literal::U64(value, span)) => InputValue::U64(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "u64", span))?,
                ),
                (Type::U128, Literal::U128(value, span)) => InputValue::U128(
                    value
                        .parse()
                        .map_err(|_| InputError::invalid_integer_value(value, "u128", span))?,
                ),
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

impl From<&InputValue> for Type {
    fn from(input_value: &InputValue) -> Self {
        match input_value {
            InputValue::Address(_) => Type::Address,
            InputValue::Boolean(_) => Type::Boolean,
            InputValue::Field(_) => Type::Field,
            InputValue::Group(_) => Type::Group,
            InputValue::Scalar(_) => Type::Scalar,
            InputValue::String(_) => Type::String,
            InputValue::I8(_) => Type::I8,
            InputValue::I16(_) => Type::I16,
            InputValue::I32(_) => Type::I32,
            InputValue::I64(_) => Type::I64,
            InputValue::I128(_) => Type::I128,
            InputValue::U8(_) => Type::U8,
            InputValue::U16(_) => Type::U16,
            InputValue::U32(_) => Type::U32,
            InputValue::U64(_) => Type::U64,
            InputValue::U128(_) => Type::U128,
        }
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputValue::Address(ref address) => write!(f, "{}", address),
            InputValue::Boolean(ref boolean) => write!(f, "{}", boolean),
            InputValue::Group(ref group) => write!(f, "{}", group),
            InputValue::Field(ref field) => write!(f, "{}", field),
            InputValue::Scalar(ref scalar) => write!(f, "{}", scalar),
            InputValue::String(ref string) => write!(f, "{}", string),
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
