use crate::ast::Rule;

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::operation_binary))]
pub enum BinaryOperation {
    Or,
    And,
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
