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

use crate::{ArrayDimensions, Identifier, IntegerType};

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

    // Data type wrappers
    /// An array type `[element; dimensions]`.
    Array(Box<Type>, ArrayDimensions),

    /// A tuple type `(T_0, T_1, ...)` made up of a list of types.
    Tuple(Vec<Type>),

    /// A reference to either a nominal type (e.g., a `circuit`) or a type alias.
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
            (Type::Address, Type::Address) => true,
            (Type::Boolean, Type::Boolean) => true,
            (Type::Char, Type::Char) => true,
            (Type::Field, Type::Field) => true,
            (Type::Group, Type::Group) => true,
            (Type::IntegerType(left), Type::IntegerType(right)) => left.eq(right),
            (Type::Identifier(left), Type::Identifier(right)) => left.eq(right),
            (Type::SelfType, Type::SelfType) => true,
            (Type::Array(left_type, left_dims), Type::Array(right_type, right_dims)) => {
                // Convert array dimensions to owned.
                let mut left_dims = left_dims.to_owned();
                let mut right_dims = right_dims.to_owned();

                // Unable to compare arrays with unspecified sizes.
                if !left_dims.is_specified() || !right_dims.is_specified() {
                    return false;
                }

                // Remove the first element from both dimensions.
                let left_first = left_dims.remove_first();
                let right_first = right_dims.remove_first();

                // Compare the first dimensions.
                if left_first.ne(&right_first) {
                    return false;
                }

                // Create a new array type from the remaining array dimensions.
                let left_new_type = inner_array_type(*left_type.to_owned(), left_dims);
                let right_new_type = inner_array_type(*right_type.to_owned(), right_dims);

                // Call eq_flat() on the new left and right types.
                left_new_type.eq_flat(&right_new_type)
            }
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
            Type::SelfType => write!(f, "SelfType"),
            Type::Array(ref array, ref dimensions) => write!(f, "[{}; {}]", *array, dimensions),
            Type::Tuple(ref tuple) => {
                let types = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "({})", types)
            }
            Type::Err => write!(f, "error"),
        }
    }
}

/// Returns the type of the inner array given an array element and array dimensions.
///
/// If the array has no dimensions, then an inner array does not exist. Simply return the given
/// element type.
///
/// If the array has dimensions, then an inner array exists. Create a new type for the
/// inner array. The element type of the new array should be the same as the old array. The
/// dimensions of the new array should be the old array dimensions with the first dimension removed.
pub fn inner_array_type(element_type: Type, dimensions: ArrayDimensions) -> Type {
    if dimensions.is_empty() {
        // The array has one dimension.
        element_type
    } else {
        // The array has multiple dimensions.
        Type::Array(Box::new(element_type), dimensions)
    }
}
