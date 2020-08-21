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

//! Enforce constraints on an expression in a compiled Leo program.

use crate::{
    arithmetic::*,
    errors::ExpressionError,
    logical::*,
    program::ConstrainedProgram,
    relational::*,
    value::{boolean::input::new_bool_constant, implicit::*, ConstrainedValue},
    Address,
    FieldType,
    GroupType,
    Integer,
};
use leo_typed::{Expression, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        expected_type: Option<Type>,
        expression: Expression,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match expression {
            // Variables
            Expression::Identifier(unresolved_variable) => {
                self.evaluate_identifier(file_scope, function_scope, expected_type, unresolved_variable)
            }

            // Values
            Expression::Address(address, span) => Ok(ConstrainedValue::Address(Address::constant(address, span)?)),
            Expression::Boolean(boolean, span) => Ok(ConstrainedValue::Boolean(new_bool_constant(boolean, span)?)),
            Expression::Field(field, span) => Ok(ConstrainedValue::Field(FieldType::constant(field, span)?)),
            Expression::Group(group_element) => Ok(ConstrainedValue::Group(G::constant(group_element)?)),
            Expression::Implicit(value, span) => Ok(enforce_number_implicit(expected_type, value, span)?),
            Expression::Integer(type_, integer, span) => {
                Ok(ConstrainedValue::Integer(Integer::new_constant(&type_, integer, span)?))
            }

            // Binary operations
            Expression::Negate(expression, span) => {
                let resolved_value =
                    self.enforce_expression(cs, file_scope, function_scope, expected_type, *expression)?;

                enforce_negate(cs, resolved_value, span)
            }
            Expression::Add(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expected_type,
                    *left,
                    *right,
                    span.clone(),
                )?;

                enforce_add(cs, resolved_left, resolved_right, span)
            }
            Expression::Sub(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expected_type,
                    *left,
                    *right,
                    span.clone(),
                )?;

                enforce_sub(cs, resolved_left, resolved_right, span)
            }
            Expression::Mul(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expected_type,
                    *left,
                    *right,
                    span.clone(),
                )?;

                enforce_mul(cs, resolved_left, resolved_right, span)
            }
            Expression::Div(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expected_type,
                    *left,
                    *right,
                    span.clone(),
                )?;

                enforce_div(cs, resolved_left, resolved_right, span)
            }
            Expression::Pow(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expected_type,
                    *left,
                    *right,
                    span.clone(),
                )?;

                enforce_pow(cs, resolved_left, resolved_right, span)
            }

            // Boolean operations
            Expression::Not(expression, span) => Ok(evaluate_not(
                self.enforce_expression(cs, file_scope, function_scope, expected_type, *expression)?,
                span,
            )?),
            Expression::Or(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expected_type,
                    *left,
                    *right,
                    span.clone(),
                )?;

                Ok(enforce_or(cs, resolved_left, resolved_right, span)?)
            }
            Expression::And(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    expected_type,
                    *left,
                    *right,
                    span.clone(),
                )?;

                Ok(enforce_and(cs, resolved_left, resolved_right, span)?)
            }
            Expression::Eq(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    None,
                    *left,
                    *right,
                    span.clone(),
                )?;

                Ok(evaluate_eq(cs, resolved_left, resolved_right, span)?)
            }
            Expression::Ge(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    None,
                    *left,
                    *right,
                    span.clone(),
                )?;

                Ok(evaluate_ge(cs, resolved_left, resolved_right, span)?)
            }
            Expression::Gt(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    None,
                    *left,
                    *right,
                    span.clone(),
                )?;

                Ok(evaluate_gt(cs, resolved_left, resolved_right, span)?)
            }
            Expression::Le(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    None,
                    *left,
                    *right,
                    span.clone(),
                )?;

                Ok(evaluate_le(cs, resolved_left, resolved_right, span)?)
            }
            Expression::Lt(left, right, span) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope.clone(),
                    function_scope.clone(),
                    None,
                    *left,
                    *right,
                    span.clone(),
                )?;

                Ok(evaluate_lt(cs, resolved_left, resolved_right, span)?)
            }

            // Conditionals
            Expression::IfElse(conditional, first, second, span) => self.enforce_conditional_expression(
                cs,
                file_scope,
                function_scope,
                expected_type,
                *conditional,
                *first,
                *second,
                span,
            ),

            // Arrays
            Expression::Array(array, span) => {
                self.enforce_array(cs, file_scope, function_scope, expected_type, array, span)
            }
            Expression::ArrayAccess(array, index, span) => {
                self.enforce_array_access(cs, file_scope, function_scope, expected_type, array, *index, span)
            }

            // Tuples
            Expression::Tuple(tuple, span) => {
                self.enforce_tuple(cs, file_scope, function_scope, expected_type, tuple, span)
            }
            Expression::TupleAccess(tuple, index, span) => {
                self.enforce_tuple_access(cs, file_scope, function_scope, expected_type, tuple, index, span)
            }

            // Circuits
            Expression::Circuit(circuit_name, members, span) => {
                self.enforce_circuit(cs, file_scope, function_scope, circuit_name, members, span)
            }
            Expression::CircuitMemberAccess(circuit_variable, circuit_member, span) => self.enforce_circuit_access(
                cs,
                file_scope,
                function_scope,
                expected_type,
                circuit_variable,
                circuit_member,
                span,
            ),
            Expression::CircuitStaticFunctionAccess(circuit_identifier, circuit_member, span) => self
                .enforce_circuit_static_access(
                    cs,
                    file_scope,
                    function_scope,
                    expected_type,
                    circuit_identifier,
                    circuit_member,
                    span,
                ),

            // Functions
            Expression::FunctionCall(function, arguments, span) => self.enforce_function_call_expression(
                cs,
                file_scope,
                function_scope,
                expected_type,
                function,
                arguments,
                span,
            ),
        }
    }
}
