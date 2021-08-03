// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{Circuit, Identifier, IntegerType, Type};
use leo_errors::{AsgError, LeoError, Span};

use indexmap::IndexMap;
use num_bigint::BigInt;
use std::{convert::TryInto, fmt};
use tendril::StrTendril;

/// Constant integer values in a program.
#[derive(Clone, Debug, PartialEq)]
pub enum ConstInt {
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

/// Specifies how to calculate a group coordinate in a program.
#[derive(Clone, Debug, PartialEq)]
pub enum GroupCoordinate {
    /// Explicit field element number string.
    Number(StrTendril),

    /// Attempt to recover with a sign high bit.
    SignHigh,

    /// Attempt to recover with a sign low bit.
    SignLow,

    /// Try recovering with a sign low - upon failure try sign high.
    Inferred,
}

impl fmt::Display for GroupCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupCoordinate::Number(number) => write!(f, "{}", number),
            GroupCoordinate::SignHigh => write!(f, "+"),
            GroupCoordinate::SignLow => write!(f, "-"),
            GroupCoordinate::Inferred => write!(f, "_"),
        }
    }
}

impl From<&leo_ast::GroupCoordinate> for GroupCoordinate {
    fn from(other: &leo_ast::GroupCoordinate) -> GroupCoordinate {
        use leo_ast::GroupCoordinate::*;
        match other {
            Number(value, _) => GroupCoordinate::Number(value.clone()),
            SignHigh => GroupCoordinate::SignHigh,
            SignLow => GroupCoordinate::SignLow,
            Inferred => GroupCoordinate::Inferred,
        }
    }
}

impl Into<leo_ast::GroupCoordinate> for &GroupCoordinate {
    fn into(self) -> leo_ast::GroupCoordinate {
        use GroupCoordinate::*;
        match self {
            Number(value) => leo_ast::GroupCoordinate::Number(value.clone(), Default::default()),
            SignHigh => leo_ast::GroupCoordinate::SignHigh,
            SignLow => leo_ast::GroupCoordinate::SignLow,
            Inferred => leo_ast::GroupCoordinate::Inferred,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroupValue {
    Single(StrTendril),
    Tuple(GroupCoordinate, GroupCoordinate),
}

impl From<leo_ast::GroupValue> for GroupValue {
    fn from(other: leo_ast::GroupValue) -> Self {
        use leo_ast::GroupValue::*;
        match other {
            Single(value, _) => GroupValue::Single(value),
            Tuple(value) => GroupValue::Tuple(GroupCoordinate::from(&value.x), GroupCoordinate::from(&value.y)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharValue {
    Scalar(char),
    NonScalar(u32),
}

impl From<&leo_ast::Char> for CharValue {
    fn from(other: &leo_ast::Char) -> Self {
        use leo_ast::Char::*;
        match other {
            Scalar(value) => CharValue::Scalar(*value),
            NonScalar(value) => CharValue::NonScalar(*value),
        }
    }
}

impl Into<leo_ast::Char> for &CharValue {
    fn into(self) -> leo_ast::Char {
        use leo_ast::Char::*;
        match self {
            CharValue::Scalar(value) => Scalar(*value),
            CharValue::NonScalar(value) => NonScalar(*value),
        }
    }
}

impl From<leo_ast::CharValue> for CharValue {
    fn from(other: leo_ast::CharValue) -> Self {
        Self::from(&other.character)
    }
}

#[derive(Clone, PartialEq)]
pub enum ConstValue<'a> {
    Int(ConstInt),
    Group(GroupValue),
    Field(BigInt),
    Address(StrTendril),
    Boolean(bool),
    Char(CharValue),

    // compounds
    Tuple(Vec<ConstValue<'a>>),
    Array(Vec<ConstValue<'a>>),
    Circuit(&'a Circuit<'a>, IndexMap<String, (Identifier, ConstValue<'a>)>),
}

macro_rules! const_int_op {
    ($name: ident, $retType: ty, $x: ident, $transform: expr) => {
        pub fn $name(&self) -> $retType {
            match self {
                ConstInt::I8($x) => $transform,
                ConstInt::I16($x) => $transform,
                ConstInt::I32($x) => $transform,
                ConstInt::I64($x) => $transform,
                ConstInt::I128($x) => $transform,
                ConstInt::U8($x) => $transform,
                ConstInt::U16($x) => $transform,
                ConstInt::U32($x) => $transform,
                ConstInt::U64($x) => $transform,
                ConstInt::U128($x) => $transform,
            }
        }
    };
}

macro_rules! const_int_biop {
    ($name: ident, $retType: ty, $x: ident, $y: ident, $transform: expr) => {
        pub fn $name(&self, other: &ConstInt) -> Option<$retType> {
            match (self, other) {
                (ConstInt::I8($x), ConstInt::I8($y)) => $transform,
                (ConstInt::I16($x), ConstInt::I16($y)) => $transform,
                (ConstInt::I32($x), ConstInt::I32($y)) => $transform,
                (ConstInt::I64($x), ConstInt::I64($y)) => $transform,
                (ConstInt::I128($x), ConstInt::I128($y)) => $transform,
                (ConstInt::U8($x), ConstInt::U8($y)) => $transform,
                (ConstInt::U16($x), ConstInt::U16($y)) => $transform,
                (ConstInt::U32($x), ConstInt::U32($y)) => $transform,
                (ConstInt::U64($x), ConstInt::U64($y)) => $transform,
                (ConstInt::U128($x), ConstInt::U128($y)) => $transform,
                _ => None,
            }
        }
    };
}

macro_rules! const_int_map {
    ($name: ident, $x: ident, $transform: expr) => {
        pub fn $name(&self) -> Option<ConstInt> {
            Some(match self {
                ConstInt::I8($x) => ConstInt::I8($transform),
                ConstInt::I16($x) => ConstInt::I16($transform),
                ConstInt::I32($x) => ConstInt::I32($transform),
                ConstInt::I64($x) => ConstInt::I64($transform),
                ConstInt::I128($x) => ConstInt::I128($transform),
                ConstInt::U8($x) => ConstInt::U8($transform),
                ConstInt::U16($x) => ConstInt::U16($transform),
                ConstInt::U32($x) => ConstInt::U32($transform),
                ConstInt::U64($x) => ConstInt::U64($transform),
                ConstInt::U128($x) => ConstInt::U128($transform),
            })
        }
    };
}

macro_rules! const_int_bimap {
    ($name: ident, $x: ident, $y: ident, $transform: expr) => {
        pub fn $name(&self, other: &ConstInt) -> Option<ConstInt> {
            Some(match (self, other) {
                (ConstInt::I8($x), ConstInt::I8($y)) => ConstInt::I8($transform),
                (ConstInt::I16($x), ConstInt::I16($y)) => ConstInt::I16($transform),
                (ConstInt::I32($x), ConstInt::I32($y)) => ConstInt::I32($transform),
                (ConstInt::I64($x), ConstInt::I64($y)) => ConstInt::I64($transform),
                (ConstInt::I128($x), ConstInt::I128($y)) => ConstInt::I128($transform),
                (ConstInt::U8($x), ConstInt::U8($y)) => ConstInt::U8($transform),
                (ConstInt::U16($x), ConstInt::U16($y)) => ConstInt::U16($transform),
                (ConstInt::U32($x), ConstInt::U32($y)) => ConstInt::U32($transform),
                (ConstInt::U64($x), ConstInt::U64($y)) => ConstInt::U64($transform),
                (ConstInt::U128($x), ConstInt::U128($y)) => ConstInt::U128($transform),
                _ => return None,
            })
        }
    };
}

#[allow(clippy::useless_conversion)]
impl ConstInt {
    const_int_op!(raw_value, String, x, format!("{}", x));

    const_int_map!(value_negate, x, x.checked_neg()?);

    const_int_map!(value_bit_negate, x, !x);

    const_int_op!(to_usize, Option<usize>, x, (*x).try_into().ok());

    const_int_op!(to_u128, u128, x, *x as u128);

    const_int_op!(to_u64, u64, x, *x as u64);

    const_int_op!(to_u32, u32, x, *x as u32);

    const_int_op!(to_u16, u16, x, *x as u16);

    const_int_op!(to_u8, u8, x, *x as u8);

    const_int_op!(to_i128, i128, x, *x as i128);

    const_int_op!(to_i64, i64, x, *x as i64);

    const_int_op!(to_i32, i32, x, *x as i32);

    const_int_op!(to_i16, i16, x, *x as i16);

    const_int_op!(to_i8, i8, x, *x as i8);

    const_int_op!(to_string, String, x, (*x).to_string());

    const_int_bimap!(value_add, x, y, x.checked_add(*y)?);

    const_int_bimap!(value_sub, x, y, x.checked_sub(*y)?);

    const_int_bimap!(value_mul, x, y, x.checked_mul(*y)?);

    const_int_bimap!(value_div, x, y, x.checked_div(*y)?);

    // TODO: limited to 32 bit exponents
    const_int_bimap!(value_pow, x, y, x.checked_pow((*y).try_into().ok()?)?);

    const_int_biop!(value_lt, bool, x, y, Some(x < y));

    const_int_biop!(value_le, bool, x, y, Some(x <= y));

    const_int_biop!(value_gt, bool, x, y, Some(x > y));

    const_int_biop!(value_ge, bool, x, y, Some(x >= y));

    pub fn get_int_type(&self) -> IntegerType {
        match self {
            ConstInt::I8(_) => IntegerType::I8,
            ConstInt::I16(_) => IntegerType::I16,
            ConstInt::I32(_) => IntegerType::I32,
            ConstInt::I64(_) => IntegerType::I64,
            ConstInt::I128(_) => IntegerType::I128,
            ConstInt::U8(_) => IntegerType::U8,
            ConstInt::U16(_) => IntegerType::U16,
            ConstInt::U32(_) => IntegerType::U32,
            ConstInt::U64(_) => IntegerType::U64,
            ConstInt::U128(_) => IntegerType::U128,
        }
    }

    pub fn cast_to(&self, target: &IntegerType) -> ConstInt {
        match target {
            IntegerType::I8 => ConstInt::I8(self.to_i8()),
            IntegerType::I16 => ConstInt::I16(self.to_i16()),
            IntegerType::I32 => ConstInt::I32(self.to_i32()),
            IntegerType::I64 => ConstInt::I64(self.to_i64()),
            IntegerType::I128 => ConstInt::I128(self.to_i128()),
            IntegerType::U8 => ConstInt::U8(self.to_u8()),
            IntegerType::U16 => ConstInt::U16(self.to_u16()),
            IntegerType::U32 => ConstInt::U32(self.to_u32()),
            IntegerType::U64 => ConstInt::U64(self.to_u64()),
            IntegerType::U128 => ConstInt::U128(self.to_u128()),
        }
    }

    pub fn get_type<'a>(&self) -> Type<'a> {
        Type::Integer(self.get_int_type())
    }

    pub fn parse(int_type: &IntegerType, value: &str, span: &Span) -> Result<ConstInt, LeoError> {
        Ok(match int_type {
            IntegerType::I8 => ConstInt::I8(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::I16 => ConstInt::I16(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::I32 => ConstInt::I32(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::I64 => ConstInt::I64(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::I128 => ConstInt::I128(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::U8 => ConstInt::U8(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::U16 => ConstInt::U16(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::U32 => ConstInt::U32(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::U64 => ConstInt::U64(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
            IntegerType::U128 => ConstInt::U128(
                value
                    .parse()
                    .map_err(|_| LeoError::from(AsgError::invalid_int(value, span)))?,
            ),
        })
    }
}

impl<'a> ConstValue<'a> {
    pub fn get_type(&'a self) -> Option<Type<'a>> {
        Some(match self {
            ConstValue::Int(i) => i.get_type(),
            ConstValue::Group(_) => Type::Group,
            ConstValue::Field(_) => Type::Field,
            ConstValue::Address(_) => Type::Address,
            ConstValue::Boolean(_) => Type::Boolean,
            ConstValue::Char(_) => Type::Char,
            ConstValue::Tuple(sub_consts) => {
                Type::Tuple(sub_consts.iter().map(|x| x.get_type()).collect::<Option<Vec<Type>>>()?)
            }
            ConstValue::Array(values) => Type::Array(Box::new(values.get(0)?.get_type()?), values.len()),
            ConstValue::Circuit(circuit, _) => Type::Circuit(circuit),
        })
    }

    pub fn int(&self) -> Option<&ConstInt> {
        match self {
            ConstValue::Int(x) => Some(x),
            _ => None,
        }
    }

    pub fn field(&self) -> Option<&BigInt> {
        match self {
            ConstValue::Field(x) => Some(x),
            _ => None,
        }
    }
}
