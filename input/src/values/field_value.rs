use crate::{ast::Rule, types::FieldType, values::NumberValue};

use crate::values::NumberImplicitValue;
use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::value_field))]
pub struct FieldValue<'ast> {
    pub number: NumberValue<'ast>,
    pub type_: FieldType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> FieldValue<'ast> {
    pub fn from_implicit(number: NumberImplicitValue<'ast>, type_: FieldType) -> Self {
        Self {
            number: number.number,
            type_,
            span: number.span,
        }
    }
}

impl<'ast> fmt::Display for FieldValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}
