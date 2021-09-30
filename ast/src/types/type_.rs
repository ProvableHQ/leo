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

use crate::{ArrayDimensions, Identifier, IntegerType, PositiveNumber};
use leo_input::types::{
    ArrayType as InputArrayType, DataType as InputDataType, TupleType as InputTupleType, Type as InputType,
};

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use std::fmt;

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Char,
    Field,
    Group,
    IntegerType(IntegerType),

    // Data type wrappers
    #[serde(serialize_with = "serialize_array")]
    Array(Box<Type>, Option<ArrayDimensions>),
    Tuple(Vec<Type>),
    Identifier(Identifier), // ex Circuit or Alias
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
            (Type::Array(left_type, left_dim), Type::Array(right_type, right_dim)) => {
                // Convert array dimensions to owned.
                let left_dim_owned = left_dim.to_owned();
                let right_dim_owned = right_dim.to_owned();

                // Unable to compare arrays with unspecified sizes.
                if left_dim_owned.is_none() || right_dim_owned.is_none() {
                    return false;
                }

                // We know that values are Some, safe to unwrap.
                let mut left_dim_owned = left_dim_owned.unwrap();
                let mut right_dim_owned = right_dim_owned.unwrap();

                // Remove the first element from both dimensions.
                let left_first = left_dim_owned.remove_first();
                let right_first = right_dim_owned.remove_first();

                // Compare the first dimensions.
                if left_first.ne(&right_first) {
                    return false;
                }

                // Create a new array type from the remaining array dimensions.
                let left_new_type = inner_array_type(*left_type.to_owned(), left_dim_owned);
                let right_new_type = inner_array_type(*right_type.to_owned(), right_dim_owned);

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

/// input pest ast -> Explicit Type

impl From<InputDataType> for Type {
    fn from(data_type: InputDataType) -> Self {
        match data_type {
            InputDataType::Address(_type) => Type::Address,
            InputDataType::Boolean(_type) => Type::Boolean,
            InputDataType::Char(_type) => Type::Char,
            InputDataType::Field(_type) => Type::Field,
            InputDataType::Group(_type) => Type::Group,
            InputDataType::Integer(type_) => Type::IntegerType(IntegerType::from(type_)),
        }
    }
}

impl<'ast> From<InputArrayType<'ast>> for Type {
    fn from(array_type: InputArrayType<'ast>) -> Self {
        let element_type = Box::new(Type::from(*array_type.type_));
        let dimensions = ArrayDimensions::from(array_type.dimensions);

        Type::Array(element_type, Some(dimensions))
    }
}

impl<'ast> From<InputTupleType<'ast>> for Type {
    fn from(tuple_type: InputTupleType<'ast>) -> Self {
        let types = tuple_type.types_.into_iter().map(Type::from).collect();

        Type::Tuple(types)
    }
}

impl<'ast> From<InputType<'ast>> for Type {
    fn from(type_: InputType<'ast>) -> Self {
        match type_ {
            InputType::Basic(type_) => Type::from(type_),
            InputType::Array(type_) => Type::from(type_),
            InputType::Tuple(type_) => Type::from(type_),
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
            Type::Array(ref array, ref dimensions) => {
                if let Some(dimensions) = dimensions {
                    write!(f, "[{}; {}]", *array, dimensions)
                } else {
                    write!(f, "[{}; _]", *array)
                }
            }
            Type::Tuple(ref tuple) => {
                let types = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "({})", types)
            }
            Type::Err => write!(f, "error"),
        }
    }
}

///
/// Returns the type of the inner array given an array element and array dimensions.
///
/// If the array has no dimensions, then an inner array does not exist. Simply return the given
/// element type.
///
/// If the array has dimensions, then an inner array exists. Create a new type for the
/// inner array. The element type of the new array should be the same as the old array. The
/// dimensions of the new array should be the old array dimensions with the first dimension removed.
///
pub fn inner_array_type(element_type: Type, dimensions: ArrayDimensions) -> Type {
    if dimensions.is_empty() {
        // The array has one dimension.
        element_type
    } else {
        // The array has multiple dimensions.
        Type::Array(Box::new(element_type), Some(dimensions))
    }
}

///
/// Custom Serializer for Type::Array. Handles the case when ArrayDimensions are None and turns it into
/// a Vec<PositiveNumber>, where the only element is "0".
///
fn serialize_array<S>(type_: &Type, dimensions: &Option<ArrayDimensions>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(2))?;
    seq.serialize_element(type_)?;
    // seq.serialize_element(dimensions)?;
    if let Some(dimensions) = dimensions {
        seq.serialize_element(&dimensions)?;
    } else {
        seq.serialize_element(&ArrayDimensions(vec![PositiveNumber { value: "0".into() }]))?;
    }
    seq.end()
}
