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

use leo_input::types::{
    IntegerType as InputIntegerType,
    SignedIntegerType as InputSignedIntegerType,
    UnsignedIntegerType as InputUnsignedIntegerType,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Explicit integer type
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,

    I8,
    I16,
    I32,
    I64,
    I128,
}

impl IntegerType {
    pub fn is_signed(&self) -> bool {
        use IntegerType::*;
        match self {
            I8 | I16 | I32 | I64 | I128 => true,
            _ => false,
        }
    }
}

impl From<InputIntegerType> for IntegerType {
    fn from(integer_type: InputIntegerType) -> Self {
        match integer_type {
            InputIntegerType::Signed(signed) => Self::from(signed),
            InputIntegerType::Unsigned(unsigned) => Self::from(unsigned),
        }
    }
}

impl From<InputUnsignedIntegerType> for IntegerType {
    fn from(integer_type: InputUnsignedIntegerType) -> Self {
        match integer_type {
            InputUnsignedIntegerType::U8Type(_type) => IntegerType::U8,
            InputUnsignedIntegerType::U16Type(_type) => IntegerType::U16,
            InputUnsignedIntegerType::U32Type(_type) => IntegerType::U32,
            InputUnsignedIntegerType::U64Type(_type) => IntegerType::U64,
            InputUnsignedIntegerType::U128Type(_type) => IntegerType::U128,
        }
    }
}

impl From<InputSignedIntegerType> for IntegerType {
    fn from(integer_type: InputSignedIntegerType) -> Self {
        match integer_type {
            InputSignedIntegerType::I8Type(_type) => IntegerType::I8,
            InputSignedIntegerType::I16Type(_type) => IntegerType::I16,
            InputSignedIntegerType::I32Type(_type) => IntegerType::I32,
            InputSignedIntegerType::I64Type(_type) => IntegerType::I64,
            InputSignedIntegerType::I128Type(_type) => IntegerType::I128,
        }
    }
}

impl fmt::Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntegerType::U8 => write!(f, "u8"),
            IntegerType::U16 => write!(f, "u16"),
            IntegerType::U32 => write!(f, "u32"),
            IntegerType::U64 => write!(f, "u64"),
            IntegerType::U128 => write!(f, "u128"),

            IntegerType::I8 => write!(f, "i8"),
            IntegerType::I16 => write!(f, "i16"),
            IntegerType::I32 => write!(f, "i32"),
            IntegerType::I64 => write!(f, "i64"),
            IntegerType::I128 => write!(f, "i128"),
        }
    }
}
