use crate::{ast::Rule, sections::Section, tables::Visibility};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::table))]
pub struct Table<'ast> {
    pub visibility: Visibility<'ast>,
    pub sections: Vec<Section<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Table<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[[{}]]", self.visibility)
    }
}
