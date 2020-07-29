use crate::{Expression, Identifier, IntegerType};
use leo_ast::types::{ArrayType, CircuitType, DataType, Type as AstType};
use leo_inputs::types::{ArrayType as InputsArrayType, DataType as InputsDataType, Type as InputsAstType};

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

impl<'ast> From<InputsArrayType<'ast>> for Type {
    fn from(array_type: InputsArrayType<'ast>) -> Self {
        let element_type = Box::new(Type::from(array_type._type));
        let dimensions = array_type
            .dimensions
            .into_iter()
            .map(|row| Expression::get_count_from_number(row))
            .collect();

        Type::Array(element_type, dimensions)
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
            AstType::Circuit(type_) => Type::from(type_),
            AstType::SelfType(_type) => Type::SelfType,
        }
    }
}

/// inputs pest ast -> Explicit Type

impl From<InputsDataType> for Type {
    fn from(data_type: InputsDataType) -> Self {
        match data_type {
            InputsDataType::Address(_type) => Type::Address,
            InputsDataType::Boolean(_type) => Type::Boolean,
            InputsDataType::Field(_type) => Type::Field,
            InputsDataType::Group(_type) => Type::Group,
            InputsDataType::Integer(type_) => Type::IntegerType(IntegerType::from(type_)),
        }
    }
}

impl<'ast> From<ArrayType<'ast>> for Type {
    fn from(array_type: ArrayType<'ast>) -> Self {
        let element_type = Box::new(Type::from(array_type._type));
        let dimensions = array_type
            .dimensions
            .into_iter()
            .map(|row| Expression::get_count_from_value(row))
            .collect();

        Type::Array(element_type, dimensions)
    }
}

impl<'ast> From<InputsAstType<'ast>> for Type {
    fn from(type_: InputsAstType<'ast>) -> Self {
        match type_ {
            InputsAstType::Basic(type_) => Type::from(type_),
            InputsAstType::Array(type_) => Type::from(type_),
        }
    }
}

impl Type {
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
        }
    }
}
