// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{Identifier, IntegerType, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

mod binary;
pub use binary::*;
mod unary;
pub use unary::*;
mod ternary;
pub use ternary::*;
mod value;
pub use value::*;
mod call;
pub use call::*;
mod err;
pub use err::*;

/// Expression that evaluates to a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    /// An identifier expression.
    Identifier(Identifier),
    /// A literal expression.
    Literal(LiteralExpression),
    /// A binary expression, e.g., `42 + 24`.
    Binary(BinaryExpression),
    /// An unary expression.
    Unary(UnaryExpression),
    /// A ternary conditional expression `cond ? if_expr : else_expr`.
    Ternary(TernaryExpression),
    /// A call expression, e.g., `my_fun(args)`.
    Call(CallExpression),
    /// An expression of type "error".
    /// Will result in a compile error eventually.
    Err(ErrExpression),
}

impl Node for Expression {
    fn span(&self) -> Span {
        use Expression::*;
        match self {
            Identifier(n) => n.span(),
            Literal(n) => n.span(),
            Binary(n) => n.span(),
            Unary(n) => n.span(),
            Ternary(n) => n.span(),
            Call(n) => n.span(),
            Err(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Expression::*;
        match self {
            Identifier(n) => n.set_span(span),
            Literal(n) => n.set_span(span),
            Binary(n) => n.set_span(span),
            Unary(n) => n.set_span(span),
            Ternary(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            Err(n) => n.set_span(span),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expression::*;
        match &self {
            Identifier(n) => n.fmt(f),
            Literal(n) => n.fmt(f),
            Binary(n) => n.fmt(f),
            Unary(n) => n.fmt(f),
            Ternary(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            Err(n) => n.fmt(f),
        }
    }
}
