use crate::{ast::Rule, common::Range, expressions::Expression};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::range_or_expression))]
pub enum RangeOrExpression<'ast> {
    Range(Range<'ast>),
    Expression(Expression<'ast>),
}

impl<'ast> fmt::Display for RangeOrExpression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RangeOrExpression::Expression(ref expression) => write!(f, "{}", expression),
            RangeOrExpression::Range(ref range) => write!(
                f,
                "{}..{}",
                range.from.as_ref().map(|v| format!("{}", v)).unwrap_or(format!("")),
                range.to.as_ref().map(|v| format!("{}", v)).unwrap_or(format!("")),
            ),
        }
    }
}
