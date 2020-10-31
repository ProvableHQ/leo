// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{common::Identifier, expressions::*, operations::BinaryOperation, values::Value};

use pest::Span;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Expression<'ast> {
    Value(Value<'ast>),
    Identifier(Identifier<'ast>),
    Unary(Box<UnaryExpression<'ast>>),
    Binary(Box<BinaryExpression<'ast>>),
    Ternary(Box<TernaryExpression<'ast>>),
    ArrayInline(ArrayInlineExpression<'ast>),
    ArrayInitializer(Box<ArrayInitializerExpression<'ast>>),
    CircuitInline(CircuitInlineExpression<'ast>),
    Postfix(PostfixExpression<'ast>),
    Tuple(TupleExpression<'ast>),
}

impl<'ast> Expression<'ast> {
    pub fn binary(
        operation: BinaryOperation,
        left: Expression<'ast>,
        right: Expression<'ast>,
        span: Span<'ast>,
    ) -> Self {
        Expression::Binary(Box::new(BinaryExpression {
            operation,
            left,
            right,
            span,
        }))
    }

    pub fn ternary(
        first: Expression<'ast>,
        second: Expression<'ast>,
        third: Expression<'ast>,
        span: Span<'ast>,
    ) -> Self {
        Expression::Ternary(Box::new(TernaryExpression {
            first,
            second,
            third,
            span,
        }))
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
                write!(f, "[{} ; ({})]", expression.expression, expression.dimensions)
            }
            Expression::CircuitInline(ref expression) => write!(f, "{}", expression.span.as_str()),
            Expression::Postfix(ref expression) => write!(f, "{}", expression.span.as_str()),
            Expression::Tuple(ref expression) => write!(f, "{}", expression.span.as_str()),
        }
    }
}
