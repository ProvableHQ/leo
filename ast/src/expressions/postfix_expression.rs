use crate::{access::Access, ast::Rule, common::Identifier, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_postfix))]
pub struct PostfixExpression<'ast> {
    pub identifier: Identifier<'ast>,
    pub accesses: Vec<Access<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
