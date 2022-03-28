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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Data types
    /// The `address` type.
    Address,
    /// The `bool` type.
    Boolean,
    /// The `char` type.
    Char,
    /// The `field` type.
    Field,
    /// The `group` type.
    Group,
    /// An integer type.
    IntegerType(IntegerType),

    /// A tuple type `(T_0, T_1, ...)` made up of a list of types.
    Tuple(Vec<Type>),

    /// A reference to either a nominal type (e.g., a `circuit`) or a type alias.
    Identifier(Identifier),

    /// Placeholder for a type that could not be resolved or was not well-formed.
    /// Will eventually lead to a compile error.
    Err,
}

impl Type {
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
            (Type::Address, Type::Address) => true,
            (Type::Boolean, Type::Boolean) => true,
            (Type::Char, Type::Char) => true,
            (Type::Field, Type::Field) => true,
            (Type::Group, Type::Group) => true,
            (Type::IntegerType(left), Type::IntegerType(right)) => left.eq(right),
            (Type::Identifier(left), Type::Identifier(right)) => left.eq(right),
            (Type::Tuple(left), Type::Tuple(right)) => left
                .iter()
                .zip(right)
                .all(|(left_type, right_type)| left_type.eq_flat(right_type)),
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::IntegerType(ref integer_type) => write!(f, "{}", integer_type),
            Type::Identifier(ref variable) => write!(f, "circuit {}", variable),
            Type::Tuple(ref tuple) => {
                let types = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "({})", types)
            }
            Type::Err => write!(f, "error"),
        }
    }
}
