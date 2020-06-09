use crate::{ast::Rule, common::Identifier};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::header))]
pub struct Header<'ast> {
    pub name: Identifier<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
