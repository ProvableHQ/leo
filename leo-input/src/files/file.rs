use crate::{ast::Rule, common::EOI, files::TableOrSection};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::file))]
pub struct File<'ast> {
    pub entries: Vec<TableOrSection<'ast>>,
    pub eoi: EOI,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
