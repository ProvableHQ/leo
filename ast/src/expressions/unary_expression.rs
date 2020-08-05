use crate::{ast::Rule, expressions::Expression, operations::UnaryOperation, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_unary))]
pub struct UnaryExpression<'ast> {
    pub operation: UnaryOperation,
    pub expression: Box<Expression<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
