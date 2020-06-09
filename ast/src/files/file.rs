use crate::{
    ast::Rule,
    circuits::Circuit,
    common::EOI,
    functions::{Function, TestFunction},
    imports::Import,
};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub imports: Vec<Import<'ast>>,
    pub circuits: Vec<Circuit<'ast>>,
    pub functions: Vec<Function<'ast>>,
    pub tests: Vec<TestFunction<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
