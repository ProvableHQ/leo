use crate::{ast::Rule, types::*};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_))]
pub enum Type<'ast> {
    Basic(DataType),
    Array(ArrayType<'ast>),
    Circuit(CircuitType<'ast>),
}

impl<'ast> fmt::Display for Type<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Basic(ref _type) => write!(f, "basic"),
            Type::Array(ref _type) => write!(f, "array"),
            Type::Circuit(ref _type) => write!(f, "struct"),
        }
    }
}
