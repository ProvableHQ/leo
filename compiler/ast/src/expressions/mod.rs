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

use crate::{GroupValue, Identifier, IntegerType, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

mod access;
pub use access::*;

mod binary;
pub use binary::*;

mod call;
pub use call::*;

mod circuit_init;
pub use circuit_init::*;

mod err;
pub use err::*;

mod ternary;
pub use ternary::*;

mod unary;
pub use unary::*;

mod value;
pub use value::*;

/// Expression that evaluates to a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    /// A binary expression, e.g., `42 + 24`.
    Binary(BinaryExpression),
    /// A call expression, e.g., `my_fun(args)`.
    Call(CallExpression),
    /// An expression constructing a structure like `Foo { bar: 42, baz }`.
    CircuitInit(CircuitInitExpression),
    /// An expression of type "error".
    /// Will result in a compile error eventually.
    Err(ErrExpression),
    /// An identifier expression.
    Identifier(Identifier),
    /// A ternary conditional expression `cond ? if_expr : else_expr`.
    Ternary(TernaryExpression),
    /// An unary expression.
    Unary(UnaryExpression),
    /// A literal expression.
    Value(ValueExpression),
}

impl Node for Expression {
    fn span(&self) -> Span {
        use Expression::*;
        match self {
            Identifier(n) => n.span(),
            Value(n) => n.span(),
            Binary(n) => n.span(),
            Unary(n) => n.span(),
            Ternary(n) => n.span(),
            Call(n) => n.span(),
            CircuitInit(n) => n.span(),
            Err(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Expression::*;
        match self {
            Identifier(n) => n.set_span(span),
            Value(n) => n.set_span(span),
            Binary(n) => n.set_span(span),
            Unary(n) => n.set_span(span),
            Ternary(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            CircuitInit(n) => n.set_span(span),
            Err(n) => n.set_span(span),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expression::*;
        match &self {
            Identifier(n) => n.fmt(f),
            Value(n) => n.fmt(f),
            Binary(n) => n.fmt(f),
            Unary(n) => n.fmt(f),
            Ternary(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            CircuitInit(n) => n.fmt(f),
            Err(n) => n.fmt(f),
        }
    }
}
