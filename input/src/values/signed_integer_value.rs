use crate::{ast::Rule, types::SignedIntegerType, values::NumberValue};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::value_integer_signed))]
pub struct SignedIntegerValue<'ast> {
    pub number: NumberValue<'ast>,
    pub type_: SignedIntegerType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for SignedIntegerValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}
