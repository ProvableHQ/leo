use crate::{ast::Rule, types::AddressType, values::NumberValue};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_address))]
pub struct AddressValue<'ast> {
    pub _type: AddressType,
    pub number: NumberValue<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for AddressValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}
