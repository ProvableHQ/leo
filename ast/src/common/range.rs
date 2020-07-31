use crate::{ast::Rule, expressions::Expression, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::range))]
pub struct Range<'ast> {
    pub from: Option<Expression<'ast>>,
    pub to: Option<Expression<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
