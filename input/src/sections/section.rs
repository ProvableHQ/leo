use crate::{ast::Rule, definitions::Definition, sections::Header};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::section))]
pub struct Section<'ast> {
    pub header: Header<'ast>,
    pub definitions: Vec<Definition<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
