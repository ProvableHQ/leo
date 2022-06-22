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
use leo_errors::{type_name, FlattenError, LeoError, Result, TypeCheckerError};
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
// (Field(types), [Field, u8](rhs), Method, StrOp, Field(output_type), (|| -> Err))
macro_rules! implement_const_op {
    (
        name: $name:ident,
        patterns: [$([
            types: $l:ident, [$($r:ident),+], $out:ident,
            logic: $logic:expr
        ]),+]
    ) => {
        pub(crate) fn $name(self, other: Self, span: Span) -> Result<Self> {
            use Value::*;

            match (self, other) {
                $(
                    $(
                        ($l(types, _), $r(rhs, _)) => {
                            let rhs_type = type_name(&rhs);
                            let out = $logic(types, rhs.into(), rhs_type, span)?;
                            Ok($out(out, span))
                        },
                    )+
                )+
                _ => unreachable!("")
            }
        }
    };
}

impl Value {
    pub(crate) fn is_int_type(&self) -> bool {
        use Value::*;
        matches!(
            self,
            I8(_, _)
                | I16(_, _)
                | I32(_, _)
                | I64(_, _)
                | I128(_, _)
                | U8(_, _)
                | U16(_, _)
                | U32(_, _)
                | U64(_, _)
                | U128(_, _)
        )
    }

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

    implement_const_op! {
        name: add,
        patterns: [
            [
                types: I8, [I8], I8,
                logic: |l: i8, r: i8, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: I16, [I16], I16,
                logic: |l: i16, r: i16, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: I32, [I32], I32,
                logic: |l: i32, r: i32, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: I64, [I64], I64,
                logic: |l: i64, r: i64, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: I128, [I128], I128,
                logic: |l: i128, r: i128, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: U8, [U8], U8,
                logic: |l: u8, r: u8, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: U16, [U16], U16,
                logic: |l: u16, r: u16, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: U32, [U32], U32,
                logic: |l: u32, r: u32, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: U64, [U64], U64,
                logic: |l: u64, r: u64, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ],
            [
                types: U128, [U128], U128,
                logic: |l: u128, r: u128, t, span| l.checked_add(r).ok_or_else(|| FlattenError::operation_overflow(l, '+', r, t, span))
            ]
        ]
    }

    implement_const_op! {
        name: eq,
        patterns: [
            [
            types: I8, [I8], Boolean,
            logic: |l: i8, r: i8, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: I16, [I16], Boolean,
                logic: |l: i16, r: i16, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: I32, [I32], Boolean,
                logic: |l: i32, r: i32, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: I64, [I64], Boolean,
                logic: |l: i64, r: i64, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: I128, [I128], Boolean,
                logic: |l: i128, r: i128, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: U8, [U8], Boolean,
                logic: |l: u8, r: u8, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: U16, [U16], Boolean,
                logic: |l: u16, r: u16, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: U32, [U32], Boolean,
                logic: |l: u32, r: u32, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: U64, [U64], Boolean,
                logic: |l: u64, r: u64, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ],
            [
                types: U128, [U128], Boolean,
                logic: |l: u128, r: u128, _, _| -> Result<bool> {Ok(l.eq(&r))}
            ]
        ]
    }

    implement_const_op! {
        name: pow,
        patterns: [
            [
                types: I8, [U8, U16, U32], I8,
                logic: |l: i8, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: I16, [U8, U16, U32], I16,
                logic: |l: i16, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: I32, [U8, U16, U32], I32,
                logic: |l: i32, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: I64, [U8, U16, U32], I64,
                logic: |l: i64, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: I128, [U8, U16, U32], I128,
                logic: |l: i128, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: U8, [U8, U16, U32], U8,
                logic: |l: u8, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: U16, [U8, U16, U32], U16,
                logic: |l: u16, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: U32, [U8, U16, U32], U32,
                logic: |l: u32, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: U64, [U8, U16, U32], U64,
                logic: |l: u64, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ],
            [
                types: U128, [U8, U16, U32], U128,
                logic: |l: u128, r: u32, t, span| l.checked_pow(r).ok_or_else(|| FlattenError::operation_overflow(l, "**", r, t, span))
            ]
        ]
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
