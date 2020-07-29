use crate::{expressions::Expression, operations::BinaryOperation, SpanDef};

use pest::Span;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BinaryExpression<'ast> {
    pub operation: BinaryOperation,
    pub left: Box<Expression<'ast>>,
    pub right: Box<Expression<'ast>>,
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
