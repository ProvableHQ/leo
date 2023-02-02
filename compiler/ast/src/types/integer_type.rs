// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use leo_span::{sym, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Explicit integer type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Is the integer type a signed one?
    pub fn is_signed(&self) -> bool {
        use IntegerType::*;
        matches!(self, I8 | I16 | I32 | I64 | I128)
    }

    /// Returns the symbol for the integer type.
    pub fn symbol(self) -> Symbol {
        match self {
            Self::I8 => sym::i8,
            Self::I16 => sym::i16,
            Self::I32 => sym::i32,
            Self::I64 => sym::i64,
            Self::I128 => sym::i128,
            Self::U8 => sym::u8,
            Self::U16 => sym::u16,
            Self::U32 => sym::u32,
            Self::U64 => sym::u64,
            Self::U128 => sym::u128,
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
