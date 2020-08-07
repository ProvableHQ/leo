use crate::ast::Rule;

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_unary))]
pub enum UnaryOperation {
    Minus(Minus),
    Not(Not),
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperation::Minus(_) => write!(f, "-"),
            UnaryOperation::Not(_) => write!(f, "!"),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_not))]
pub struct Not {}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_minus))]
pub struct Minus {}
