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

use crate::{ArrayDimensions, GroupValue, Identifier, IntegerType, Node, SpreadOrExpression};

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
mod array_inline;
pub use array_inline::*;
mod array_init;
pub use array_init::*;
mod tuple_init;
pub use tuple_init::*;
mod circuit_init;
pub use circuit_init::*;
mod value;
pub use value::*;
mod call;
pub use call::*;
mod cast;
pub use cast::*;
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
    /// A cast expression `expr as type`.
    Cast(CastExpression),
    /// An access expression of some sort, e.g., `array[idx]` or `foo.bar`.
    Access(AccessExpression),
    /// An array expression where individual elements are listed inline,
    /// for example `[4, 6, ...[5, 7], 2]`.
    ArrayInline(ArrayInlineExpression),
    /// An array-repeat expression, e.g., `[42; 3]` yielding `[42, 42, 42]`.
    ArrayInit(ArrayInitExpression),
    /// A tuple expression e.g., `(foo, 42, true)`.
    TupleInit(TupleInitExpression),
    /// An expression constructing a structure like `Foo { bar: 42, baz }`.
    CircuitInit(CircuitInitExpression),
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
            ArrayInline(n) => n.span(),
            ArrayInit(n) => n.span(),
            TupleInit(n) => n.span(),
            CircuitInit(n) => n.span(),
            Call(n) => n.span(),
            Cast(n) => n.span(),
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
            ArrayInline(n) => n.set_span(span),
            ArrayInit(n) => n.set_span(span),
            TupleInit(n) => n.set_span(span),
            CircuitInit(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            Cast(n) => n.set_span(span),
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
            ArrayInline(n) => n.fmt(f),
            ArrayInit(n) => n.fmt(f),
            TupleInit(n) => n.fmt(f),
            CircuitInit(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            Cast(n) => n.fmt(f),
            Access(n) => n.fmt(f),
            Err(n) => n.fmt(f),
        }
    }
}
