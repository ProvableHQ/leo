// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{Identifier, Node, NodeID};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

mod access;
pub use access::*;

mod array;
pub use array::*;

mod binary;
pub use binary::*;

mod call;
pub use call::*;

mod cast;
pub use cast::*;

mod struct_init;
pub use struct_init::*;

mod err;
pub use err::*;

mod ternary;
pub use ternary::*;

mod tuple;
pub use tuple::*;

mod unary;
pub use unary::*;

mod unit;
pub use unit::*;

mod literal;
pub use literal::*;

/// Expression that evaluates to a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    /// A struct access expression, e.g. `Foo.bar`.
    Access(AccessExpression),
    /// An array expression, e.g., `[true, false, true, false]`.
    Array(ArrayExpression),
    /// A binary expression, e.g., `42 + 24`.
    Binary(BinaryExpression),
    /// A call expression, e.g., `my_fun(args)`.
    Call(CallExpression),
    /// A cast expression, e.g., `42u32 as u8`.
    Cast(CastExpression),
    /// An expression constructing a struct like `Foo { bar: 42, baz }`.
    Struct(StructExpression),
    /// An expression of type "error".
    /// Will result in a compile error eventually.
    Err(ErrExpression),
    /// An identifier.
    Identifier(Identifier),
    /// A literal expression.
    Literal(Literal),
    /// A ternary conditional expression `cond ? if_expr : else_expr`.
    Ternary(TernaryExpression),
    /// A tuple expression e.g., `(foo, 42, true)`.
    Tuple(TupleExpression),
    /// An unary expression.
    Unary(UnaryExpression),
    /// A unit expression e.g. `()`
    Unit(UnitExpression),
}

impl Node for Expression {
    fn span(&self) -> Span {
        use Expression::*;
        match self {
            Access(n) => n.span(),
            Array(n) => n.span(),
            Binary(n) => n.span(),
            Call(n) => n.span(),
            Cast(n) => n.span(),
            Struct(n) => n.span(),
            Err(n) => n.span(),
            Identifier(n) => n.span(),
            Literal(n) => n.span(),
            Ternary(n) => n.span(),
            Tuple(n) => n.span(),
            Unary(n) => n.span(),
            Unit(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Expression::*;
        match self {
            Access(n) => n.set_span(span),
            Array(n) => n.set_span(span),
            Binary(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            Cast(n) => n.set_span(span),
            Struct(n) => n.set_span(span),
            Identifier(n) => n.set_span(span),
            Literal(n) => n.set_span(span),
            Err(n) => n.set_span(span),
            Ternary(n) => n.set_span(span),
            Tuple(n) => n.set_span(span),
            Unary(n) => n.set_span(span),
            Unit(n) => n.set_span(span),
        }
    }

    fn id(&self) -> NodeID {
        use Expression::*;
        match self {
            Access(n) => n.id(),
            Array(n) => n.id(),
            Binary(n) => n.id(),
            Call(n) => n.id(),
            Cast(n) => n.id(),
            Struct(n) => n.id(),
            Identifier(n) => n.id(),
            Literal(n) => n.id(),
            Err(n) => n.id(),
            Ternary(n) => n.id(),
            Tuple(n) => n.id(),
            Unary(n) => n.id(),
            Unit(n) => n.id(),
        }
    }

    fn set_id(&mut self, id: NodeID) {
        use Expression::*;
        match self {
            Access(n) => n.set_id(id),
            Array(n) => n.set_id(id),
            Binary(n) => n.set_id(id),
            Call(n) => n.set_id(id),
            Cast(n) => n.set_id(id),
            Struct(n) => n.set_id(id),
            Identifier(n) => n.set_id(id),
            Literal(n) => n.set_id(id),
            Err(n) => n.set_id(id),
            Ternary(n) => n.set_id(id),
            Tuple(n) => n.set_id(id),
            Unary(n) => n.set_id(id),
            Unit(n) => n.set_id(id),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expression::*;
        match &self {
            Access(n) => n.fmt(f),
            Array(n) => n.fmt(f),
            Binary(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            Cast(n) => n.fmt(f),
            Struct(n) => n.fmt(f),
            Err(n) => n.fmt(f),
            Identifier(n) => n.fmt(f),
            Literal(n) => n.fmt(f),
            Ternary(n) => n.fmt(f),
            Tuple(n) => n.fmt(f),
            Unary(n) => n.fmt(f),
            Unit(n) => n.fmt(f),
        }
    }
}
