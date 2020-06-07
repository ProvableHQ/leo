use crate::ast::Rule;

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
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
