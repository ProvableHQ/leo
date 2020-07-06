use crate::{
    ast::Rule,
    values::{AddressValue, BooleanValue, FieldValue, GroupValue, IntegerValue, NumberImplicitValue},
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value))]
pub enum Value<'ast> {
    Address(AddressValue<'ast>),
    Boolean(BooleanValue<'ast>),
    Field(FieldValue<'ast>),
    Group(GroupValue<'ast>),
    Implicit(NumberImplicitValue<'ast>),
    Integer(IntegerValue<'ast>),
}

impl<'ast> Value<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            Value::Address(value) => &value.span,
            Value::Boolean(value) => &value.span,
            Value::Field(value) => &value.span,
            Value::Group(value) => &value.span,
            Value::Implicit(value) => &value.span,
            Value::Integer(value) => &value.span,
        }
    }
}

impl<'ast> fmt::Display for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Address(ref value) => write!(f, "{}", value),
            Value::Boolean(ref value) => write!(f, "{}", value),
            Value::Field(ref value) => write!(f, "{}", value),
            Value::Group(ref value) => write!(f, "{}", value),
            Value::Implicit(ref value) => write!(f, "{}", value),
            Value::Integer(ref value) => write!(f, "{}", value),
        }
    }
}
