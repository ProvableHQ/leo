use crate::{ast::Rule, expressions::*, values::Value};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'ast> {
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    Value(Value<'ast>),
}

impl<'ast> Expression<'ast> {
    pub fn span(&self) -> &Span {
        match self {
            Expression::ArrayInline(expression) => &expression.span,
            Expression::ArrayInitializer(expression) => &expression.span,
            Expression::Value(value) => value.span(),
        }
    }
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Value(ref expression) => write!(f, "{}", expression),
            Expression::ArrayInline(ref expression) => {
                for (i, value) in expression.expressions.iter().enumerate() {
                    write!(f, "array [{}", value)?;
                    if i < expression.expressions.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Expression::ArrayInitializer(ref expression) => {
                write!(f, "array [{} ; {}]", expression.expression, expression.count)
            }
        }
    }
}
