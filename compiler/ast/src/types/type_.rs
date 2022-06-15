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

use crate::{Identifier, IntegerType};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// The `scalar` type.
    Scalar,
    /// The `string` type.
    String,
    /// An integer type.
    IntegerType(IntegerType),
    /// A reference to a built in type.
    Identifier(Identifier),
    /// The `Self` type, allowed within `circuit` definitions.
    SelfType,

    /// Placeholder for a type that could not be resolved or was not well-formed.
    /// Will eventually lead to a compile error.
    Err,
}

impl Type {
    ///
    /// Returns `true` if the self `Type` is the `SelfType`.
    ///
    pub fn is_self(&self) -> bool {
        matches!(self, Type::SelfType)
    }

    ///
    /// Returns `true` if the self `Type` is a `Circuit`.
    ///
    pub fn is_circuit(&self) -> bool {
        matches!(self, Type::Identifier(_))
    }

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
            | (Type::String, Type::String)
            | (Type::SelfType, Type::SelfType) => true,
            (Type::IntegerType(left), Type::IntegerType(right)) => left.eq(right),
            (Type::Identifier(left), Type::Identifier(right)) => left.eq(right),
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "bool"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::Scalar => write!(f, "scalar"),
            Type::String => write!(f, "string"),
            Type::SelfType => write!(f, "SelfType"),
            Type::IntegerType(ref integer_type) => write!(f, "{}", integer_type),
            Type::Identifier(ref variable) => write!(f, "circuit {}", variable),
            Type::Err => write!(f, "error"),
        }
    }
}
