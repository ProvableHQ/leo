use crate::{ast::Rule, common::ReturnTuple, expressions::Expression};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::return_))]
pub enum Return<'ast> {
    Single(Expression<'ast>),
    Tuple(ReturnTuple<'ast>),
}

impl<'ast> fmt::Display for Return<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Return::Single(ref expression) => write!(f, "{}", expression),
            Return::Tuple(ref expressions) => write!(f, "{}", expressions),
        }
    }
}
