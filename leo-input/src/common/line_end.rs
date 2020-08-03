use crate::ast::Rule;

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::LINE_END))]
pub struct LineEnd;
