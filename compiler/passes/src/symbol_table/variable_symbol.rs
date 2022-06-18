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

use leo_ast::{GroupValue, IntegerType, ParamMode, Type};
use leo_errors::{Result, TypeCheckerError};
use leo_span::Span;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Address(String),
    Boolean(bool),
    Field(String),
    Group(Box<GroupValue>),
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
    Scalar(String),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Address(val) => write!(f, "{val}"),
            Boolean(val) => write!(f, "{val}"),
            Field(val) => write!(f, "{val}"),
            Group(val) => write!(f, "{val}"),
            I8(val) => write!(f, "{val}"),
            I16(val) => write!(f, "{val}"),
            I32(val) => write!(f, "{val}"),
            I64(val) => write!(f, "{val}"),
            I128(val) => write!(f, "{val}"),
            U8(val) => write!(f, "{val}"),
            U16(val) => write!(f, "{val}"),
            U32(val) => write!(f, "{val}"),
            U64(val) => write!(f, "{val}"),
            U128(val) => write!(f, "{val}"),
            Scalar(val) => write!(f, "{val}"),
            String(val) => write!(f, "{val}"),
        }
    }
}

impl Value {
    pub(crate) fn get_as_usize(&self, span: Span) -> Result<usize> {
        use Value::*;
        match self {
            I8(val) => usize::try_from(*val),
            I16(val) => usize::try_from(*val),
            I32(val) => usize::try_from(*val),
            I64(val) => usize::try_from(*val),
            I128(val) => usize::try_from(*val),
            U8(val) => Ok(*val as usize),
            U16(val) => Ok(*val as usize),
            U32(val) => Ok(*val as usize),
            U64(val) => Ok(*val as usize),
            U128(val) => Ok(*val as usize),
            _ => return Err(TypeCheckerError::cannot_use_type_as_loop_bound(self, span).into()),
        }
        .map_err(|_| TypeCheckerError::loop_has_neg_value(Type::from(self), span).into())
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
            Address(_) => Type::Address,
            Boolean(_) => Type::Boolean,
            Field(_) => Type::Field,
            Group(_) => Type::Group,
            I8(_) => Type::IntegerType(IntegerType::I8),
            I16(_) => Type::IntegerType(IntegerType::I16),
            I32(_) => Type::IntegerType(IntegerType::I32),
            I64(_) => Type::IntegerType(IntegerType::I64),
            I128(_) => Type::IntegerType(IntegerType::I128),
            U8(_) => Type::IntegerType(IntegerType::U8),
            U16(_) => Type::IntegerType(IntegerType::U16),
            U32(_) => Type::IntegerType(IntegerType::U32),
            U64(_) => Type::IntegerType(IntegerType::U64),
            U128(_) => Type::IntegerType(IntegerType::U128),
            Scalar(_) => Type::Scalar,
            String(_) => Type::String,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Declaration {
    Const(Option<Value>),
    Input(ParamMode),
    Mut(Option<Value>),
}

impl Declaration {
    pub fn get_as_usize(&self, span: Span) -> Result<Option<usize>> {
        use Declaration::*;

        match self {
            Const(Some(value)) | Mut(Some(value)) => Ok(Some(value.get_as_usize(span)?)),
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
            Const(Some(value)) | Mut(Some(value)) => Some(value.into()),
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
