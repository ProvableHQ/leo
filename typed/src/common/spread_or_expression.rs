use crate::Expression;
use leo_ast::common::SpreadOrExpression as AstSpreadOrExpression;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Spread or expression
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpreadOrExpression {
    Spread(Expression),
    Expression(Expression),
}

impl<'ast> From<AstSpreadOrExpression<'ast>> for SpreadOrExpression {
    fn from(s_or_e: AstSpreadOrExpression<'ast>) -> Self {
        match s_or_e {
            AstSpreadOrExpression::Spread(spread) => SpreadOrExpression::Spread(Expression::from(spread.expression)),
            AstSpreadOrExpression::Expression(expression) => {
                SpreadOrExpression::Expression(Expression::from(expression))
            }
        }
    }
}

impl fmt::Display for SpreadOrExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpreadOrExpression::Spread(ref spread) => write!(f, "...{}", spread),
            SpreadOrExpression::Expression(ref expression) => write!(f, "{}", expression),
        }
    }
}
