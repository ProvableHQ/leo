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

use std::fmt::Display;

use leo_ast::{GroupLiteral, IntegerType, LiteralExpression, Type};
use leo_errors::{FlattenError, LeoError, Result, TypeCheckerError};
use leo_span::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Address(String, Span),
    Boolean(bool, Span),
    Field(String, Span),
    Group(Box<GroupLiteral>),
    I8(i8, Span),
    I16(i16, Span),
    I32(i32, Span),
    I64(i64, Span),
    I128(i128, Span),
    U8(u8, Span),
    U16(u16, Span),
    U32(u32, Span),
    U64(u64, Span),
    U128(u128, Span),
    Scalar(String, Span),
    String(String, Span),
}

impl Value {
    pub(crate) fn from_usize(type_: Type, value: usize, span: Span) -> Self {
        match type_ {
            Type::IntegerType(int_type) => match int_type {
                IntegerType::U8 => Value::U8(value as u8, span),
                IntegerType::U16 => Value::U16(value as u16, span),
                IntegerType::U32 => Value::U32(value as u32, span),
                IntegerType::U64 => Value::U64(value as u64, span),
                IntegerType::U128 => Value::U128(value as u128, span),
                IntegerType::I8 => Value::I8(value as i8, span),
                IntegerType::I16 => Value::I16(value as i16, span),
                IntegerType::I32 => Value::I32(value as i32, span),
                IntegerType::I64 => Value::I64(value as i64, span),
                IntegerType::I128 => Value::I128(value as i128, span),
            },
            _ => unreachable!(),
        }
    }

    pub(crate) fn add(self, rhs: Self, span: Span) -> Result<Self> {
        use Value::*;
        match (self, rhs) {
            (Field(_, _), Field(_, _)) => Ok(Field(todo!(), span)),
            (Group(_), Group(_)) => Ok(Group(todo!())),
            (I8(l, _), I8(r, _)) => Ok(I8(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("i8", l, '+', r, span))?,
                span,
            )),
            (I16(l, _), I16(r, _)) => Ok(I16(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("i16", l, '+', r, span))?,
                span,
            )),
            (I32(l, _), I32(r, _)) => Ok(I32(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("i32", l, '+', r, span))?,
                span,
            )),
            (I64(l, _), I64(r, _)) => Ok(I64(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("i64", l, '+', r, span))?,
                span,
            )),
            (I128(l, _), I128(r, _)) => Ok(I128(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("i128", l, '+', r, span))?,
                span,
            )),
            (U8(l, _), U8(r, _)) => Ok(U8(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("u8", l, '+', r, span))?,
                span,
            )),
            (U16(l, _), U16(r, _)) => Ok(U16(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("u16", l, '+', r, span))?,
                span,
            )),
            (U32(l, _), U32(r, _)) => Ok(U32(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("u32", l, '+', r, span))?,
                span,
            )),
            (U64(l, _), U64(r, _)) => Ok(U64(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("u64", l, '+', r, span))?,
                span,
            )),
            (U128(l, _), U128(r, _)) => Ok(U128(
                l.checked_add(r)
                    .ok_or_else(|| FlattenError::operation_overflow("u128", l, '+', r, span))?,
                span,
            )),
            (Scalar(_, _), Scalar(_, _)) => Ok(Scalar(todo!(), span)),
            _ => unreachable!(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Address(val, _) => write!(f, "{val}"),
            Boolean(val, _) => write!(f, "{val}"),
            Field(val, _) => write!(f, "{val}"),
            Group(val) => write!(f, "{val}"),
            I8(val, _) => write!(f, "{val}"),
            I16(val, _) => write!(f, "{val}"),
            I32(val, _) => write!(f, "{val}"),
            I64(val, _) => write!(f, "{val}"),
            I128(val, _) => write!(f, "{val}"),
            U8(val, _) => write!(f, "{val}"),
            U16(val, _) => write!(f, "{val}"),
            U32(val, _) => write!(f, "{val}"),
            U64(val, _) => write!(f, "{val}"),
            U128(val, _) => write!(f, "{val}"),
            Scalar(val, _) => write!(f, "{val}"),
            String(val, _) => write!(f, "{val}"),
        }
    }
}

impl TryFrom<Value> for usize {
    type Error = LeoError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value.as_ref().try_into()
    }
}

impl TryFrom<&Value> for usize {
    type Error = LeoError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        use Value::*;
        match value {
            I8(val, span) => {
                usize::try_from(*val).map_err(|_| TypeCheckerError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I16(val, span) => {
                usize::try_from(*val).map_err(|_| TypeCheckerError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I32(val, span) => {
                usize::try_from(*val).map_err(|_| TypeCheckerError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I64(val, span) => {
                usize::try_from(*val).map_err(|_| TypeCheckerError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I128(val, span) => {
                usize::try_from(*val).map_err(|_| TypeCheckerError::loop_has_neg_value(Type::from(value), *span).into())
            }
            U8(val, _) => Ok(*val as usize),
            U16(val, _) => Ok(*val as usize),
            U32(val, _) => Ok(*val as usize),
            U64(val, _) => Ok(*val as usize),
            U128(val, _) => Ok(*val as usize),
            Address(_, span) | Boolean(_, span) | Field(_, span) | Scalar(_, span) | String(_, span) => {
                Err(TypeCheckerError::cannot_use_type_as_loop_bound(value, *span).into())
            }
            Group(val) => return Err(TypeCheckerError::cannot_use_type_as_loop_bound(value, *val.span()).into()),
        }
    }
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<Value> for Type {
    fn from(v: Value) -> Self {
        v.as_ref().into()
    }
}

impl From<&Value> for Type {
    fn from(v: &Value) -> Self {
        use Value::*;
        match v {
            Address(_, _) => Type::Address,
            Boolean(_, _) => Type::Boolean,
            Field(_, _) => Type::Field,
            Group(_) => Type::Group,
            I8(_, _) => Type::IntegerType(IntegerType::I8),
            I16(_, _) => Type::IntegerType(IntegerType::I16),
            I32(_, _) => Type::IntegerType(IntegerType::I32),
            I64(_, _) => Type::IntegerType(IntegerType::I64),
            I128(_, _) => Type::IntegerType(IntegerType::I128),
            U8(_, _) => Type::IntegerType(IntegerType::U8),
            U16(_, _) => Type::IntegerType(IntegerType::U16),
            U32(_, _) => Type::IntegerType(IntegerType::U32),
            U64(_, _) => Type::IntegerType(IntegerType::U64),
            U128(_, _) => Type::IntegerType(IntegerType::U128),
            Scalar(_, _) => Type::Scalar,
            String(_, _) => Type::String,
        }
    }
}

impl From<Value> for LiteralExpression {
    fn from(v: Value) -> Self {
        use Value::*;
        match v {
            Address(v, span) => LiteralExpression::Address(v, span),
            Boolean(v, span) => LiteralExpression::Boolean(v, span),
            Field(v, span) => LiteralExpression::Field(v, span),
            Group(v) => LiteralExpression::Group(v),
            I8(v, span) => LiteralExpression::Integer(IntegerType::I8, v.to_string(), span),
            I16(v, span) => LiteralExpression::Integer(IntegerType::I16, v.to_string(), span),
            I32(v, span) => LiteralExpression::Integer(IntegerType::I32, v.to_string(), span),
            I64(v, span) => LiteralExpression::Integer(IntegerType::I64, v.to_string(), span),
            I128(v, span) => LiteralExpression::Integer(IntegerType::I128, v.to_string(), span),
            U8(v, span) => LiteralExpression::Integer(IntegerType::U8, v.to_string(), span),
            U16(v, span) => LiteralExpression::Integer(IntegerType::U16, v.to_string(), span),
            U32(v, span) => LiteralExpression::Integer(IntegerType::U32, v.to_string(), span),
            U64(v, span) => LiteralExpression::Integer(IntegerType::U64, v.to_string(), span),
            U128(v, span) => LiteralExpression::Integer(IntegerType::U128, v.to_string(), span),
            Scalar(v, span) => LiteralExpression::Scalar(v, span),
            String(v, span) => LiteralExpression::String(v, span),
        }
    }
}
