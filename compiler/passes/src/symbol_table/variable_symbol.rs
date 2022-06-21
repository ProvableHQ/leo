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

use leo_ast::{GroupLiteral, IntegerType, LiteralExpression, ParamMode, Type};
use leo_errors::{LeoError, Result, TypeCheckerError};
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Declaration {
    Const(Option<Value>),
    Input(ParamMode),
    Mut,
}

impl Declaration {
    pub fn get_as_usize(&self) -> Result<Option<usize>> {
        use Declaration::*;

        match self {
            Const(Some(value)) => Ok(Some(value.try_into()?)),
            Input(_) => Ok(None),
            _ => Ok(None),
        }
    }

    pub fn get_const_value(&self) -> Option<Value> {
        if let Self::Const(Some(v)) = self {
            Some(v.clone())
        } else {
            None
        }
    }

    pub fn get_type(&self) -> Option<Type> {
        use Declaration::*;

        match self {
            Const(Some(value)) => Some(value.into()),
            Input(_) => None,
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariableSymbol {
    pub type_: Type,
    pub span: Span,
    pub declaration: Declaration,
}
