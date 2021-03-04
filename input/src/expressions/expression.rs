// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::ast::Rule;
use crate::expressions::*;
use crate::values::Value;

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
                write!(f, "array [{} ; {}]", expression.expression, expression.dimensions)
            }
            Expression::ArrayInline(ref array) => {
                let values = array
                    .expressions
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "array [{}]", values)
            }
            Expression::Tuple(ref tuple) => {
                let values = tuple
                    .expressions
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "({})", values)
            }
            Expression::Value(ref expression) => write!(f, "{}", expression),
        }
    }
}
