use crate::{ast::Rule, common::EOI, sections::Section, tables::Table};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub tables: Vec<Table<'ast>>,
    pub sections: Vec<Section<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
