use crate::{ast::Rule, common::Test, imports::Import, circuits::Circuit};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub imports: Vec<Import<'ast>>,
    pub circuits: Vec<Circuit<'ast>>,
    pub functions: Vec<Function<'ast>>,
    pub tests: Vec<Test<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
