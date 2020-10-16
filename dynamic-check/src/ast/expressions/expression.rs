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

use crate::{ExpressionError, ExpressionValue, FunctionBody, ResolvedNode, VariableTable};
use leo_static_check::{SymbolTable, Type};
use leo_typed::{Expression as UnresolvedExpression, Span};

use serde::{Deserialize, Serialize};

/// Stores a type-checked expression
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expression {
    /// The type this expression evaluates to
    pub(crate) type_: Type,
    /// The value of this expression
    pub(crate) value: ExpressionValue,
}

impl Expression {
    /// Return the type this expression evaluates to
    pub fn type_(&self) -> &Type {
        &self.type_
    }

    /// Return the span
    pub fn span(&self) -> &Span {
        self.value.span()
    }

    /// Returns `Ok` if this expression resolves to an integer type
    pub fn check_type_integer(&self) -> Result<(), ExpressionError> {
        self.type_.check_type_integer(self.value.span().clone())?;

        Ok(())
    }

    ///
    /// Returns a new `Expression` from a given `UnresolvedExpression`.
    ///
    /// Performs a lookup in the given function body's variable table if the expression contains
    /// user-defined variables.
    ///
    pub fn new(
        function_body: &FunctionBody,
        unresolved_expression: UnresolvedExpression,
    ) -> Result<Self, ExpressionError> {
        match unresolved_expression {
            // Identifier
            UnresolvedExpression::Identifier(identifier) => Self::variable(function_body, identifier),

            // Values
            UnresolvedExpression::Address(string, span) => Self::address(string, span),
            UnresolvedExpression::Boolean(string, span) => Self::boolean(string, span),
            UnresolvedExpression::Field(string, span) => Self::field(string, span),
            UnresolvedExpression::Group(group_value) => Self::group(group_value),
            UnresolvedExpression::Implicit(string, span) => Self::implicit(string, span),
            UnresolvedExpression::Integer(integer_type, string, span) => Self::integer(integer_type, string, span),

            // Arithmetic Operations
            UnresolvedExpression::Add(lhs, rhs, span) => Self::add(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Sub(lhs, rhs, span) => Self::sub(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Mul(lhs, rhs, span) => Self::mul(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Div(lhs, rhs, span) => Self::div(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Pow(lhs, rhs, span) => Self::pow(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Negate(expression, span) => Self::negate(variable_table, *expression, span),

            // Logical Operations
            UnresolvedExpression::And(lhs, rhs, span) => Self::and(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Or(lhs, rhs, span) => Self::or(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Not(expression, span) => Self::not(variable_table, *expression, span),

            // Relational Operations
            UnresolvedExpression::Eq(lhs, rhs, span) => Self::eq(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Ge(lhs, rhs, span) => Self::ge(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Gt(lhs, rhs, span) => Self::gt(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Le(lhs, rhs, span) => Self::le(variable_table, *lhs, *rhs, span),
            UnresolvedExpression::Lt(lhs, rhs, span) => Self::lt(variable_table, *lhs, *rhs, span),

            // Conditionals
            UnresolvedExpression::IfElse(cond, first, second, span) => {
                Self::conditional(variable_table, *cond, *first, *second, span)
            }

            // Arrays
            UnresolvedExpression::Array(elements, span) => Self::array(variable_table, elements, span),
            UnresolvedExpression::ArrayAccess(array, access, span) => {
                Self::array_access(variable_table, array, access, span)
            }

            // Tuples
            UnresolvedExpression::Tuple(elements, span) => Self::tuple(variable_table, elements, span),
            UnresolvedExpression::TupleAccess(tuple, index, span) => {
                Self::tuple_access(variable_table, tuple, index, span)
            }

            // Circuits
            UnresolvedExpression::Circuit(identifier, variables, span) => {
                Self::circuit(variable_table, identifier, variables, span)
            }
            UnresolvedExpression::CircuitMemberAccess(circuit, member, span) => {
                Self::circuit_access(variable_table, circuit, member, span)
            }
            UnresolvedExpression::CircuitStaticFunctionAccess(circuit, member, span) => {
                Self::circuit_static_access(variable_table, circuit, member, span)
            }

            // Functions
            UnresolvedExpression::FunctionCall(function, inputs, span) => {
                Self::function_call(variable_table, function, inputs, span)
            }
            UnresolvedExpression::CoreFunctionCall(_name, _inputs, _span) => {
                unimplemented!("core function calls not type checked")
                // Self::core_function_call(variable_table, expected_type, function, inputs, span)
            }
        }
    }
}

impl ResolvedNode for Expression {
    type Error = ExpressionError;
    /// (expected type, unresolved expression)
    type UnresolvedNode = (Option<Type>, UnresolvedExpression);

    /// Type check an expression inside a program AST
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let expected_type = unresolved.0;
        let expression = unresolved.1;

        match expression {
            // Identifier
            UnresolvedExpression::Identifier(identifier) => Self::identifier(table, expected_type, identifier),

            // Values
            UnresolvedExpression::Address(string, span) => Self::address(expected_type, string, span),
            UnresolvedExpression::Boolean(string, span) => Self::boolean(expected_type, string, span),
            UnresolvedExpression::Field(string, span) => Self::field(expected_type, string, span),
            UnresolvedExpression::Group(group_value) => Self::group(expected_type, group_value),
            UnresolvedExpression::Implicit(string, span) => Self::implicit(expected_type, string, span),
            UnresolvedExpression::Integer(integer_type, string, span) => {
                Self::integer(expected_type, integer_type, string, span)
            }

            // Arithmetic Operations
            UnresolvedExpression::Add(lhs, rhs, span) => Self::add(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Sub(lhs, rhs, span) => Self::sub(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Mul(lhs, rhs, span) => Self::mul(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Div(lhs, rhs, span) => Self::div(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Pow(lhs, rhs, span) => Self::pow(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Negate(expression, span) => Self::negate(table, expected_type, *expression, span),

            // Logical Operations
            UnresolvedExpression::And(lhs, rhs, span) => Self::and(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Or(lhs, rhs, span) => Self::or(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Not(expression, span) => Self::not(table, expected_type, *expression, span),

            // Relational Operations
            UnresolvedExpression::Eq(lhs, rhs, span) => Self::eq(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Ge(lhs, rhs, span) => Self::ge(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Gt(lhs, rhs, span) => Self::gt(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Le(lhs, rhs, span) => Self::le(table, expected_type, *lhs, *rhs, span),
            UnresolvedExpression::Lt(lhs, rhs, span) => Self::lt(table, expected_type, *lhs, *rhs, span),

            // Conditionals
            UnresolvedExpression::IfElse(cond, first, second, span) => {
                Self::conditional(table, expected_type, *cond, *first, *second, span)
            }

            // Arrays
            UnresolvedExpression::Array(elements, span) => Self::array(table, expected_type, elements, span),
            UnresolvedExpression::ArrayAccess(array, access, span) => {
                Self::array_access(table, expected_type, array, access, span)
            }

            // Tuples
            UnresolvedExpression::Tuple(elements, span) => Self::tuple(table, expected_type, elements, span),
            UnresolvedExpression::TupleAccess(tuple, index, span) => {
                Self::tuple_access(table, expected_type, tuple, index, span)
            }

            // Circuits
            UnresolvedExpression::Circuit(identifier, variables, span) => {
                Self::circuit(table, expected_type, identifier, variables, span)
            }
            UnresolvedExpression::CircuitMemberAccess(circuit, member, span) => {
                Self::circuit_access(table, expected_type, circuit, member, span)
            }
            UnresolvedExpression::CircuitStaticFunctionAccess(circuit, member, span) => {
                Self::circuit_static_access(table, expected_type, circuit, member, span)
            }

            // Functions
            UnresolvedExpression::FunctionCall(function, inputs, span) => {
                Self::function_call(table, expected_type, function, inputs, span)
            }
            UnresolvedExpression::CoreFunctionCall(_name, _inputs, _span) => {
                unimplemented!("core function calls not type checked")
                // Self::core_function_call(table, expected_type, function, inputs, span)
            }
        }
    }
}
