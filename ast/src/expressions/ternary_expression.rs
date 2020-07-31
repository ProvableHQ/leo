use crate::{ast::Rule, expressions::Expression, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_conditional))]
pub struct TernaryExpression<'ast> {
    pub first: Box<Expression<'ast>>,
    pub second: Box<Expression<'ast>>,
    pub third: Box<Expression<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
