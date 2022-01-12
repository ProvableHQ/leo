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
    logical::*,
    program::ConstrainedProgram,
    relational::*,
    // resolve_core_circuit,
    value::{Address, Char, CharType, ConstrainedCircuitMember, ConstrainedValue, Integer},
    FieldType,
    GroupType,
};
use leo_asg::{expression::*, ConstValue, Expression, Node};
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::boolean::Boolean;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub(crate) fn enforce_const_value<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        value: &'a ConstValue<'a>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        Ok(match value {
            ConstValue::Address(value) => ConstrainedValue::Address(Address::constant(value.to_string(), span)?),
            ConstValue::Boolean(value) => ConstrainedValue::Boolean(Boolean::Constant(*value)),
            ConstValue::Char(value) => {
                use leo_asg::CharValue::*;
                match value {
                    Scalar(scalar) => ConstrainedValue::Char(Char::constant(
                        cs,
                        CharType::Scalar(*scalar),
                        format!("{}", *scalar as u32),
                        span,
                    )?),
                    NonScalar(non_scalar) => ConstrainedValue::Char(Char::constant(
                        cs,
                        CharType::NonScalar(*non_scalar),
                        format!("{}", *non_scalar),
                        span,
                    )?),
                }
            }
            ConstValue::Field(value) => ConstrainedValue::Field(FieldType::constant(cs, value.to_string(), span)?),
            ConstValue::Group(value) => ConstrainedValue::Group(G::constant(value, span)?),
            ConstValue::Int(value) => ConstrainedValue::Integer(Integer::new(value)),
            ConstValue::Tuple(values) => ConstrainedValue::Tuple(
                values
                    .iter()
                    .map(|x| self.enforce_const_value(cs, x, span))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            ConstValue::Array(values) => ConstrainedValue::Array(
                values
                    .iter()
                    .map(|x| self.enforce_const_value(cs, x, span))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            ConstValue::Circuit(circuit, members) => {
                let mut constrained_members = Vec::new();
                for (_, (identifier, member)) in members.iter() {
                    constrained_members.push(ConstrainedCircuitMember(
                        identifier.clone(),
                        self.enforce_const_value(cs, member, span)?,
                    ));
                }

                ConstrainedValue::CircuitExpression(circuit, constrained_members)
            }
        })
    }

    pub(crate) fn enforce_expression<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expression: &'a Expression<'a>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        let span = &expression.span().cloned().unwrap_or_default();
        match expression {
            // Cast
            Expression::Cast(_) => unimplemented!("casts not implemented"),

            // LengthOf
            Expression::LengthOf(lengthof) => self.enforce_lengthof(cs, lengthof, span),

            // Variables
            Expression::VariableRef(variable_ref) => self.evaluate_ref(variable_ref),

            // Values
            Expression::Constant(Constant { value, .. }) => self.enforce_const_value(cs, value, span),

            // Binary operations
            Expression::Binary(BinaryExpression {
                left, right, operation, ..
            }) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(cs, left.get(), right.get())?;

                match operation {
                    BinaryOperation::Add => enforce_add(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Sub => enforce_sub(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Mul => enforce_mul(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Div => enforce_div(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Pow => enforce_pow(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Or => enforce_or(cs, resolved_left, resolved_right, span),
                    BinaryOperation::And => enforce_and(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Eq => evaluate_eq(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Ne => evaluate_not(evaluate_eq(cs, resolved_left, resolved_right, span)?, span),
                    BinaryOperation::Ge => evaluate_ge(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Gt => evaluate_gt(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Le => evaluate_le(cs, resolved_left, resolved_right, span),
                    BinaryOperation::Lt => evaluate_lt(cs, resolved_left, resolved_right, span),
                    _ => unimplemented!("unimplemented binary operator"),
                }
            }

            // Unary operations
            Expression::Unary(UnaryExpression { inner, operation, .. }) => match operation {
                UnaryOperation::Negate => {
                    let resolved_inner = self.enforce_expression(cs, inner.get())?;
                    enforce_negate(cs, resolved_inner, span)
                }
                UnaryOperation::Not => Ok(evaluate_not(self.enforce_expression(cs, inner.get())?, span)?),
                _ => unimplemented!("unimplemented unary operator"),
            },

            Expression::Ternary(TernaryExpression {
                condition,
                if_true,
                if_false,
                ..
            }) => self.enforce_conditional_expression(cs, condition.get(), if_true.get(), if_false.get(), span),

            // Arrays
            Expression::ArrayInline(ArrayInlineExpression { elements, .. }) => {
                self.enforce_array(cs, &elements[..], span)
            }
            Expression::ArrayInit(ArrayInitExpression { element, len, .. }) => {
                self.enforce_array_initializer(cs, element.get(), *len)
            }
            Expression::ArrayAccess(ArrayAccessExpression { array, index, .. }) => {
                self.enforce_array_access(cs, array.get(), index.get(), span)
            }
            Expression::ArrayRangeAccess(ArrayRangeAccessExpression {
                array,
                left,
                right,
                length,
                ..
            }) => self.enforce_array_range_access(cs, array.get(), left.get(), right.get(), *length, span),

            // Tuples
            Expression::TupleInit(TupleInitExpression { elements, .. }) => self.enforce_tuple(cs, &elements[..]),
            Expression::TupleAccess(TupleAccessExpression { tuple_ref, index, .. }) => {
                self.enforce_tuple_access(cs, tuple_ref.get(), *index, span)
            }

            // Circuits
            Expression::CircuitInit(expr) => self.enforce_circuit(cs, expr, span),
            Expression::CircuitAccess(expr) => self.enforce_circuit_access(cs, expr),

            // Functions
            Expression::Call(CallExpression {
                // function,
                // target,
                // arguments,
                ..
            }) => {
                unimplemented!("core circuits are not supported yet")
                // if let Some(circuit) = function.get().circuit.get() {
                //     let core_mapping = circuit.core_mapping.borrow();
                //     if let Some(core_mapping) = core_mapping.as_deref() {
                //         let core_circuit = resolve_core_circuit::<F, G>(core_mapping);
                //         return self.enforce_core_circuit_call_expression(
                //             cs,
                //             &core_circuit,
                //             function.get(),
                //             target.get(),
                //             &arguments[..],
                //             span,
                //         );
                //     }
                // }
                // self.enforce_function_call_expression(cs, function.get(), target.get(), &arguments[..], span)
            }
        }
    }
}
