use crate::{ast::Rule, types::*};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::type_))]
pub enum Type<'ast> {
    Basic(DataType),
    Array(ArrayType<'ast>),
    Tuple(TupleType<'ast>),
    Circuit(CircuitType<'ast>),
    SelfType(SelfType),
}

impl<'ast> fmt::Display for Type<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Basic(ref _type) => write!(f, "basic"),
            Type::Array(ref _type) => write!(f, "array"),
            Type::Tuple(ref _type) => write!(f, "tuple"),
            Type::Circuit(ref _type) => write!(f, "struct"),
            Type::SelfType(ref _type) => write!(f, "Self"),
        }
    }
}
