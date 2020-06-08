use crate::{ast::Rule, common::Identifier, expressions::*, values::Value,};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'ast> {
    CircuitInline(CircuitInlineExpression<'ast>),
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    Value(Value<'ast>),
    Variable(Identifier<'ast>),
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Value(ref expression) => write!(f, "{}", expression),
            Expression::Variable(ref expression) => write!(f, "{}", expression),
            Expression::ArrayInline(ref expression) => {
                for (i, spread_or_expression) in expression.expressions.iter().enumerate() {
                    write!(f, "{}", spread_or_expression)?;
                    if i < expression.expressions.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "")
            }
            Expression::ArrayInitializer(ref expression) => {
                write!(f, "[{} ; {}]", expression.expression, expression.count)
            }
            Expression::CircuitInline(ref expression) => write!(
                f,
                "inline circuit display not impl {}",
                expression.identifier
            ),
        }
    }
}
