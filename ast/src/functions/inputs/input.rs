use crate::{
    ast::Rule,
    functions::{FunctionInput, Record, Registers, State, StateLeaf},
};

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::input))]
pub enum Input<'ast> {
    FunctionInput(FunctionInput<'ast>),
    Record(Record<'ast>),
    Registers(Registers<'ast>),
    State(State<'ast>),
    StateLeaf(StateLeaf<'ast>),
}
