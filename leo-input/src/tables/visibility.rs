use crate::{
    ast::Rule,
    tables::{Private, Public},
};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::visibility))]
pub enum Visibility<'ast> {
    Private(Private<'ast>),
    Public(Public<'ast>),
}

impl<'ast> fmt::Display for Visibility<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Visibility::Private(private) => write!(f, "{}", private),
            Visibility::Public(public) => write!(f, "{}", public),
        }
    }
}
