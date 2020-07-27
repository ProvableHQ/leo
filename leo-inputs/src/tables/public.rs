use crate::ast::Rule;

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::public))]
pub struct Public<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
