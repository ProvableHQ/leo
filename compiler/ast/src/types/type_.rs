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

use crate::{Identifier, Tuple};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Data types
    /// The `address` type.
    Address,
    /// The `bool` type.
    Boolean,
    /// The `field` type.
    Field,
    /// The `group` type.
    Group,
    /// The 8-bit signed integer type.
    I8,
    /// The 16-bit signed integer type.
    I16,
    /// The 32-bit signed integer type.
    I32,
    /// The 64-bit signed integer type.
    I64,
    /// The 128-bit signed integer type.
    I128,
    /// A reference to a built in type.
    Identifier(Identifier),
    /// The `scalar` type.
    Scalar,
    /// The `string` type.
    String,
    /// The 8-bit unsigned integer type.
    U8,
    /// The 16-bit unsigned integer type.
    U16,
    /// The 32-bit unsigned integer type.
    U32,
    /// The 64-bit unsigned integer type.
    U64,
    /// The 128-bit unsigned integer type.
    U128,
    /// A static tuple of at least one type.
    Tuple(Tuple),

    /// Placeholder for a type that could not be resolved or was not well-formed.
    /// Will eventually lead to a compile error.
    Err,
}

impl Type {
    ///
    /// Returns `true` if the self `Type` is equal to the other `Type`.
    ///
    /// Flattens array syntax: `[[u8; 1]; 2] == [u8; (2, 1)] == true`
    ///
    pub fn eq_flat(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Address, Type::Address)
            | (Type::Boolean, Type::Boolean)
            | (Type::Field, Type::Field)
            | (Type::Group, Type::Group)
            | (Type::I8, Type::I8)
            | (Type::I16, Type::I16)
            | (Type::I32, Type::I32)
            | (Type::I64, Type::I64)
            | (Type::I128, Type::I128)
            | (Type::Scalar, Type::Scalar)
            | (Type::String, Type::String)
            | (Type::U8, Type::U8)
            | (Type::U16, Type::U16)
            | (Type::U32, Type::U32)
            | (Type::U64, Type::U64)
            | (Type::U128, Type::U128) => true,
            (Type::Tuple(left), Type::Tuple(right)) => left
                .iter()
                .zip(right.iter())
                .all(|(left_type, right_type)| left_type.eq_flat(right_type)),
            (Type::Identifier(left), Type::Identifier(right)) => left.matches(right),
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "boolean"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::I128 => write!(f, "i128"),
            Type::Identifier(ref variable) => write!(f, "circuit {}", variable),
            Type::Scalar => write!(f, "scalar"),
            Type::String => write!(f, "string"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::U128 => write!(f, "u128"),
            Type::Tuple(ref tuple) => write!(f, "{}", tuple),
            Type::Err => write!(f, "error"),
        }
    }
}
