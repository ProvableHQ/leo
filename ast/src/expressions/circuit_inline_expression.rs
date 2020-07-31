use crate::{ast::Rule, circuits::CircuitField, common::Identifier, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_circuit_inline))]
pub struct CircuitInlineExpression<'ast> {
    pub identifier: Identifier<'ast>,
    pub members: Vec<CircuitField<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
