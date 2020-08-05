use crate::{
    ast::Rule,
    types::{SignedIntegerType, UnsignedIntegerType},
};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_integer))]
pub enum IntegerType {
    Signed(UnsignedIntegerType),
    Unsigned(SignedIntegerType),
}

impl fmt::Display for IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntegerType::Signed(integer) => write!(f, "{}", integer),
            IntegerType::Unsigned(integer) => write!(f, "{}", integer),
        }
    }
}
