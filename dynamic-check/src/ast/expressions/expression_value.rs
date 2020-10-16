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
    expressions::array::{RangeOrExpression, SpreadOrExpression},
    CircuitVariableDefinition,
    Expression,
};
use leo_typed::{GroupValue, Identifier, IntegerType, Span};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressionValue {
    // Identifier
    Identifier(Identifier),

    // Values
    Address(String, Span),
    Boolean(String, Span),
    Field(String, Span),
    Group(GroupValue),
    Integer(IntegerType, String, Span),

    // Arithmetic operations
    Add(Box<Expression>, Box<Expression>, Span),
    Sub(Box<Expression>, Box<Expression>, Span),
    Mul(Box<Expression>, Box<Expression>, Span),
    Div(Box<Expression>, Box<Expression>, Span),
    Pow(Box<Expression>, Box<Expression>, Span),
    Negate(Box<Expression>, Span),

    // Logical operations
    And(Box<Expression>, Box<Expression>, Span),
    Or(Box<Expression>, Box<Expression>, Span),
    Not(Box<Expression>, Span),

    // Relational operations
    Eq(Box<Expression>, Box<Expression>, Span),
    Ge(Box<Expression>, Box<Expression>, Span),
    Gt(Box<Expression>, Box<Expression>, Span),
    Le(Box<Expression>, Box<Expression>, Span),
    Lt(Box<Expression>, Box<Expression>, Span),

    // Conditionals
    // (conditional, first_value, second_value, span)
    IfElse(Box<Expression>, Box<Expression>, Box<Expression>, Span),

    // Arrays
    // (array_elements, span)
    Array(Vec<Box<SpreadOrExpression>>, Span),
    // (array_name, range, span)
    ArrayAccess(Box<Expression>, Box<RangeOrExpression>, Span),

    // Tuples
    // (tuple_elements, span)
    Tuple(Vec<Expression>, Span),
    // (tuple_name, index, span)
    TupleAccess(Box<Expression>, usize, Span),

    // Circuits
    // (defined_circuit_name, circuit_members, span)
    Circuit(Identifier, Vec<CircuitVariableDefinition>, Span),
    // (declared_circuit name, circuit_member_name, span)
    CircuitMemberAccess(Box<Expression>, Identifier, Span),
    // (defined_circuit name, circuit_static_function_name, span)
    CircuitStaticFunctionAccess(Box<Expression>, Identifier, Span),

    // Functions
    // (declared_function_name, function_arguments, span)
    FunctionCall(Box<Expression>, Vec<Expression>, Span),
    // (core_function_name, function_arguments, span)
    CoreFunctionCall(String, Vec<Expression>, Span),
}

impl ExpressionValue {
    /// Return the span
    pub fn span(&self) -> &Span {
        match self {
            ExpressionValue::Identifier(identifier) => &identifier.span,
            ExpressionValue::Address(_, span) => span,
            ExpressionValue::Boolean(_, span) => span,
            ExpressionValue::Field(_, span) => span,
            ExpressionValue::Group(group_value) => group_value.span(),
            ExpressionValue::Integer(_type, _, span) => span,

            ExpressionValue::Add(_, _, span) => span,
            ExpressionValue::Sub(_, _, span) => span,
            ExpressionValue::Mul(_, _, span) => span,
            ExpressionValue::Div(_, _, span) => span,
            ExpressionValue::Pow(_, _, span) => span,
            ExpressionValue::Negate(_, span) => span,

            ExpressionValue::And(_, _, span) => span,
            ExpressionValue::Or(_, _, span) => span,
            ExpressionValue::Not(_, span) => span,

            ExpressionValue::Eq(_, _, span) => span,
            ExpressionValue::Ge(_, _, span) => span,
            ExpressionValue::Gt(_, _, span) => span,
            ExpressionValue::Le(_, _, span) => span,
            ExpressionValue::Lt(_, _, span) => span,

            ExpressionValue::IfElse(_, _, _, span) => span,

            ExpressionValue::Array(_, span) => span,
            ExpressionValue::ArrayAccess(_, _, span) => span,

            ExpressionValue::Tuple(_, span) => span,
            ExpressionValue::TupleAccess(_, _, span) => span,

            ExpressionValue::Circuit(_, _, span) => span,
            ExpressionValue::CircuitMemberAccess(_, _, span) => span,
            ExpressionValue::CircuitStaticFunctionAccess(_, _, span) => span,

            ExpressionValue::FunctionCall(_, _, span) => span,
            ExpressionValue::CoreFunctionCall(_, _, span) => span,
        }
    }
}
