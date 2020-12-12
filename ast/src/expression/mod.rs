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

use crate::{
    ArrayDimensions,
    CircuitVariableDefinition,
    GroupValue,
    Identifier,
    IntegerType,
    PositiveNumber,
    Span,
    SpreadOrExpression,
};
use leo_grammar::{
    access::{Access, AssigneeAccess},
    common::{Assignee, Identifier as GrammarIdentifier, RangeOrExpression as GrammarRangeOrExpression},
    expressions::{
        ArrayInitializerExpression,
        ArrayInlineExpression as GrammarArrayInlineExpression,
        BinaryExpression as GrammarBinaryExpression,
        CircuitInlineExpression,
        Expression as GrammarExpression,
        PostfixExpression,
        TernaryExpression,
        UnaryExpression as GrammarUnaryExpression,
    },
    operations::{BinaryOperation as GrammarBinaryOperation, UnaryOperation as GrammarUnaryOperation},
    values::{
        AddressValue,
        BooleanValue,
        FieldValue,
        GroupValue as GrammarGroupValue,
        IntegerValue,
        NumberValue as GrammarNumber,
        Value,
    },
};

use leo_grammar::{access::TupleAccess, expressions::TupleExpression};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::Node;

mod binary;
pub use binary::*;
mod unary;
pub use unary::*;
mod conditional;
pub use conditional::*;
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

/// Expression that evaluates to a value
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    Identifier(Identifier),
    Value(ValueExpression),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Conditional(ConditionalExpression),

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
            Conditional(n) => n.span(),
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
        }
    }

    fn set_span(&mut self, span: Span) {
        use Expression::*;
        match self {
            Identifier(n) => n.set_span(span),
            Value(n) => n.set_span(span),
            Binary(n) => n.set_span(span),
            Unary(n) => n.set_span(span),
            Conditional(n) => n.set_span(span),
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
        }
    }
}

impl<'ast> fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expression::*;
        match &self {
            Identifier(n) => n.fmt(f),
            Value(n) => n.fmt(f),
            Binary(n) => n.fmt(f),
            Unary(n) => n.fmt(f),
            Conditional(n) => n.fmt(f),
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
        }
    }
}

impl<'ast> From<CircuitInlineExpression<'ast>> for Expression {
    fn from(expression: CircuitInlineExpression<'ast>) -> Self {
        let circuit_name = Identifier::from(expression.name);
        let members = expression
            .members
            .into_iter()
            .map(CircuitVariableDefinition::from)
            .collect::<Vec<CircuitVariableDefinition>>();

        Expression::CircuitInit(CircuitInitExpression {
            name: circuit_name,
            members,
            span: Span::from(expression.span),
        })
    }
}

impl<'ast> From<PostfixExpression<'ast>> for Expression {
    fn from(expression: PostfixExpression<'ast>) -> Self {
        let variable = Expression::Identifier(Identifier::from(expression.name));

        // ast::PostFixExpression contains an array of "accesses": `a(34)[42]` is represented as `[a, [Call(34), Select(42)]]`, but Access call expressions
        // are recursive, so it is `Select(Call(a, 34), 42)`. We apply this transformation here

        // we start with the id, and we fold the array of accesses by wrapping the current value
        expression
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                // Handle array accesses
                Access::Array(array) => match array.expression {
                    GrammarRangeOrExpression::Expression(expression) => {
                        Expression::ArrayAccess(ArrayAccessExpression {
                            array: Box::new(acc),
                            index: Box::new(Expression::from(expression)),
                            span: Span::from(array.span),
                        })
                    }
                    GrammarRangeOrExpression::Range(range) => {
                        Expression::ArrayRangeAccess(ArrayRangeAccessExpression {
                            array: Box::new(acc),
                            left: range.from.map(Expression::from).map(Box::new),
                            right: range.to.map(Expression::from).map(Box::new),
                            span: Span::from(array.span),
                        })
                    }
                },

                // Handle tuple access
                Access::Tuple(tuple) => Expression::TupleAccess(TupleAccessExpression {
                    tuple: Box::new(acc),
                    index: PositiveNumber::from(tuple.number),
                    span: Span::from(tuple.span),
                }),

                // Handle function calls
                Access::Call(function) => Expression::Call(CallExpression {
                    function: Box::new(acc),
                    arguments: function.expressions.into_iter().map(Expression::from).collect(),
                    span: Span::from(function.span),
                }),

                // Handle circuit member accesses
                Access::Object(circuit_object) => Expression::CircuitMemberAccess(CircuitMemberAccessExpression {
                    circuit: Box::new(acc),
                    name: Identifier::from(circuit_object.identifier),
                    span: Span::from(circuit_object.span),
                }),
                Access::StaticObject(circuit_object) => {
                    Expression::CircuitStaticFunctionAccess(CircuitStaticFunctionAccessExpression {
                        circuit: Box::new(acc),
                        name: Identifier::from(circuit_object.identifier),
                        span: Span::from(circuit_object.span),
                    })
                }
            })
    }
}

impl<'ast> From<GrammarExpression<'ast>> for Expression {
    fn from(expression: GrammarExpression<'ast>) -> Self {
        match expression {
            GrammarExpression::Value(value) => Expression::from(value),
            GrammarExpression::Identifier(variable) => Expression::from(variable),
            GrammarExpression::Unary(expression) => Expression::from(*expression),
            GrammarExpression::Binary(expression) => Expression::from(*expression),
            GrammarExpression::Ternary(expression) => Expression::from(*expression),
            GrammarExpression::ArrayInline(expression) => Expression::from(expression),
            GrammarExpression::ArrayInitializer(expression) => Expression::from(*expression),
            GrammarExpression::Tuple(expression) => Expression::from(expression),
            GrammarExpression::CircuitInline(expression) => Expression::from(expression),
            GrammarExpression::Postfix(expression) => Expression::from(expression),
        }
    }
}

// Assignee -> Expression for operator assign statements
impl<'ast> From<Assignee<'ast>> for Expression {
    fn from(assignee: Assignee<'ast>) -> Self {
        let variable = Expression::Identifier(Identifier::from(assignee.name));

        // we start with the id, and we fold the array of accesses by wrapping the current value
        assignee
            .accesses
            .into_iter()
            .fold(variable, |acc, access| match access {
                AssigneeAccess::Member(circuit_member) => {
                    Expression::CircuitMemberAccess(CircuitMemberAccessExpression {
                        circuit: Box::new(acc),
                        name: Identifier::from(circuit_member.identifier),
                        span: Span::from(circuit_member.span),
                    })
                }
                AssigneeAccess::Array(array) => match array.expression {
                    GrammarRangeOrExpression::Expression(expression) => {
                        Expression::ArrayAccess(ArrayAccessExpression {
                            array: Box::new(acc),
                            index: Box::new(Expression::from(expression)),
                            span: Span::from(array.span),
                        })
                    }
                    GrammarRangeOrExpression::Range(range) => {
                        Expression::ArrayRangeAccess(ArrayRangeAccessExpression {
                            array: Box::new(acc),
                            left: range.from.map(Expression::from).map(Box::new),
                            right: range.to.map(Expression::from).map(Box::new),
                            span: Span::from(array.span),
                        })
                    }
                },
                AssigneeAccess::Tuple(tuple) => Expression::TupleAccess(TupleAccessExpression {
                    tuple: Box::new(acc),
                    index: PositiveNumber::from(tuple.number),
                    span: Span::from(tuple.span.clone()),
                }),
            })
    }
}

impl<'ast> From<GrammarBinaryExpression<'ast>> for Expression {
    fn from(expression: GrammarBinaryExpression<'ast>) -> Self {
        use GrammarBinaryOperation::*;
        let operator = match expression.operation {
            Or => BinaryOperation::Or,
            And => BinaryOperation::And,
            Eq => BinaryOperation::Eq,
            Ne => BinaryOperation::Ne,
            Ge => BinaryOperation::Ge,
            Gt => BinaryOperation::Gt,
            Le => BinaryOperation::Le,
            Lt => BinaryOperation::Lt,
            Add => BinaryOperation::Add,
            Sub => BinaryOperation::Sub,
            Mul => BinaryOperation::Mul,
            Div => BinaryOperation::Div,
            Pow => BinaryOperation::Pow,
        };
        Expression::Binary(BinaryExpression {
            left: Box::new(Expression::from(expression.left)),
            right: Box::new(Expression::from(expression.right)),
            op: operator,
            span: Span::from(expression.span),
        })
    }
}

impl<'ast> From<TernaryExpression<'ast>> for Expression {
    fn from(expression: TernaryExpression<'ast>) -> Self {
        Expression::Conditional(ConditionalExpression {
            condition: Box::new(Expression::from(expression.first)),
            if_true: Box::new(Expression::from(expression.second)),
            if_false: Box::new(Expression::from(expression.third)),
            span: Span::from(expression.span),
        })
    }
}

impl<'ast> From<GrammarArrayInlineExpression<'ast>> for Expression {
    fn from(array: GrammarArrayInlineExpression<'ast>) -> Self {
        Expression::ArrayInline(ArrayInlineExpression {
            elements: array.expressions.into_iter().map(SpreadOrExpression::from).collect(),
            span: Span::from(array.span),
        })
    }
}

impl<'ast> From<ArrayInitializerExpression<'ast>> for Expression {
    fn from(array: ArrayInitializerExpression<'ast>) -> Self {
        Expression::ArrayInit(ArrayInitExpression {
            element: Box::new(Expression::from(array.expression)),
            dimensions: ArrayDimensions::from(array.dimensions),
            span: Span::from(array.span),
        })
    }
}

impl<'ast> From<TupleExpression<'ast>> for Expression {
    fn from(tuple: TupleExpression<'ast>) -> Self {
        Expression::TupleInit(TupleInitExpression {
            elements: tuple.expressions.into_iter().map(Expression::from).collect(),
            span: Span::from(tuple.span),
        })
    }
}

impl<'ast> From<Value<'ast>> for Expression {
    fn from(value: Value<'ast>) -> Self {
        match value {
            Value::Address(address) => Expression::from(address),
            Value::Boolean(boolean) => Expression::from(boolean),
            Value::Field(field) => Expression::from(field),
            Value::Group(group) => Expression::from(group),
            Value::Implicit(number) => Expression::from(number),
            Value::Integer(integer) => Expression::from(integer),
        }
    }
}

impl<'ast> From<GrammarUnaryExpression<'ast>> for Expression {
    fn from(expression: GrammarUnaryExpression<'ast>) -> Self {
        use GrammarUnaryOperation::*;
        let operator = match expression.operation {
            Not(_) => UnaryOperation::Not,
            Negate(_) => UnaryOperation::Negate,
        };
        Expression::Unary(UnaryExpression {
            inner: Box::new(Expression::from(expression.expression)),
            op: operator,
            span: Span::from(expression.span),
        })
    }
}

impl<'ast> From<AddressValue<'ast>> for Expression {
    fn from(address: AddressValue<'ast>) -> Self {
        Expression::Value(ValueExpression::Address(
            address.address.value,
            Span::from(address.span),
        ))
    }
}

impl<'ast> From<BooleanValue<'ast>> for Expression {
    fn from(boolean: BooleanValue<'ast>) -> Self {
        Expression::Value(ValueExpression::Boolean(boolean.value, Span::from(boolean.span)))
    }
}

impl<'ast> From<FieldValue<'ast>> for Expression {
    fn from(field: FieldValue<'ast>) -> Self {
        Expression::Value(ValueExpression::Field(field.number.to_string(), Span::from(field.span)))
    }
}

impl<'ast> From<GrammarGroupValue<'ast>> for Expression {
    fn from(ast_group: GrammarGroupValue<'ast>) -> Self {
        Expression::Value(ValueExpression::Group(Box::new(GroupValue::from(ast_group))))
    }
}

impl<'ast> From<GrammarNumber<'ast>> for Expression {
    fn from(number: GrammarNumber<'ast>) -> Self {
        let (value, span) = match number {
            GrammarNumber::Positive(number) => (number.value, number.span),
            GrammarNumber::Negative(number) => (number.value, number.span),
        };

        Expression::Value(ValueExpression::Implicit(value, Span::from(span)))
    }
}

impl<'ast> From<IntegerValue<'ast>> for Expression {
    fn from(integer: IntegerValue<'ast>) -> Self {
        let span = Span::from(integer.span().clone());
        let (type_, value) = match integer {
            IntegerValue::Signed(integer) => {
                let type_ = IntegerType::from(integer.type_);
                let number = match integer.number {
                    GrammarNumber::Negative(number) => number.value,
                    GrammarNumber::Positive(number) => number.value,
                };

                (type_, number)
            }
            IntegerValue::Unsigned(integer) => {
                let type_ = IntegerType::from(integer.type_);
                let number = integer.number.value;

                (type_, number)
            }
        };

        Expression::Value(ValueExpression::Integer(type_, value, span))
    }
}

impl<'ast> From<TupleAccess<'ast>> for Expression {
    fn from(tuple: TupleAccess<'ast>) -> Self {
        Expression::Value(ValueExpression::Implicit(
            tuple.number.to_string(),
            Span::from(tuple.span),
        ))
    }
}

impl<'ast> From<GrammarIdentifier<'ast>> for Expression {
    fn from(identifier: GrammarIdentifier<'ast>) -> Self {
        Expression::Identifier(Identifier::from(identifier))
    }
}
