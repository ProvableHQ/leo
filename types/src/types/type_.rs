use crate::{Expression, Identifier, IntegerType};
use leo_ast::types::{ArrayType, CircuitType, DataType, Type as AstType};

use std::fmt;

/// Explicit type used for defining a variable or expression type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    IntegerType(IntegerType),
    Field,
    Group,
    Boolean,
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
    fn from(basic_type: DataType) -> Self {
        match basic_type {
            DataType::Integer(_type) => Type::IntegerType(IntegerType::from(_type)),
            DataType::Field(_type) => Type::Field,
            DataType::Group(_type) => Type::Group,
            DataType::Boolean(_type) => Type::Boolean,
        }
    }
}

impl<'ast> From<ArrayType<'ast>> for Type {
    fn from(array_type: ArrayType<'ast>) -> Self {
        let element_type = Box::new(Type::from(array_type._type));
        let dimensions = array_type
            .dimensions
            .into_iter()
            .map(|row| Expression::get_count(row))
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
    fn from(_type: AstType<'ast>) -> Self {
        match _type {
            AstType::Basic(_type) => Type::from(_type),
            AstType::Array(_type) => Type::from(_type),
            AstType::Circuit(_type) => Type::from(_type),
            AstType::SelfType(_type) => Type::SelfType,
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
            Type::IntegerType(ref integer_type) => write!(f, "{}", integer_type),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::Boolean => write!(f, "bool"),
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
