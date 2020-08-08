use crate::Expression;
use leo_ast::common::RangeOrExpression as AstRangeOrExpression;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Range or expression enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RangeOrExpression {
    Range(Option<Expression>, Option<Expression>),
    Expression(Expression),
}

impl<'ast> From<AstRangeOrExpression<'ast>> for RangeOrExpression {
    fn from(range_or_expression: AstRangeOrExpression<'ast>) -> Self {
        match range_or_expression {
            AstRangeOrExpression::Range(range) => RangeOrExpression::Range(
                range.from.map(|expression| Expression::from(expression)),
                range.to.map(|expression| Expression::from(expression)),
            ),
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
