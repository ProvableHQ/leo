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
use leo_ast::{expression::*, Expression, Type};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: &str,
        function_scope: &str,
        expected_type: Option<Type>,
        expression: Expression,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        match expression {
            // Variables
            Expression::Identifier(unresolved_variable) => {
                self.evaluate_identifier(file_scope, function_scope, expected_type, unresolved_variable)
            }

            // Values
            Expression::Value(ValueExpression::Address(address, span)) => {
                Ok(ConstrainedValue::Address(Address::constant(address, &span)?))
            }
            Expression::Value(ValueExpression::Boolean(boolean, span)) => {
                Ok(ConstrainedValue::Boolean(new_bool_constant(boolean, &span)?))
            }
            Expression::Value(ValueExpression::Field(field, span)) => {
                Ok(ConstrainedValue::Field(FieldType::constant(field, &span)?))
            }
            Expression::Value(ValueExpression::Group(group_element)) => {
                Ok(ConstrainedValue::Group(G::constant(*group_element)?))
            }
            Expression::Value(ValueExpression::Implicit(value, span)) => {
                Ok(enforce_number_implicit(expected_type, value, &span)?)
            }
            Expression::Value(ValueExpression::Integer(type_, integer, span)) => Ok(ConstrainedValue::Integer(
                Integer::new(expected_type, &type_, integer, &span)?,
            )),

            // Binary operations
            Expression::Binary(BinaryExpression { left, right, op, span }) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(
                    cs,
                    file_scope,
                    function_scope,
                    if op.class() == BinaryOperationClass::Numeric {
                        expected_type
                    } else {
                        None
                    },
                    *left,
                    *right,
                    &span,
                )?;

                match op {
                    BinaryOperation::Add => enforce_add(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Sub => enforce_sub(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Mul => enforce_mul(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Div => enforce_div(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Pow => enforce_pow(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Or => {
                        enforce_or(cs, resolved_left, resolved_right, &span).map_err(ExpressionError::BooleanError)
                    }
                    BinaryOperation::And => {
                        enforce_and(cs, resolved_left, resolved_right, &span).map_err(ExpressionError::BooleanError)
                    }
                    BinaryOperation::Eq => evaluate_eq(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Ne => evaluate_not(evaluate_eq(cs, resolved_left, resolved_right, &span)?, span)
                        .map_err(ExpressionError::BooleanError),
                    BinaryOperation::Ge => evaluate_ge(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Gt => evaluate_gt(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Le => evaluate_le(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Lt => evaluate_lt(cs, resolved_left, resolved_right, &span),
                }
            }

            // Unary operations
            Expression::Unary(UnaryExpression { inner, op, span }) => match op {
                UnaryOperation::Negate => {
                    let resolved_inner =
                        self.enforce_expression(cs, file_scope, function_scope, expected_type, *inner)?;
                    enforce_negate(cs, resolved_inner, &span)
                }
                UnaryOperation::Not => Ok(evaluate_not(
                    self.enforce_operand(cs, file_scope, function_scope, expected_type, *inner, &span)?,
                    span,
                )?),
            },

            Expression::Ternary(TernaryExpression {
                condition,
                if_true,
                if_false,
                span,
            }) => self.enforce_conditional_expression(
                cs,
                file_scope,
                function_scope,
                expected_type,
                *condition,
                *if_true,
                *if_false,
                &span,
            ),

            // Arrays
            Expression::ArrayInline(ArrayInlineExpression { elements, span }) => {
                self.enforce_array(cs, file_scope, function_scope, expected_type, elements, span)
            }
            Expression::ArrayInit(ArrayInitExpression {
                element,
                dimensions,
                span,
            }) => self.enforce_array_initializer(
                cs,
                file_scope,
                function_scope,
                expected_type,
                *element,
                dimensions,
                span,
            ),
            Expression::ArrayAccess(ArrayAccessExpression { array, index, span }) => {
                self.enforce_array_access(cs, file_scope, function_scope, expected_type, *array, *index, &span)
            }
            Expression::ArrayRangeAccess(ArrayRangeAccessExpression {
                array,
                left,
                right,
                span,
            }) => self.enforce_array_range_access(
                cs,
                file_scope,
                function_scope,
                expected_type,
                *array,
                left.map(|x| *x),
                right.map(|x| *x),
                &span,
            ),

            // Tuples
            Expression::TupleInit(TupleInitExpression { elements, span }) => {
                self.enforce_tuple(cs, file_scope, function_scope, expected_type, elements, span)
            }
            Expression::TupleAccess(TupleAccessExpression { tuple, index, span }) => {
                self.enforce_tuple_access(cs, file_scope, function_scope, expected_type, *tuple, index, &span)
            }

            // Circuits
            Expression::CircuitInit(CircuitInitExpression { name, members, span }) => {
                self.enforce_circuit(cs, file_scope, function_scope, name, members, span)
            }
            Expression::CircuitMemberAccess(CircuitMemberAccessExpression { circuit, name, span }) => {
                self.enforce_circuit_access(cs, file_scope, function_scope, expected_type, *circuit, name, span)
            }
            Expression::CircuitStaticFunctionAccess(CircuitStaticFunctionAccessExpression { circuit, name, span }) => {
                self.enforce_circuit_static_access(cs, file_scope, function_scope, expected_type, *circuit, name, span)
            }

            // Functions
            Expression::Call(CallExpression {
                function,
                arguments,
                span,
            }) => match *function {
                Expression::Identifier(id) if id.is_core() => self.enforce_core_circuit_call_expression(
                    cs,
                    file_scope,
                    function_scope,
                    expected_type,
                    id.name,
                    arguments,
                    span,
                ),
                function => self.enforce_function_call_expression(
                    cs,
                    file_scope,
                    function_scope,
                    expected_type,
                    function,
                    arguments,
                    span,
                ),
            },
        }
    }
}
