use crate::{
    ast::Rule,
    tables::{Private, Public},
};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::visibility))]
pub enum Visibility<'ast> {
    Private(Private<'ast>),
    Public(Public<'ast>),
}
