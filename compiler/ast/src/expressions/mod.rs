// Copyright (C) 2019-2025 Provable Inc.
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

mod array_access;
pub use array_access::*;

mod associated_constant;
pub use associated_constant::*;

mod associated_function;
pub use associated_function::*;

mod async_;
pub use async_::*;

mod array;
pub use array::*;

mod binary;
pub use binary::*;

mod call;
pub use call::*;

mod cast;
pub use cast::*;

mod err;
pub use err::*;

mod member_access;
pub use member_access::*;

mod repeat;
pub use repeat::*;

mod struct_init;
pub use struct_init::*;

mod ternary;
pub use ternary::*;

mod tuple;
pub use tuple::*;

mod tuple_access;
pub use tuple_access::*;

mod unary;
pub use unary::*;

mod unit;
pub use unit::*;

mod literal;
pub use literal::*;

pub mod locator;
pub use locator::*;

/// Expression that evaluates to a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    /// An array access, e.g. `arr[i]`.
    ArrayAccess(Box<ArrayAccess>),
    /// An associated constant; e.g., `group::GEN`.
    AssociatedConstant(AssociatedConstantExpression),
    /// An associated function; e.g., `BHP256::hash_to_field`.
    AssociatedFunction(AssociatedFunctionExpression),
    /// An `async` block: e.g. `async { my_mapping.set(1, 2); }`.
    Async(AsyncExpression),
    /// An array expression, e.g., `[true, false, true, false]`.
    Array(ArrayExpression),
    /// A binary expression, e.g., `42 + 24`.
    Binary(Box<BinaryExpression>),
    /// A call expression, e.g., `my_fun(args)`.
    Call(Box<CallExpression>),
    /// A cast expression, e.g., `42u32 as u8`.
    Cast(Box<CastExpression>),
    /// An expression of type "error".
    /// Will result in a compile error eventually.
    Err(ErrExpression),
    /// An identifier.
    Identifier(Identifier),
    /// A literal expression.
    Literal(Literal),
    /// A locator expression, e.g., `hello.aleo/foo`.
    Locator(LocatorExpression),
    /// An access of a struct member, e.g. `struc.member`.
    MemberAccess(Box<MemberAccess>),
    /// An array expression constructed from one repeated element, e.g., `[1u32; 5]`.
    Repeat(Box<RepeatExpression>),
    /// An expression constructing a struct like `Foo { bar: 42, baz }`.
    Struct(StructExpression),
    /// A ternary conditional expression `cond ? if_expr : else_expr`.
    Ternary(Box<TernaryExpression>),
    /// A tuple expression e.g., `(foo, 42, true)`.
    Tuple(TupleExpression),
    /// A tuple access expression e.g., `foo.2`.
    TupleAccess(Box<TupleAccess>),
    /// An unary expression.
    Unary(Box<UnaryExpression>),
    /// A unit expression e.g. `()`
    Unit(UnitExpression),
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Err(Default::default())
    }
}

impl Node for Expression {
    fn span(&self) -> Span {
        use Expression::*;
        match self {
            ArrayAccess(n) => n.span(),
            Array(n) => n.span(),
            AssociatedConstant(n) => n.span(),
            AssociatedFunction(n) => n.span(),
            Async(n) => n.span(),
            Binary(n) => n.span(),
            Call(n) => n.span(),
            Cast(n) => n.span(),
            Err(n) => n.span(),
            Identifier(n) => n.span(),
            Literal(n) => n.span(),
            Locator(n) => n.span(),
            MemberAccess(n) => n.span(),
            Repeat(n) => n.span(),
            Struct(n) => n.span(),
            Ternary(n) => n.span(),
            Tuple(n) => n.span(),
            TupleAccess(n) => n.span(),
            Unary(n) => n.span(),
            Unit(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Expression::*;
        match self {
            ArrayAccess(n) => n.set_span(span),
            Array(n) => n.set_span(span),
            AssociatedConstant(n) => n.set_span(span),
            AssociatedFunction(n) => n.set_span(span),
            Async(n) => n.set_span(span),
            Binary(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            Cast(n) => n.set_span(span),
            Err(n) => n.set_span(span),
            Identifier(n) => n.set_span(span),
            Literal(n) => n.set_span(span),
            Locator(n) => n.set_span(span),
            MemberAccess(n) => n.set_span(span),
            Repeat(n) => n.set_span(span),
            Struct(n) => n.set_span(span),
            Ternary(n) => n.set_span(span),
            Tuple(n) => n.set_span(span),
            TupleAccess(n) => n.set_span(span),
            Unary(n) => n.set_span(span),
            Unit(n) => n.set_span(span),
        }
    }

    fn id(&self) -> NodeID {
        use Expression::*;
        match self {
            Array(n) => n.id(),
            ArrayAccess(n) => n.id(),
            AssociatedConstant(n) => n.id(),
            AssociatedFunction(n) => n.id(),
            Async(n) => n.id(),
            Binary(n) => n.id(),
            Call(n) => n.id(),
            Cast(n) => n.id(),
            Identifier(n) => n.id(),
            Literal(n) => n.id(),
            Locator(n) => n.id(),
            MemberAccess(n) => n.id(),
            Repeat(n) => n.id(),
            Err(n) => n.id(),
            Struct(n) => n.id(),
            Ternary(n) => n.id(),
            Tuple(n) => n.id(),
            TupleAccess(n) => n.id(),
            Unary(n) => n.id(),
            Unit(n) => n.id(),
        }
    }

    fn set_id(&mut self, id: NodeID) {
        use Expression::*;
        match self {
            Array(n) => n.set_id(id),
            ArrayAccess(n) => n.set_id(id),
            AssociatedConstant(n) => n.set_id(id),
            AssociatedFunction(n) => n.set_id(id),
            Async(n) => n.set_id(id),
            Binary(n) => n.set_id(id),
            Call(n) => n.set_id(id),
            Cast(n) => n.set_id(id),
            Identifier(n) => n.set_id(id),
            Literal(n) => n.set_id(id),
            Locator(n) => n.set_id(id),
            MemberAccess(n) => n.set_id(id),
            Repeat(n) => n.set_id(id),
            Err(n) => n.set_id(id),
            Struct(n) => n.set_id(id),
            Ternary(n) => n.set_id(id),
            Tuple(n) => n.set_id(id),
            TupleAccess(n) => n.set_id(id),
            Unary(n) => n.set_id(id),
            Unit(n) => n.set_id(id),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expression::*;
        match &self {
            Array(n) => n.fmt(f),
            ArrayAccess(n) => n.fmt(f),
            AssociatedConstant(n) => n.fmt(f),
            AssociatedFunction(n) => n.fmt(f),
            Async(n) => n.fmt(f),
            Binary(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            Cast(n) => n.fmt(f),
            Err(n) => n.fmt(f),
            Identifier(n) => n.fmt(f),
            Literal(n) => n.fmt(f),
            Locator(n) => n.fmt(f),
            MemberAccess(n) => n.fmt(f),
            Repeat(n) => n.fmt(f),
            Struct(n) => n.fmt(f),
            Ternary(n) => n.fmt(f),
            Tuple(n) => n.fmt(f),
            TupleAccess(n) => n.fmt(f),
            Unary(n) => n.fmt(f),
            Unit(n) => n.fmt(f),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Associativity {
    Left,
    Right,
    None,
}

impl Expression {
    pub(crate) fn precedence(&self) -> u32 {
        use Expression::*;
        match self {
            Binary(e) => e.precedence(),
            Cast(_) => 12,
            Ternary(_) => 14,
            Array(_)
            | ArrayAccess(_)
            | AssociatedConstant(_)
            | AssociatedFunction(_)
            | Async(_)
            | Call(_)
            | Err(_)
            | Identifier(_)
            | Literal(_)
            | Locator(_)
            | MemberAccess(_)
            | Repeat(_)
            | Struct(_)
            | Tuple(_)
            | TupleAccess(_)
            | Unary(_)
            | Unit(_) => 20,
        }
    }

    pub(crate) fn associativity(&self) -> Associativity {
        if let Expression::Binary(bin) = self { bin.associativity() } else { Associativity::None }
    }

    /// Returns `self` as a known `u32` if possible. Otherwise, returns a `None`. This allows for large and/or signed
    /// types but only if they can be safely cast to a `u32`.
    pub fn as_u32(&self) -> Option<u32> {
        if let Expression::Literal(literal) = &self {
            if let LiteralVariant::Integer(int_type, s, ..) = &literal.variant {
                use crate::IntegerType::*;
                let s = s.replace("_", "");

                return match int_type {
                    U8 => u8::from_str_by_radix(&s).map(|v| v as u32).ok(),
                    U16 => u16::from_str_by_radix(&s).map(|v| v as u32).ok(),
                    U32 => u32::from_str_by_radix(&s).ok(),
                    U64 => u64::from_str_by_radix(&s).ok().and_then(|v| u32::try_from(v).ok()),
                    U128 => u128::from_str_by_radix(&s).ok().and_then(|v| u32::try_from(v).ok()),
                    I8 => i8::from_str_by_radix(&s).ok().and_then(|v| u32::try_from(v).ok()),
                    I16 => i16::from_str_by_radix(&s).ok().and_then(|v| u32::try_from(v).ok()),
                    I32 => i32::from_str_by_radix(&s).ok().and_then(|v| u32::try_from(v).ok()),
                    I64 => i64::from_str_by_radix(&s).ok().and_then(|v| u32::try_from(v).ok()),
                    I128 => i128::from_str_by_radix(&s).ok().and_then(|v| u32::try_from(v).ok()),
                };
            } else if let LiteralVariant::Unsuffixed(s) = &literal.variant {
                // Assume unsuffixed literals are `u32`. The type checker should enforce that as the default type.
                let s = s.replace("_", "");
                return u32::from_str_by_radix(&s).ok();
            }
        }
        None
    }
}
