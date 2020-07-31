use crate::{ast::Rule, common::LineEnd, expressions::Expression, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::statement_expression))]
pub struct ExpressionStatement<'ast> {
    pub expression: Expression<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
