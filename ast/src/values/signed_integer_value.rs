use crate::{ast::Rule, types::SignedIntegerType, values::NumberValue, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::value_integer_signed))]
pub struct SignedIntegerValue<'ast> {
    pub number: NumberValue<'ast>,
    pub type_: SignedIntegerType,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for SignedIntegerValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.number, self.type_)
    }
}
