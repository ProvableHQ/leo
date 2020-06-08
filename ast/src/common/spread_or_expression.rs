use crate::{ast::Rule, common::Spread, expressions::Expression};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::spread_or_expression))]
pub enum SpreadOrExpression<'ast> {
    Spread(Spread<'ast>),
    Expression(Expression<'ast>),
}

impl<'ast> fmt::Display for SpreadOrExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadOrExpression::Spread(ref spread) => write!(f, "{}", spread),
            SpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}
