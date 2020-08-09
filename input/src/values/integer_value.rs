use crate::{
    ast::Rule,
    values::{SignedIntegerValue, UnsignedIntegerValue},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::value_integer))]
pub enum IntegerValue<'ast> {
    Signed(SignedIntegerValue<'ast>),
    Unsigned(UnsignedIntegerValue<'ast>),
}

impl<'ast> IntegerValue<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            IntegerValue::Signed(integer) => &integer.span,
            IntegerValue::Unsigned(integer) => &integer.span,
        }
    }
}

impl<'ast> fmt::Display for IntegerValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntegerValue::Signed(integer) => write!(f, "{}", integer),
            IntegerValue::Unsigned(integer) => write!(f, "{}", integer),
        }
    }
}
