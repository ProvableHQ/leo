use crate::{
    ast::Rule,
    common::LineEnd,
    imports::{ImportSource, ImportSymbol},
};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::import))]
pub struct Import<'ast> {
    pub source: ImportSource<'ast>,
    pub symbols: Vec<ImportSymbol<'ast>>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
