use crate::{ast::Rule, expressions::Expression, operations::NotOperation, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_not))]
pub struct NotExpression<'ast> {
    pub operation: NotOperation<'ast>,
    pub expression: Box<Expression<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
