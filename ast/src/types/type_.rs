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

use crate::{ArrayDimensions, Identifier, IntegerType};
use leo_grammar::types::{ArrayType, CircuitType, DataType, TupleType, Type as AstType};
use leo_input::types::{
    ArrayType as InputArrayType,
    DataType as InputDataType,
    TupleType as InputTupleType,
    Type as InputAstType,
};

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    IntegerType(IntegerType),

    // Data type wrappers
    Array(Box<Type>, ArrayDimensions),
    Tuple(Vec<Type>),
    Circuit(Identifier),
    SelfType,
}

impl Type {
    pub fn is_self(&self) -> bool {
        matches!(self, Type::SelfType)
    }

    pub fn is_circuit(&self) -> bool {
        matches!(self, Type::Circuit(_))
    }
}

/// pest ast -> Explicit Type for defining circuit members and function params

impl From<DataType> for Type {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Address(_type) => Type::Address,
            DataType::Boolean(_type) => Type::Boolean,
            DataType::Field(_type) => Type::Field,
            DataType::Group(_type) => Type::Group,
            DataType::Integer(_type) => Type::IntegerType(IntegerType::from(_type)),
        }
    }
}

impl<'ast> From<ArrayType<'ast>> for Type {
    fn from(array_type: ArrayType<'ast>) -> Self {
        let element_type = Box::new(Type::from(*array_type.type_));
        let dimensions = ArrayDimensions::from(array_type.dimensions);

        Type::Array(element_type, dimensions)
    }
}

impl<'ast> From<TupleType<'ast>> for Type {
    fn from(tuple_type: TupleType<'ast>) -> Self {
        let types = tuple_type.types.into_iter().map(Type::from).collect();

        Type::Tuple(types)
    }
}

impl<'ast> From<CircuitType<'ast>> for Type {
    fn from(circuit_type: CircuitType<'ast>) -> Self {
        Type::Circuit(Identifier::from(circuit_type.identifier))
    }
}

impl<'ast> From<AstType<'ast>> for Type {
    fn from(type_: AstType<'ast>) -> Self {
        match type_ {
            AstType::Basic(type_) => Type::from(type_),
            AstType::Array(type_) => Type::from(type_),
            AstType::Tuple(type_) => Type::from(type_),
            AstType::Circuit(type_) => Type::from(type_),
            AstType::SelfType(_type) => Type::SelfType,
        }
    }
}

/// input pest ast -> Explicit Type

impl From<InputDataType> for Type {
    fn from(data_type: InputDataType) -> Self {
        match data_type {
            InputDataType::Address(_type) => Type::Address,
            InputDataType::Boolean(_type) => Type::Boolean,
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

        Type::Array(element_type, dimensions)
    }
}

impl<'ast> From<InputTupleType<'ast>> for Type {
    fn from(tuple_type: InputTupleType<'ast>) -> Self {
        let types = tuple_type.types_.into_iter().map(Type::from).collect();

        Type::Tuple(types)
    }
}

impl<'ast> From<InputAstType<'ast>> for Type {
    fn from(type_: InputAstType<'ast>) -> Self {
        match type_ {
            InputAstType::Basic(type_) => Type::from(type_),
            InputAstType::Array(type_) => Type::from(type_),
            InputAstType::Tuple(type_) => Type::from(type_),
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
            Type::IntegerType(ref integer_type) => write!(f, "{}", integer_type),
            Type::Circuit(ref variable) => write!(f, "circuit {}", variable),
            Type::SelfType => write!(f, "SelfType"),
            Type::Array(ref array, ref dimensions) => write!(f, "[{}; {}]", *array, dimensions),
            Type::Tuple(ref tuple) => {
                let types = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "({})", types)
            }
        }
    }
}

/// Compares two types while flattening array types.
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Address, Type::Address) => true,
            (Type::Boolean, Type::Boolean) => true,
            (Type::Field, Type::Field) => true,
            (Type::Group, Type::Group) => true,
            (Type::IntegerType(left), Type::IntegerType(right)) => left.eq(&right),
            (Type::Circuit(left), Type::Circuit(right)) => left.eq(&right),
            (Type::SelfType, Type::SelfType) => true,
            (Type::Array(left_type, left_dim), Type::Array(right_type, right_dim)) => {
                let mut left_dim_owned = left_dim.to_owned();
                let mut right_dim_owned = right_dim.to_owned();

                println!("left_owned {}", left_dim_owned);
                println!("right_owned {}", right_dim_owned);

                let left_first = left_dim_owned.remove_first();
                let right_first = right_dim_owned.remove_first();

                if left_first.ne(&right_first) {
                    return false;
                }

                let left_new_type = inner_array_type(*left_type.to_owned(), left_dim_owned);
                let right_new_type = inner_array_type(*right_type.to_owned(), right_dim_owned);
                println!("left_new {}", left_new_type);
                println!("right_new {}", right_new_type);

                return left_new_type.eq(&right_new_type);
            }
            (Type::Tuple(left), Type::Tuple(right)) => left
                .iter()
                .zip(right)
                .all(|(left_type, right_type)| left_type.eq(right_type)),
            _ => false,
        }
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
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
        Type::Array(Box::new(element_type), dimensions)
    }
}
