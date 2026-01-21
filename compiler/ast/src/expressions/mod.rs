// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{Identifier, IntegerType, Intrinsic, Node, NodeBuilder, NodeID, Path, Type};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::fmt;

mod array_access;
pub use array_access::*;

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

mod composite_init;
pub use composite_init::*;

mod err;
pub use err::*;

mod member_access;
pub use member_access::*;

mod intrinsic;
pub use intrinsic::*;

mod repeat;
pub use repeat::*;

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
    /// An `async` block: e.g. `async { my_mapping.set(1, 2); }`.
    Async(AsyncExpression),
    /// An array expression, e.g., `[true, false, true, false]`.
    Array(ArrayExpression),
    /// A binary expression, e.g., `42 + 24`.
    Binary(Box<BinaryExpression>),
    /// An intrinsic expression, e.g., `_my_intrinsic(args)`.
    Intrinsic(Box<IntrinsicExpression>),
    /// A call expression, e.g., `my_fun(args)`.
    Call(Box<CallExpression>),
    /// A cast expression, e.g., `42u32 as u8`.
    Cast(Box<CastExpression>),
    /// An expression of type "error".
    /// Will result in a compile error eventually.
    /// An expression constructing a composite like `Foo { bar: 42, baz }`.
    Composite(CompositeExpression),
    Err(ErrExpression),
    /// A path to some item, e.g., `foo::bar::x`.
    Path(Path),
    /// A literal expression.
    Literal(Literal),
    /// A locator expression, e.g., `hello.aleo/foo`.
    Locator(LocatorExpression),
    /// An access of a composite member, e.g. `composite.member`.
    MemberAccess(Box<MemberAccess>),
    /// An array expression constructed from one repeated element, e.g., `[1u32; 5]`.
    Repeat(Box<RepeatExpression>),
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
            Async(n) => n.span(),
            Binary(n) => n.span(),
            Call(n) => n.span(),
            Cast(n) => n.span(),
            Composite(n) => n.span(),
            Err(n) => n.span(),
            Intrinsic(n) => n.span(),
            Path(n) => n.span(),
            Literal(n) => n.span(),
            Locator(n) => n.span(),
            MemberAccess(n) => n.span(),
            Repeat(n) => n.span(),
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
            Async(n) => n.set_span(span),
            Binary(n) => n.set_span(span),
            Call(n) => n.set_span(span),
            Cast(n) => n.set_span(span),
            Composite(n) => n.set_span(span),
            Err(n) => n.set_span(span),
            Intrinsic(n) => n.set_span(span),
            Path(n) => n.set_span(span),
            Literal(n) => n.set_span(span),
            Locator(n) => n.set_span(span),
            MemberAccess(n) => n.set_span(span),
            Repeat(n) => n.set_span(span),
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
            Async(n) => n.id(),
            Binary(n) => n.id(),
            Call(n) => n.id(),
            Cast(n) => n.id(),
            Composite(n) => n.id(),
            Path(n) => n.id(),
            Literal(n) => n.id(),
            Locator(n) => n.id(),
            MemberAccess(n) => n.id(),
            Repeat(n) => n.id(),
            Err(n) => n.id(),
            Intrinsic(n) => n.id(),
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
            Async(n) => n.set_id(id),
            Binary(n) => n.set_id(id),
            Call(n) => n.set_id(id),
            Cast(n) => n.set_id(id),
            Composite(n) => n.set_id(id),
            Path(n) => n.set_id(id),
            Literal(n) => n.set_id(id),
            Locator(n) => n.set_id(id),
            MemberAccess(n) => n.set_id(id),
            Repeat(n) => n.set_id(id),
            Err(n) => n.set_id(id),
            Intrinsic(n) => n.set_id(id),
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
            Async(n) => n.fmt(f),
            Binary(n) => n.fmt(f),
            Call(n) => n.fmt(f),
            Cast(n) => n.fmt(f),
            Composite(n) => n.fmt(f),
            Err(n) => n.fmt(f),
            Intrinsic(n) => n.fmt(f),
            Path(n) => n.fmt(f),
            Literal(n) => n.fmt(f),
            Locator(n) => n.fmt(f),
            MemberAccess(n) => n.fmt(f),
            Repeat(n) => n.fmt(f),
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
            Ternary(_) => 0,
            Array(_) | ArrayAccess(_) | Async(_) | Call(_) | Composite(_) | Err(_) | Intrinsic(_) | Path(_)
            | Literal(_) | Locator(_) | MemberAccess(_) | Repeat(_) | Tuple(_) | TupleAccess(_) | Unary(_)
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

    pub fn is_none_expr(&self) -> bool {
        matches!(self, Expression::Literal(Literal { variant: LiteralVariant::None, .. }))
    }

    /// Returns true if we can confidently say evaluating this expression has no side effects, false otherwise
    pub fn is_pure(&self, get_type: &impl Fn(NodeID) -> Type) -> bool {
        match self {
            // Discriminate intrinsics
            Expression::Intrinsic(intr) => {
                if let Some(intrinsic) = Intrinsic::from_symbol(intr.name, &intr.type_parameters) {
                    intrinsic.is_pure()
                } else {
                    false
                }
            }

            // We may be indirectly referring to an impure item
            // This analysis could be more granular
            Expression::Call(..) | Expression::Err(..) | Expression::Async(..) | Expression::Cast(..) => false,

            Expression::Binary(expr) => {
                use BinaryOperation::*;
                match expr.op {
                    // These can halt for any of their operand types.
                    Div | Mod | Rem | Shl | Shr => false,
                    // These can only halt for integers.
                    Add | Mul | Pow => !matches!(get_type(expr.id()), Type::Integer(..)),
                    _ => expr.left.is_pure(get_type) && expr.right.is_pure(get_type),
                }
            }
            Expression::Unary(expr) => {
                use UnaryOperation::*;
                match expr.op {
                    // These can halt for any of their operand types.
                    Abs | Inverse | SquareRoot => false,
                    // Negate can only halt for integers.
                    Negate => !matches!(get_type(expr.id()), Type::Integer(..)),
                    _ => expr.receiver.is_pure(get_type),
                }
            }

            // Always pure
            Expression::Locator(..) | Expression::Literal(..) | Expression::Path(..) | Expression::Unit(..) => true,

            // Recurse
            Expression::ArrayAccess(expr) => expr.array.is_pure(get_type) && expr.index.is_pure(get_type),
            Expression::MemberAccess(expr) => expr.inner.is_pure(get_type),
            Expression::Repeat(expr) => expr.expr.is_pure(get_type) && expr.count.is_pure(get_type),
            Expression::TupleAccess(expr) => expr.tuple.is_pure(get_type),
            Expression::Array(expr) => expr.elements.iter().all(|e| e.is_pure(get_type)),
            Expression::Composite(expr) => {
                expr.const_arguments.iter().all(|e| e.is_pure(get_type))
                    && expr.members.iter().all(|init| init.expression.as_ref().is_none_or(|e| e.is_pure(get_type)))
            }
            Expression::Ternary(expr) => {
                expr.condition.is_pure(get_type) && expr.if_true.is_pure(get_type) && expr.if_false.is_pure(get_type)
            }
            Expression::Tuple(expr) => expr.elements.iter().all(|e| e.is_pure(get_type)),
        }
    }

    /// Returns the *zero value expression* for a given type, if one exists.
    ///
    /// This is used during lowering and reconstruction to provide default or
    /// placeholder values (e.g., for `get_or_use` calls or composite initialization).
    ///
    /// Supported types:
    /// - **Integers** (`i8`–`i128`, `u8`–`u128`): literal `0`
    /// - **Boolean**: literal `false`
    /// - **Field**, **Group**, **Scalar**: zero literals `"0"`
    /// - **Composites**: recursively constructs a composite with all members zeroed
    /// - **Arrays**: repeats a zero element for the array length
    ///
    /// Returns `None` if the type has no well-defined zero representation
    /// (e.g. mapping, Future).
    ///
    /// The `composite_lookup` callback provides member definitions for composite types.
    #[allow(clippy::type_complexity)]
    pub fn zero(
        ty: &Type,
        span: Span,
        node_builder: &NodeBuilder,
        composite_lookup: &dyn Fn(&[Symbol]) -> Vec<(Symbol, Type)>,
    ) -> Option<Self> {
        let id = node_builder.next_id();

        match ty {
            // Numeric types
            Type::Integer(IntegerType::I8) => Some(Literal::integer(IntegerType::I8, "0".to_string(), span, id).into()),
            Type::Integer(IntegerType::I16) => {
                Some(Literal::integer(IntegerType::I16, "0".to_string(), span, id).into())
            }
            Type::Integer(IntegerType::I32) => {
                Some(Literal::integer(IntegerType::I32, "0".to_string(), span, id).into())
            }
            Type::Integer(IntegerType::I64) => {
                Some(Literal::integer(IntegerType::I64, "0".to_string(), span, id).into())
            }
            Type::Integer(IntegerType::I128) => {
                Some(Literal::integer(IntegerType::I128, "0".to_string(), span, id).into())
            }
            Type::Integer(IntegerType::U8) => Some(Literal::integer(IntegerType::U8, "0".to_string(), span, id).into()),
            Type::Integer(IntegerType::U16) => {
                Some(Literal::integer(IntegerType::U16, "0".to_string(), span, id).into())
            }
            Type::Integer(IntegerType::U32) => {
                Some(Literal::integer(IntegerType::U32, "0".to_string(), span, id).into())
            }
            Type::Integer(IntegerType::U64) => {
                Some(Literal::integer(IntegerType::U64, "0".to_string(), span, id).into())
            }
            Type::Integer(IntegerType::U128) => {
                Some(Literal::integer(IntegerType::U128, "0".to_string(), span, id).into())
            }

            // Boolean
            Type::Boolean => Some(Literal::boolean(false, span, id).into()),

            // Address: addresses don't have a well defined _zero_ but this value is often used as
            // the "zero" address in practical applications. It really should never be used directly though.
            // It should only be used as a placeholder for representating `none` for example.
            Type::Address => Some(
                Literal::address(
                    "aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc".to_string(),
                    span,
                    id,
                )
                .into(),
            ),

            // Field, Group, Scalar
            Type::Field => Some(Literal::field("0".to_string(), span, id).into()),
            Type::Group => Some(Literal::group("0".to_string(), span, id).into()),
            Type::Scalar => Some(Literal::scalar("0".to_string(), span, id).into()),

            // Composite types
            Type::Composite(composite_type) => {
                let path = &composite_type.path;
                let members = composite_lookup(&path.expect_global_location().path);

                let composite_members = members
                    .into_iter()
                    .map(|(symbol, member_type)| {
                        let member_id = node_builder.next_id();
                        let zero_expr = Self::zero(&member_type, span, node_builder, composite_lookup)?;

                        Some(CompositeFieldInitializer {
                            span,
                            id: member_id,
                            identifier: Identifier::new(symbol, node_builder.next_id()),
                            expression: Some(zero_expr),
                        })
                    })
                    .collect::<Option<Vec<_>>>()?;

                Some(Expression::Composite(CompositeExpression {
                    span,
                    id,
                    path: path.clone(),
                    const_arguments: composite_type.const_arguments.clone(),
                    members: composite_members,
                }))
            }

            // Arrays
            Type::Array(array_type) => {
                let element_ty = &array_type.element_type;

                let element_expr = Self::zero(element_ty, span, node_builder, composite_lookup)?;

                Some(Expression::Repeat(
                    RepeatExpression { span, id, expr: element_expr, count: *array_type.length.clone() }.into(),
                ))
            }

            // Other types are not expected or supported just yet
            _ => None,
        }
    }
}
