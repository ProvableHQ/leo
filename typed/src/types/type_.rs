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

use crate::{Expression, Identifier, IntegerType};
use leo_ast::types::{ArrayType, CircuitType, DataType, TupleType, Type as AstType};
use leo_input::types::{
    ArrayType as InputArrayType,
    DataType as InputDataType,
    TupleType as InputTupleType,
    Type as InputAstType,
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    IntegerType(IntegerType),

    // Data type wrappers
    Array(Box<Type>, Vec<usize>),
    Tuple(Vec<Type>),
    Circuit(Identifier),
    SelfType,
}

impl Type {
    pub fn is_self(&self) -> bool {
        if let Type::SelfType = self {
            return true;
        }
        false
    }

    pub fn is_circuit(&self) -> bool {
        if let Type::Circuit(_) = self {
            return true;
        }
        false
    }

    pub fn outer_dimension(&self, dimensions: &Vec<usize>) -> Self {
        let type_ = self.clone();

        if dimensions.len() > 1 {
            let mut next = vec![];
            next.extend_from_slice(&dimensions[1..]);

            return Type::Array(Box::new(type_), next);
        }

        type_
    }

    pub fn inner_dimension(&self, dimensions: &Vec<usize>) -> Self {
        let type_ = self.clone();

        if dimensions.len() > 1 {
            let mut next = vec![];
            next.extend_from_slice(&dimensions[..dimensions.len() - 1]);

            return Type::Array(Box::new(type_), next);
        }

        type_
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
        let element_type = Box::new(Type::from(array_type._type));
        let dimensions = array_type
            .dimensions
            .into_iter()
            .map(|row| Expression::get_count_from_ast(row))
            .collect();

        Type::Array(element_type, dimensions)
    }
}

impl<'ast> From<TupleType<'ast>> for Type {
    fn from(tuple_type: TupleType<'ast>) -> Self {
        let types = tuple_type.types.into_iter().map(|type_| Type::from(type_)).collect();

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
        let element_type = Box::new(Type::from(array_type._type));
        let dimensions = array_type
            .dimensions
            .into_iter()
            .map(|row| Expression::get_count_from_input_ast(row))
            .collect();

        Type::Array(element_type, dimensions)
    }
}

impl<'ast> From<InputTupleType<'ast>> for Type {
    fn from(tuple_type: InputTupleType<'ast>) -> Self {
        let types = tuple_type.types_.into_iter().map(|type_| Type::from(type_)).collect();

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
            Type::Array(ref array, ref dimensions) => {
                write!(f, "{}", *array)?;
                for row in dimensions {
                    write!(f, "[{}]", row)?;
                }
                write!(f, "")
            }
            Type::Tuple(ref tuple) => {
                let types = tuple.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ");

                write!(f, "({})", types)
            }
        }
    }
}
