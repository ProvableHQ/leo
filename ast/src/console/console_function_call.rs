use crate::{
    ast::Rule,
    common::LineEnd,
    console::{ConsoleFunction, ConsoleKeyword},
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::console_function_call))]
pub struct ConsoleFunctionCall<'ast> {
    pub keyword: ConsoleKeyword<'ast>,
    pub function: ConsoleFunction<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for ConsoleFunctionCall<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "console.{};", self.function)
    }
}
