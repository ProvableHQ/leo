// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{FromStrRadix as _, Identifier, IntegerType, Literal, LiteralVariant, Type};

use leo_errors::{FlattenError, LeoError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use std::{fmt::Display, num::ParseIntError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Address(String, Span),
    Boolean(bool, Span),
    Struct(Identifier, IndexMap<Symbol, Value>),
    Field(String, Span),
    Group(String, Span),
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Address(val, _) => write!(f, "{val}"),
            Struct(val, _) => write!(f, "{}", val.name),
            Boolean(val, _) => write!(f, "{val}"),
            Field(val, _) => write!(f, "{val}"),
            Group(val, _) => write!(f, "{val}"),
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

impl TryFrom<Value> for i128 {
    type Error = LeoError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value.as_ref().try_into()
    }
}

impl TryFrom<&Value> for i128 {
    type Error = LeoError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        use Value::*;
        match value {
            U8(val, _) => Ok(*val as i128),
            U16(val, _) => Ok(*val as i128),
            U32(val, _) => Ok(*val as i128),
            U64(val, _) => Ok(*val as i128),
            U128(val, span) => {
                i128::try_from(*val).map_err(|_| FlattenError::u128_to_i128(Type::from(value), *span).into())
            }
            I8(val, _) => Ok(*val as i128),
            I16(val, _) => Ok(*val as i128),
            I32(val, _) => Ok(*val as i128),
            I64(val, _) => Ok(*val as i128),
            I128(val, _) => Ok(*val),
            _ => unreachable!(),
        }
    }
}

impl TryFrom<Value> for u128 {
    type Error = LeoError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value.as_ref().try_into()
    }
}

impl TryFrom<&Value> for u128 {
    type Error = LeoError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        use Value::*;
        match value {
            I8(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I16(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I32(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I64(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            I128(val, span) => {
                u128::try_from(*val).map_err(|_| FlattenError::loop_has_neg_value(Type::from(value), *span).into())
            }
            U8(val, _) => Ok(*val as u128),
            U16(val, _) => Ok(*val as u128),
            U32(val, _) => Ok(*val as u128),
            U64(val, _) => Ok(*val as u128),
            U128(val, _) => Ok(*val),
            _ => unreachable!(),
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
            Struct(ident, _) => Type::Identifier(*ident),
            Field(_, _) => Type::Field,
            Group(_, _) => Type::Group,
            I8(_, _) => Type::Integer(IntegerType::I8),
            I16(_, _) => Type::Integer(IntegerType::I16),
            I32(_, _) => Type::Integer(IntegerType::I32),
            I64(_, _) => Type::Integer(IntegerType::I64),
            I128(_, _) => Type::Integer(IntegerType::I128),
            U8(_, _) => Type::Integer(IntegerType::U8),
            U16(_, _) => Type::Integer(IntegerType::U16),
            U32(_, _) => Type::Integer(IntegerType::U32),
            U64(_, _) => Type::Integer(IntegerType::U64),
            U128(_, _) => Type::Integer(IntegerType::U128),
            Scalar(_, _) => Type::Scalar,
            String(_, _) => Type::String,
        }
    }
}

impl TryFrom<&Literal> for Value {
    type Error = ParseIntError;

    /// Converts a literal to a value.
    fn try_from(literal: &Literal) -> Result<Self, Self::Error> {
        let span = literal.span;
        Ok(match &literal.variant {
            LiteralVariant::Address(string) => Self::Address(string.clone(), span),
            LiteralVariant::Boolean(bool) => Self::Boolean(*bool, span),
            LiteralVariant::Field(string) => Self::Field(string.clone(), span),
            LiteralVariant::Group(string) => Self::Group(string.clone(), span),
            LiteralVariant::Scalar(string) => Self::Scalar(string.clone(), span),
            LiteralVariant::String(string) => Self::String(string.clone(), span),
            LiteralVariant::Integer(integer_type, raw_string) => {
                let string = raw_string.replace('_', "");
                match integer_type {
                    IntegerType::U8 => Self::U8(u8::from_str_by_radix(&string)?, span),
                    IntegerType::U16 => Self::U16(u16::from_str_by_radix(&string)?, span),
                    IntegerType::U32 => Self::U32(u32::from_str_by_radix(&string)?, span),
                    IntegerType::U64 => Self::U64(u64::from_str_by_radix(&string)?, span),
                    IntegerType::U128 => Self::U128(u128::from_str_by_radix(&string)?, span),
                    IntegerType::I8 => Self::I8(i8::from_str_by_radix(&string)?, span),
                    IntegerType::I16 => Self::I16(i16::from_str_by_radix(&string)?, span),
                    IntegerType::I32 => Self::I32(i32::from_str_by_radix(&string)?, span),
                    IntegerType::I64 => Self::I64(i64::from_str_by_radix(&string)?, span),
                    IntegerType::I128 => Self::I128(i128::from_str_by_radix(&string)?, span),
                }
            }
        })
    }
}
