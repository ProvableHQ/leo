use crate::{ast::Rule, common::Static, functions::Function, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::circuit_function))]
pub struct CircuitFunction<'ast> {
    pub _static: Option<Static>,
    pub function: Function<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
