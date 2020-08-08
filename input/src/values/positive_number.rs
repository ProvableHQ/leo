use crate::ast::{span_into_string, Rule};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::number_positive))]
pub struct PositiveNumber<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for PositiveNumber<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
