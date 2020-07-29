use crate::{ast::Rule, common::Identifier, expressions::Expression, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::circuit_field))]
pub struct CircuitField<'ast> {
    pub identifier: Identifier<'ast>,
    pub expression: Expression<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
