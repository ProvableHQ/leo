use crate::{
    ast::Rule,
    types::IntegerType,
    values::{NumberImplicitValue, NumberValue},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::value_integer))]
pub struct IntegerValue<'ast> {
    pub number: NumberValue<'ast>,
    pub type_: IntegerType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> IntegerValue<'ast> {
    pub fn from_implicit(number: NumberImplicitValue<'ast>, type_: IntegerType) -> Self {
        Self {
            number: number.number,
            type_,
            span: number.span,
        }
    }
}

impl<'ast> fmt::Display for IntegerValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.type_)
    }
}
