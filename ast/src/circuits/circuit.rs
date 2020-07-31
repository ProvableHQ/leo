use crate::{ast::Rule, circuits::CircuitMember, common::Identifier, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::circuit))]
pub struct Circuit<'ast> {
    pub identifier: Identifier<'ast>,
    pub members: Vec<CircuitMember<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
