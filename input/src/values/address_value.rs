use crate::{
    ast::Rule,
    values::{Address, AddressTyped},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_address))]
pub enum AddressValue<'ast> {
    Implicit(Address<'ast>),
    Explicit(AddressTyped<'ast>),
}

impl<'ast> AddressValue<'ast> {
    pub(crate) fn span(&self) -> &Span<'ast> {
        match self {
            AddressValue::Implicit(address) => &address.span,
            AddressValue::Explicit(address) => &address.span,
        }
    }
}

impl<'ast> fmt::Display for AddressValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddressValue::Explicit(address) => write!(f, "{}", address),
            AddressValue::Implicit(address) => write!(f, "{}", address),
        }
    }
}
