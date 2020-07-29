use crate::{ast::Rule, common::Identifier, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::import_symbol))]
pub struct ImportSymbol<'ast> {
    pub value: Identifier<'ast>,
    pub alias: Option<Identifier<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
