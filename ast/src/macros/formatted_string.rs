use crate::{
    ast::{span_into_string, Rule},
    macros::{FormattedContainer, FormattedParameter},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::formatted_string))]
pub struct FormattedString<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub string: String,
    pub containers: Vec<FormattedContainer<'ast>>,
    pub parameters: Vec<FormattedParameter<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for FormattedString<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
