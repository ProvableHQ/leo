use crate::ast::Rule;

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_integer_unsigned))]
pub enum UnsignedIntegerType {
    U8Type(U8Type),
    U16Type(U16Type),
    U32Type(U32Type),
    U64Type(U64Type),
    U128Type(U128Type),
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u8))]
pub struct U8Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u16))]
pub struct U16Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u32))]
pub struct U32Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u64))]
pub struct U64Type {}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u128))]
pub struct U128Type {}

impl fmt::Display for UnsignedIntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnsignedIntegerType::U8Type(_) => write!(f, "u8"),
            UnsignedIntegerType::U16Type(_) => write!(f, "u16"),
            UnsignedIntegerType::U32Type(_) => write!(f, "u32"),
            UnsignedIntegerType::U64Type(_) => write!(f, "u64"),
            UnsignedIntegerType::U128Type(_) => write!(f, "u128"),
        }
    }
}
