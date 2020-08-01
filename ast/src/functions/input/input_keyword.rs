use crate::{
    ast::{span_into_string, Rule},
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::input_keyword))]
pub struct InputKeyword<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub keyword: String,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
