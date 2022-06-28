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

use crate::{Expression, GroupLiteral, IntegerType, LiteralExpression, Node, Type, UnaryOperation};
use leo_errors::{InputError, LeoError, Result};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputValue {
    Address(String),
    Boolean(bool),
    Field(String),
    Group(GroupLiteral),
    Integer(IntegerType, IntegerValue),
    Scalar(String),
    String(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegerValue {
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

impl fmt::Display for IntegerValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntegerValue::I8(i) => write!(f, "{i}"),
            IntegerValue::I16(i) => write!(f, "{i}"),
            IntegerValue::I32(i) => write!(f, "{i}"),
            IntegerValue::I64(i) => write!(f, "{i}"),
            IntegerValue::I128(i) => write!(f, "{i}"),
            IntegerValue::U8(i) => write!(f, "{i}"),
            IntegerValue::U16(i) => write!(f, "{i}"),
            IntegerValue::U32(i) => write!(f, "{i}"),
            IntegerValue::U64(i) => write!(f, "{i}"),
            IntegerValue::U128(i) => write!(f, "{i}"),
        }
    }
}

impl TryFrom<(Type, Expression)> for InputValue {
    type Error = LeoError;
    fn try_from(value: (Type, Expression)) -> Result<Self> {
        Ok(match value {
            (type_, Expression::Literal(lit)) => match (type_, lit) {
                (Type::Address, LiteralExpression::Address(value, _)) => Self::Address(value),
                (Type::Boolean, LiteralExpression::Boolean(value, _)) => Self::Boolean(value),
                (Type::Field, LiteralExpression::Field(value, _)) => Self::Field(value),
                (Type::Group, LiteralExpression::Group(value)) => Self::Group(*value),
                (Type::IntegerType(expected), LiteralExpression::Integer(actual, value, span)) => {
                    if expected == actual {
                        Self::Integer(
                            expected,
                            match actual {
                                IntegerType::U8 => IntegerValue::U8(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "u8", span))?,
                                ),
                                IntegerType::U16 => IntegerValue::U16(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "u16", span))?,
                                ),
                                IntegerType::U32 => IntegerValue::U32(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "u32", span))?,
                                ),
                                IntegerType::U64 => IntegerValue::U64(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "u64", span))?,
                                ),
                                IntegerType::U128 => IntegerValue::U128(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "u128", span))?,
                                ),
                                IntegerType::I8 => IntegerValue::I8(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "i8", span))?,
                                ),
                                IntegerType::I16 => IntegerValue::I16(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "i16", span))?,
                                ),
                                IntegerType::I32 => IntegerValue::I32(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "i32", span))?,
                                ),
                                IntegerType::I64 => IntegerValue::I64(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "i64", span))?,
                                ),
                                IntegerType::I128 => IntegerValue::I128(
                                    value
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value, "i128", span))?,
                                ),
                            },
                        )
                    } else {
                        return Err(InputError::unexpected_type(expected.to_string(), actual, span).into());
                    }
                }
                (Type::Scalar, LiteralExpression::Scalar(value, _)) => Self::Address(value),
                (Type::String, LiteralExpression::String(value, _)) => Self::Address(value),
                (x, y) => {
                    return Err(InputError::unexpected_type(x, &y, y.span()).into());
                }
            },
            (expected, Expression::Unary(unary)) if unary.op == UnaryOperation::Negate => {
                let mut negate = true;
                let mut unary = unary;
                while let Expression::Unary(receiver) = *unary.receiver {
                    if receiver.op == UnaryOperation::Negate {
                        negate = !negate;
                        unary = receiver;
                    } else {
                        return Err(
                            InputError::illegal_expression(Expression::Unary(receiver.clone()), receiver.span).into(),
                        );
                    }
                }
                match *unary.receiver {
                    Expression::Literal(LiteralExpression::Integer(actual, value, span))
                        if matches!(
                            expected,
                            Type::IntegerType(
                                IntegerType::I8
                                    | IntegerType::I16
                                    | IntegerType::I32
                                    | IntegerType::I64
                                    | IntegerType::I128
                            )
                        ) =>
                    {
                        let value_str = if negate { format!("-{}", value) } else { value };

                        Self::Integer(
                            actual,
                            match actual {
                                IntegerType::I8 => IntegerValue::I8(
                                    value_str
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value_str, "i8", span))?,
                                ),
                                IntegerType::I16 => IntegerValue::I16(
                                    value_str
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value_str, "i16", span))?,
                                ),
                                IntegerType::I32 => IntegerValue::I32(
                                    value_str
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value_str, "i32", span))?,
                                ),
                                IntegerType::I64 => IntegerValue::I64(
                                    value_str
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value_str, "i64", span))?,
                                ),
                                IntegerType::I128 => IntegerValue::I128(
                                    value_str
                                        .parse()
                                        .map_err(|_| InputError::illegal_integer_value(value_str, "i128", span))?,
                                ),
                                _ => unreachable!(),
                            },
                        )
                    }
                    Expression::Literal(LiteralExpression::Field(value, _)) if matches!(expected, Type::Field) => {
                        let value_str = if negate { format!("-{}", value) } else { value };
                        Self::Field(value_str)
                    }
                    Expression::Literal(LiteralExpression::Group(lit)) if matches!(expected, Type::Group) => {
                        Self::Group(*lit)
                    }
                    _ => {
                        return Err(
                            InputError::illegal_expression(Expression::Unary(unary.clone()), unary.span).into(),
                        );
                    }
                }
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
            InputValue::Integer(ref type_, ref number) => write!(f, "{}{:?}", number, type_),
            InputValue::String(ref string) => write!(f, "{}", string),
            InputValue::Scalar(ref scalar) => write!(f, "{}", scalar),
        }
    }
}
