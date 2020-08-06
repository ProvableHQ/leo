use crate::ast::Rule;

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_unary))]
pub enum UnaryOperation {
    Minus(Minus),
    Not(Not),
}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_not))]
pub struct Not {}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_minus))]
pub struct Minus {}
