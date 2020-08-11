use crate::{ast::Rule, expressions::*, values::Value};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'ast> {
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    ArrayInline(ArrayInlineExpression<'ast>),
    Tuple(TupleExpression<'ast>),
    Value(Value<'ast>),
}

impl<'ast> Expression<'ast> {
    pub fn span(&self) -> &Span {
        match self {
            Expression::ArrayInitializer(expression) => &expression.span,
            Expression::ArrayInline(expression) => &expression.span,
            Expression::Tuple(tuple) => &tuple.span,
            Expression::Value(value) => value.span(),
        }
    }
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::ArrayInitializer(ref expression) => {
                write!(f, "array [{} ; {}]", expression.expression, expression.count)
            }
            Expression::ArrayInline(ref array) => {
                let values = array
                    .expressions
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "array [{}]", values)
            }
            Expression::Tuple(ref tuple) => {
                let values = tuple
                    .expressions
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "({})", values)
            }
            Expression::Value(ref expression) => write!(f, "{}", expression),
        }
    }
}
