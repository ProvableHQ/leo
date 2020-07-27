use crate::{
    ast::Rule,
    common::Identifier,
    sections::{Main, Record, Registers, State, StateLeaf},
};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::header))]
pub enum Header<'ast> {
    Main(Main<'ast>),
    Record(Record<'ast>),
    Registers(Registers<'ast>),
    State(State<'ast>),
    StateLeaf(StateLeaf<'ast>),
    Identifier(Identifier<'ast>),
}
