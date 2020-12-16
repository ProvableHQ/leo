
pub use leo_ast::IntegerType;
use std::sync::{ Arc, Weak };
use crate::Circuit;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Type {
    // Data types
    Address,
    Boolean,
    Field,
    Group,
    Integer(IntegerType),

    // Data type wrappers
    Array(Box<Type>, usize),
    Tuple(Vec<Type>),
    Circuit(Arc<Circuit>),
}


#[derive(Clone)]
pub enum WeakType {
    Type(Type), // circuit not allowed
    Circuit(Weak<Circuit>),
}

impl Into<Type> for WeakType {
    fn into(self) -> Type {
        match self {
            WeakType::Type(t) => t,
            WeakType::Circuit(circuit) => Type::Circuit(circuit.upgrade().unwrap()),
        }
    }
}

impl Into<WeakType> for Type {
    fn into(self) -> WeakType {
        match self {
            Type::Circuit(circuit) => WeakType::Circuit(Arc::downgrade(&circuit)),
            t => WeakType::Type(t),
        }
    }
}

impl Type {
    pub fn is_assignable_from(&self, from: &Type) -> bool {
        self == from
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Address => write!(f, "address"),
            Type::Boolean => write!(f, "boolean"),
            Type::Field => write!(f, "field"),
            Type::Group => write!(f, "group"),
            Type::Integer(sub_type) => sub_type.fmt(f),
            Type::Array(sub_type, len) => write!(f, "{}[{}]", sub_type, len),
            Type::Tuple(sub_types) => {
                write!(f, "(")?;
                for (i, sub_type) in sub_types.iter().enumerate() {
                    write!(f, "{}", sub_type)?;
                    if i < sub_types.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            },
            Type::Circuit(circuit) => write!(f, "{}", &circuit.name.name),
        }
    }
}

impl Into<leo_ast::Type> for &Type {
    fn into(self) -> leo_ast::Type {
        use Type::*;
        match self {
            Address => leo_ast::Type::Address,
            Boolean => leo_ast::Type::Boolean,
            Field => leo_ast::Type::Field,
            Group => leo_ast::Type::Group,
            Integer(int_type) => leo_ast::Type::IntegerType(int_type.clone()),
            Array(type_, len) => leo_ast::Type::Array(Box::new(type_.as_ref().into()), leo_ast::ArrayDimensions(vec![leo_ast::PositiveNumber { value: len.to_string() }])),
            Tuple(subtypes) => leo_ast::Type::Tuple(subtypes.iter().map(Into::into).collect()),
            Circuit(circuit) => leo_ast::Type::Circuit(circuit.name.clone()),
        }
    }
}