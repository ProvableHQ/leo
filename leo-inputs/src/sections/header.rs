use crate::{
    ast::Rule,
    common::Identifier,
    sections::{Main, Record, Registers, State, StateLeaf},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

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

impl<'ast> Header<'ast> {
    pub fn span(self) -> Span<'ast> {
        match self {
            Header::Main(main) => main.span,
            Header::Record(record) => record.span,
            Header::Registers(registers) => registers.span,
            Header::State(state) => state.span,
            Header::StateLeaf(state_leaf) => state_leaf.span,
            Header::Identifier(identifier) => identifier.span,
        }
    }
}

impl<'ast> fmt::Display for Header<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Header::Main(_main) => write!(f, "main"),
            Header::Record(_record) => write!(f, "record"),
            Header::Registers(_registers) => write!(f, "registers"),
            Header::State(_state) => write!(f, "state"),
            Header::StateLeaf(_state_leaf) => write!(f, "state_leaf"),
            Header::Identifier(identifier) => write!(f, "{}", identifier.value),
        }
    }
}
