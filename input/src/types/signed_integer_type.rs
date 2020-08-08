use crate::ast::Rule;

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_integer_signed))]
pub enum SignedIntegerType {
    I8Type(I8Type),
    I16Type(I16Type),
    I32Type(I32Type),
    I64Type(I64Type),
    I128Type(I128Type),
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i8))]
pub struct I8Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i16))]
pub struct I16Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i32))]
pub struct I32Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i64))]
pub struct I64Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_i128))]
pub struct I128Type {}

impl fmt::Display for SignedIntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SignedIntegerType::I8Type(_) => write!(f, "i8"),
            SignedIntegerType::I16Type(_) => write!(f, "i16"),
            SignedIntegerType::I32Type(_) => write!(f, "i32"),
            SignedIntegerType::I64Type(_) => write!(f, "i64"),
            SignedIntegerType::I128Type(_) => write!(f, "i128"),
        }
    }
}
