use crate::{ast::Rule, types::Identifier};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::import_symbol))]
pub struct ImportSymbol<'ast> {
    pub value: Identifier<'ast>,
    pub alias: Option<Identifier<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
