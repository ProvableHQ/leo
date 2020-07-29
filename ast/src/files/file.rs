use crate::{
    ast::Rule,
    circuits::Circuit,
    common::EOI,
    functions::{Function, TestFunction},
    imports::Import,
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub imports: Vec<Import<'ast>>,
    pub circuits: Vec<Circuit<'ast>>,
    pub functions: Vec<Function<'ast>>,
    pub tests: Vec<TestFunction<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
