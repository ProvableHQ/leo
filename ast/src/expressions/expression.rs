use crate::{common::Identifier, expressions::*, operations::BinaryOperation, values::Value};

use pest::Span;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Expression<'ast> {
    Value(Value<'ast>),
    Identifier(Identifier<'ast>),
    Unary(UnaryExpression<'ast>),
    Binary(BinaryExpression<'ast>),
    Ternary(TernaryExpression<'ast>),
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(ArrayInitializerExpression<'ast>),
    CircuitInline(CircuitInlineExpression<'ast>),
    Postfix(PostfixExpression<'ast>),
    Tuple(TupleExpression<'ast>),
}

impl<'ast> Expression<'ast> {
    pub fn binary(
        operation: BinaryOperation,
        left: Box<Expression<'ast>>,
        right: Box<Expression<'ast>>,
        span: Span<'ast>,
    ) -> Self {
        Expression::Binary(BinaryExpression {
            operation,
            left,
            right,
            span,
        })
    }

    pub fn ternary(
        first: Box<Expression<'ast>>,
        second: Box<Expression<'ast>>,
        third: Box<Expression<'ast>>,
        span: Span<'ast>,
    ) -> Self {
        Expression::Ternary(TernaryExpression {
            first,
            second,
            third,
            span,
        })
    }

    pub fn span(&self) -> &Span<'ast> {
        match self {
            Expression::Value(expression) => &expression.span(),
            Expression::Identifier(expression) => &expression.span,
            Expression::Unary(expression) => &expression.span,
            Expression::Binary(expression) => &expression.span,
            Expression::Ternary(expression) => &expression.span,
            Expression::ArrayInline(expression) => &expression.span,
            Expression::ArrayInitializer(expression) => &expression.span,
            Expression::CircuitInline(expression) => &expression.span,
            Expression::Postfix(expression) => &expression.span,
            Expression::Tuple(expression) => &expression.span,
        }
    }
}

impl<'ast> fmt::Display for Expression<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Value(ref expression) => write!(f, "{}", expression),
            Expression::Identifier(ref expression) => write!(f, "{}", expression),
            Expression::Unary(ref expression) => write!(f, "{}", expression),
            Expression::Binary(ref expression) => write!(f, "{} == {}", expression.left, expression.right),
            Expression::Ternary(ref expression) => write!(
                f,
                "if {} ? {} : {}",
                expression.first, expression.second, expression.third
            ),
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
            Expression::CircuitInline(ref expression) => write!(f, "{}", expression.span.as_str()),
            Expression::Postfix(ref expression) => write!(f, "{}", expression.span.as_str()),
            Expression::Tuple(ref expression) => write!(f, "{}", expression.span.as_str()),
        }
    }
}
