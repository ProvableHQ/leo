use crate::{
    ast::Rule,
    circuits::Circuit,
    functions::{Function, TestFunction},
};

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::definition))]
pub enum Definition<'ast> {
    Circuit(Circuit<'ast>),
    Function(Function<'ast>),
    TestFunction(TestFunction<'ast>),
}
