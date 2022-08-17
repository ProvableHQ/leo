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

use crate::{Identifier, IntegerType, Tuple};

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
    /// A reference to a built in type.
    Identifier(Identifier),
    /// An integer type.
    Integer(IntegerType),
    /// The `scalar` type.
    Scalar,
    /// The `string` type.
    String,
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
            | (Type::Scalar, Type::Scalar)
            | (Type::String, Type::String) => true,
            (Type::Integer(left), Type::Integer(right)) => left.eq(right),
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
            Type::Identifier(ref variable) => write!(f, "{}", variable),
            Type::Integer(ref integer_type) => write!(f, "{}", integer_type),
            Type::Scalar => write!(f, "scalar"),
            Type::String => write!(f, "string"),
            Type::Tuple(ref tuple) => write!(f, "{}", tuple),
            Type::Err => write!(f, "error"),
        }
    }
}
