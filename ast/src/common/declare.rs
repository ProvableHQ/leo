use crate::ast::Rule;

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::declare))]
pub enum Declare {
    Const(Const),
    Let(Let),
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::const_))]
pub struct Const {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::let_))]
pub struct Let {}
