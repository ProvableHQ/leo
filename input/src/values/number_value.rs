use crate::{
    ast::Rule,
    values::{NegativeNumber, PositiveNumber},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::value_number))]
pub enum NumberValue<'ast> {
    Negative(NegativeNumber<'ast>),
    Positive(PositiveNumber<'ast>),
}

impl<'ast> NumberValue<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            NumberValue::Negative(number) => &number.span,
            NumberValue::Positive(number) => &number.span,
        }
    }
}

impl<'ast> fmt::Display for NumberValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberValue::Negative(number) => write!(f, "{}", number),
            NumberValue::Positive(number) => write!(f, "{}", number),
        }
    }
}
