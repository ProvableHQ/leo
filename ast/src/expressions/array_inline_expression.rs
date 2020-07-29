use crate::{ast::Rule, common::SpreadOrExpression, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_array_inline))]
pub struct ArrayInlineExpression<'ast> {
    pub expressions: Vec<SpreadOrExpression<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
