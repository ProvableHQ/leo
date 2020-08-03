use crate::{ast::Rule, sections::Section, tables::Table};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::table_or_section))]
pub enum TableOrSection<'ast> {
    Section(Section<'ast>),
    Table(Table<'ast>),
}
