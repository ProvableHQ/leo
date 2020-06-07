use crate::{ast::Rule, circuits::CircuitMember, common::Identifier};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_definition))]
pub struct Circuit<'ast> {
    pub identifier: Identifier<'ast>,
    pub members: Vec<CircuitMember<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
