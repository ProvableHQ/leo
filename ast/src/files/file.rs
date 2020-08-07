use crate::{ast::Rule, common::EOI, definitions::Definition, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub definitions: Vec<Definition<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
