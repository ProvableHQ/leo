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
    resolve_core_circuit,
    value::{Address, ConstrainedValue, Integer},
    FieldType,
    GroupType,
};
use leo_asg::{expression::*, ConstValue, Expression, Node};
use std::sync::Arc;

use snarkvm_models::{
    curves::PrimeField,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: &Arc<Expression>,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        let span = expression.span().cloned().unwrap_or_default();
        match &**expression {
            // Variables
            Expression::VariableRef(variable_ref) => self.evaluate_ref(variable_ref),

            // Values
            Expression::Constant(Constant { value, .. }) => {
                Ok(match value {
                    ConstValue::Address(value) => ConstrainedValue::Address(Address::constant(value.clone(), &span)?),
                    ConstValue::Boolean(value) => ConstrainedValue::Boolean(Boolean::Constant(*value)),
                    ConstValue::Field(value) => ConstrainedValue::Field(FieldType::constant(value.to_string(), &span)?),
                    ConstValue::Group(value) => ConstrainedValue::Group(G::constant(value, &span)?),
                    ConstValue::Int(value) => ConstrainedValue::Integer(Integer::new(value)),
                    ConstValue::Tuple(_) | ConstValue::Array(_) => unimplemented!(), // shouldnt be in the asg here
                })
            }

            // Binary operations
            Expression::Binary(BinaryExpression {
                left, right, operation, ..
            }) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(cs, left, right)?;

                match operation {
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
                    BinaryOperation::Ne => evaluate_not(evaluate_eq(cs, resolved_left, resolved_right, &span)?, &span)
                        .map_err(ExpressionError::BooleanError),
                    BinaryOperation::Ge => evaluate_ge(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Gt => evaluate_gt(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Le => evaluate_le(cs, resolved_left, resolved_right, &span),
                    BinaryOperation::Lt => evaluate_lt(cs, resolved_left, resolved_right, &span),
                }
            }

            // Unary operations
            Expression::Unary(UnaryExpression { inner, operation, .. }) => match operation {
                UnaryOperation::Negate => {
                    let resolved_inner = self.enforce_expression(cs, inner)?;
                    enforce_negate(cs, resolved_inner, &span)
                }
                UnaryOperation::Not => Ok(evaluate_not(self.enforce_expression(cs, inner)?, &span)?),
            },

            Expression::Ternary(TernaryExpression {
                condition,
                if_true,
                if_false,
                ..
            }) => self.enforce_conditional_expression(cs, condition, if_true, if_false, &span),

            // Arrays
            Expression::ArrayInline(ArrayInlineExpression { elements, .. }) => self.enforce_array(cs, elements, span),
            Expression::ArrayInit(ArrayInitExpression { element, len, .. }) => {
                self.enforce_array_initializer(cs, element, *len)
            }
            Expression::ArrayAccess(ArrayAccessExpression { array, index, .. }) => {
                self.enforce_array_access(cs, array, index, &span)
            }
            Expression::ArrayRangeAccess(ArrayRangeAccessExpression { array, left, right, .. }) => {
                self.enforce_array_range_access(cs, array, left.as_ref(), right.as_ref(), &span)
            }

            // Tuples
            Expression::TupleInit(TupleInitExpression { elements, .. }) => self.enforce_tuple(cs, elements),
            Expression::TupleAccess(TupleAccessExpression { tuple_ref, index, .. }) => {
                self.enforce_tuple_access(cs, tuple_ref, *index, &span)
            }

            // Circuits
            Expression::CircuitInit(expr) => self.enforce_circuit(cs, expr, &span),
            Expression::CircuitAccess(expr) => self.enforce_circuit_access(cs, expr),

            // Functions
            Expression::Call(CallExpression {
                function,
                target,
                arguments,
                ..
            }) => {
                if let Some(circuit) = function
                    .circuit
                    .borrow()
                    .as_ref()
                    .map(|x| x.upgrade().expect("stale circuit for member function"))
                {
                    let core_mapping = circuit.core_mapping.borrow();
                    if let Some(core_mapping) = core_mapping.as_deref() {
                        let core_circuit = resolve_core_circuit::<F, G>(core_mapping);
                        return self.enforce_core_circuit_call_expression(
                            cs,
                            &core_circuit,
                            &function,
                            target.as_ref(),
                            arguments,
                            &span,
                        );
                    }
                }
                self.enforce_function_call_expression(cs, &function, target.as_ref(), arguments, &span)
            }
        }
    }
}
