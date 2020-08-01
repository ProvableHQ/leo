use crate::{
    ast::Rule,
    common::{Identifier, Mutable},
    types::Type,
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::function_input))]
pub struct FunctionInput<'ast> {
    pub mutable: Option<Mutable>,
    pub identifier: Identifier<'ast>,
    pub _type: Type<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
