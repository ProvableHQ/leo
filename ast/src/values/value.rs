use crate::{ast::{Group, Rule}, values::{BooleanValue, IntegerValue, FieldValue, NumberImplicitValue}};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value))]
pub enum Value<'ast> {
    Integer(IntegerValue<'ast>),
    Field(FieldValue<'ast>),
    Group(Group<'ast>),
    Boolean(BooleanValue<'ast>),
    Implicit(NumberImplicitValue<'ast>),
}

impl<'ast> Value<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            Value::Integer(value) => &value.span,
            Value::Field(value) => &value.span,
            Value::Group(value) => &value.span,
            Value::Boolean(value) => &value.span,
            Value::Implicit(value) => &value.span,
        }
    }
}

impl<'ast> fmt::Display for Value<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Integer(ref value) => write!(f, "{}", value),
            Value::Field(ref value) => write!(f, "{}", value),
            Value::Group(ref value) => write!(f, "{}", value),
            Value::Boolean(ref value) => write!(f, "{}", value),
            Value::Implicit(ref value) => write!(f, "{}", value),
        }
    }
}
