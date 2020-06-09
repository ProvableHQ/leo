use crate::{Expression, Integer};
use leo_ast::common::RangeOrExpression as AstRangeOrExpression;

use std::fmt;

/// Range or expression enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeOrExpression {
    Range(Option<Integer>, Option<Integer>),
    Expression(Expression),
}

impl<'ast> From<AstRangeOrExpression<'ast>> for RangeOrExpression {
    fn from(range_or_expression: AstRangeOrExpression<'ast>) -> Self {
        match range_or_expression {
            AstRangeOrExpression::Range(range) => {
                let from = range.from.map(|from| match Expression::from(from.0) {
                    Expression::Integer(number) => number,
                    Expression::Implicit(string) => Integer::from_implicit(string),
                    expression => unimplemented!("Range bounds should be integers, found {}", expression),
                });
                let to = range.to.map(|to| match Expression::from(to.0) {
                    Expression::Integer(number) => number,
                    Expression::Implicit(string) => Integer::from_implicit(string),
                    expression => unimplemented!("Range bounds should be integers, found {}", expression),
                });

                RangeOrExpression::Range(from, to)
            }
            AstRangeOrExpression::Expression(expression) => RangeOrExpression::Expression(Expression::from(expression)),
        }
    }
}

impl fmt::Display for RangeOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RangeOrExpression::Range(ref from, ref to) => write!(
                f,
                "{}..{}",
                from.as_ref().map(|e| format!("{}", e)).unwrap_or("".to_string()),
                to.as_ref().map(|e| format!("{}", e)).unwrap_or("".to_string())
            ),
            RangeOrExpression::Expression(ref e) => write!(f, "{}", e),
        }
    }
}
