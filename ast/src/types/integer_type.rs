use crate::{
    ast::Rule,
    types::{SignedIntegerType, UnsignedIntegerType},
};

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::type_integer))]
pub enum IntegerType {
    Signed(UnsignedIntegerType),
    Unsigned(SignedIntegerType),
}
