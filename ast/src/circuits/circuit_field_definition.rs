use crate::{ast::Rule, common::Identifier, types::Type, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::circuit_field_definition))]
pub struct CircuitFieldDefinition<'ast> {
    pub identifier: Identifier<'ast>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
