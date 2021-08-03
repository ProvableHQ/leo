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

use crate::{
    ArrayDimensions, CircuitImpliedVariableDefinition, GroupValue, Identifier, IntegerType, PositiveNumber, Span,
    SpreadOrExpression,
};

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::Node;

mod binary;
pub use binary::*;
mod unary;
pub use unary::*;
mod ternary;
pub use ternary::*;
mod array_access;
pub use array_access::*;
mod array_range_access;
pub use array_range_access::*;
mod array_inline;
pub use array_inline::*;
mod array_init;
pub use array_init::*;
mod tuple_access;
pub use tuple_access::*;
mod tuple_init;
pub use tuple_init::*;
mod circuit_static_function_access;
pub use circuit_static_function_access::*;
mod circuit_member_access;
pub use circuit_member_access::*;
mod circuit_init;
pub use circuit_init::*;
mod value;
pub use value::*;
mod call;
pub use call::*;
mod cast;
pub use cast::*;

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    Identifier(Identifier),
    Value(ValueExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Ternary(TernaryExpression),
    Cast(CastExpression),

    ArrayInline(ArrayInlineExpression),
    ArrayInit(ArrayInitExpression),
    ArrayAccess(ArrayAccessExpression),
    ArrayRangeAccess(ArrayRangeAccessExpression),

    TupleInit(TupleInitExpression),
    TupleAccess(TupleAccessExpression),

    CircuitInit(CircuitInitExpression),
    CircuitMemberAccess(CircuitMemberAccessExpression),
    CircuitStaticFunctionAccess(CircuitStaticFunctionAccessExpression),

    Call(CallExpression),
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
            ArrayAccess(n) => n.span(),
            ArrayRangeAccess(n) => n.span(),
            TupleInit(n) => n.span(),
            TupleAccess(n) => n.span(),
            CircuitInit(n) => n.span(),
            CircuitMemberAccess(n) => n.span(),
            CircuitStaticFunctionAccess(n) => n.span(),
            Call(n) => n.span(),
            Cast(n) => n.span(),
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
            ArrayAccess(n) => n.set_span(span),
            ArrayRangeAccess(n) => n.set_span(span),
            TupleInit(n) => n.set_span(span),
            TupleAccess(n) => n.set_span(span),
            CircuitInit(n) => n.set_span(span),
            CircuitMemberAccess(n) => n.set_span(span),
            CircuitStaticFunctionAccess(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            Cast(n) => n.set_span(span),
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
            ArrayAccess(n) => n.fmt(f),
            ArrayRangeAccess(n) => n.fmt(f),
            TupleInit(n) => n.fmt(f),
            TupleAccess(n) => n.fmt(f),
            CircuitInit(n) => n.fmt(f),
            CircuitMemberAccess(n) => n.fmt(f),
            CircuitStaticFunctionAccess(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            Cast(n) => n.fmt(f),
        }
    }
}
