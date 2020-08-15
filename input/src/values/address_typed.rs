use crate::{ast::Rule, types::AddressType, values::address::Address};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::address_typed))]
pub struct AddressTyped<'ast> {
    pub type_: AddressType,
    pub address: Address<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for AddressTyped<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address({})", self.address)
    }
}
