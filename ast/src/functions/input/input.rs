use crate::{
    ast::Rule,
    functions::{FunctionInput, InputKeyword},
};

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::input))]
pub enum Input<'ast> {
    InputKeyword(InputKeyword<'ast>),
    FunctionInput(FunctionInput<'ast>),
}
