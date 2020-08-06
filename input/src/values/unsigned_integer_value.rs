use crate::{ast::Rule, types::UnsignedIntegerType, values::PositiveNumber};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::value_integer_unsigned))]
pub struct UnsignedIntegerValue<'ast> {
    pub number: PositiveNumber<'ast>,
    pub type_: UnsignedIntegerType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for UnsignedIntegerValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}
