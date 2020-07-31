use crate::{ast::Rule, common::Identifier, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::type_circuit))]
pub struct CircuitType<'ast> {
    pub identifier: Identifier<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
