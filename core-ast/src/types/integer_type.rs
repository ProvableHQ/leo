// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use leo_grammar::types::{
    IntegerType as AstIntegerType,
    SignedIntegerType as AstSignedIntegerType,
    UnsignedIntegerType as AstUnsignedIntegerType,
};
use leo_input::types::{
    IntegerType as InputAstIntegerType,
    SignedIntegerType as InputAstSignedIntegerType,
    UnsignedIntegerType as InputAstUnsignedIntegerType,
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

impl From<AstIntegerType> for IntegerType {
    fn from(integer_type: AstIntegerType) -> Self {
        match integer_type {
            AstIntegerType::Signed(signed) => Self::from(signed),
            AstIntegerType::Unsigned(unsigned) => Self::from(unsigned),
        }
    }
}

impl From<AstUnsignedIntegerType> for IntegerType {
    fn from(integer_type: AstUnsignedIntegerType) -> Self {
        match integer_type {
            AstUnsignedIntegerType::U8Type(_type) => IntegerType::U8,
            AstUnsignedIntegerType::U16Type(_type) => IntegerType::U16,
            AstUnsignedIntegerType::U32Type(_type) => IntegerType::U32,
            AstUnsignedIntegerType::U64Type(_type) => IntegerType::U64,
            AstUnsignedIntegerType::U128Type(_type) => IntegerType::U128,
        }
    }
}

impl From<AstSignedIntegerType> for IntegerType {
    fn from(integer_type: AstSignedIntegerType) -> Self {
        match integer_type {
            AstSignedIntegerType::I8Type(_type) => IntegerType::I8,
            AstSignedIntegerType::I16Type(_type) => IntegerType::I16,
            AstSignedIntegerType::I32Type(_type) => IntegerType::I32,
            AstSignedIntegerType::I64Type(_type) => IntegerType::I64,
            AstSignedIntegerType::I128Type(_type) => IntegerType::I128,
        }
    }
}

impl From<InputAstIntegerType> for IntegerType {
    fn from(integer_type: InputAstIntegerType) -> Self {
        match integer_type {
            InputAstIntegerType::Signed(signed) => Self::from(signed),
            InputAstIntegerType::Unsigned(unsigned) => Self::from(unsigned),
        }
    }
}

impl From<InputAstUnsignedIntegerType> for IntegerType {
    fn from(integer_type: InputAstUnsignedIntegerType) -> Self {
        match integer_type {
            InputAstUnsignedIntegerType::U8Type(_type) => IntegerType::U8,
            InputAstUnsignedIntegerType::U16Type(_type) => IntegerType::U16,
            InputAstUnsignedIntegerType::U32Type(_type) => IntegerType::U32,
            InputAstUnsignedIntegerType::U64Type(_type) => IntegerType::U64,
            InputAstUnsignedIntegerType::U128Type(_type) => IntegerType::U128,
        }
    }
}

impl From<InputAstSignedIntegerType> for IntegerType {
    fn from(integer_type: InputAstSignedIntegerType) -> Self {
        match integer_type {
            InputAstSignedIntegerType::I8Type(_type) => IntegerType::I8,
            InputAstSignedIntegerType::I16Type(_type) => IntegerType::I16,
            InputAstSignedIntegerType::I32Type(_type) => IntegerType::I32,
            InputAstSignedIntegerType::I64Type(_type) => IntegerType::I64,
            InputAstSignedIntegerType::I128Type(_type) => IntegerType::I128,
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
