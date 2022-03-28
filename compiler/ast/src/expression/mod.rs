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

mod accesses;
pub use accesses::*;
mod binary;
pub use binary::*;
mod unary;
pub use unary::*;
mod ternary;
pub use ternary::*;
mod tuple_init;
pub use tuple_init::*;
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
    Value(ValueExpression),
    /// A binary expression, e.g., `42 + 24`.
    Binary(BinaryExpression),
    /// An unary expression.
    Unary(UnaryExpression),
    /// A ternary conditional expression `cond ? if_expr : else_expr`.
    Ternary(TernaryExpression),
    /// An access expression of some sort, e.g., `array[idx]` or `foo.bar`.
    Access(AccessExpression),
    /// A tuple expression e.g., `(foo, 42, true)`.
    TupleInit(TupleInitExpression),
    /// A call expression like `my_fun(args)`.
    Call(CallExpression),
    /// An expression of type "error".
    /// Will result in a compile error eventually.
    Err(ErrExpression),
}

impl Node for Expression {
    fn span(&self) -> &Span {
        use Expression::*;
        match &self {
            Identifier(n) => n.span(),
            Value(n) => n.span(),
            Binary(n) => n.span(),
            Unary(n) => n.span(),
            Ternary(n) => n.span(),
            TupleInit(n) => n.span(),
            Call(n) => n.span(),
            Access(n) => n.span(),
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
            TupleInit(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            Access(n) => n.set_span(span),
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
            TupleInit(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            Access(n) => n.fmt(f),
            Err(n) => n.fmt(f),
        }
    }
}
