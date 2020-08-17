use crate::{ast::Rule, common::Identifier, functions::input::Input, statements::Statement, types::Type, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::function))]
pub struct Function<'ast> {
    pub identifier: Identifier<'ast>,
    pub parameters: Vec<Input<'ast>>,
    pub returns: Option<Type<'ast>>,
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
