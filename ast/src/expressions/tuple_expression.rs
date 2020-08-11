use crate::{ast::Rule, expressions::Expression, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_tuple))]
pub struct TupleExpression<'ast> {
    pub expressions: Vec<Expression<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
