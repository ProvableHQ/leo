use crate::ast::Rule;

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::error))]
pub struct Error<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Error<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}
