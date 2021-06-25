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
    errors::{AddressError, ExpressionError},
    program::Program,
};
use bech32::FromBase32;
use leo_asg::{CharValue, ConstInt, ConstValue, Expression, GroupValue, Node, Span, expression::*};
use num_bigint::Sign;
use snarkvm_ir::{Group, GroupCoordinate, Integer, Value};

pub(crate) fn asg_group_coordinate_to_ir(coordinate: &leo_asg::GroupCoordinate) -> GroupCoordinate {
    match coordinate {
        leo_asg::GroupCoordinate::Number(parsed) => GroupCoordinate::Field(snarkvm_ir::Field {
            values: parsed.magnitude().iter_u64_digits().collect(),
            negate: parsed.sign() == Sign::Minus,
        }),
        leo_asg::GroupCoordinate::SignHigh => GroupCoordinate::SignHigh,
        leo_asg::GroupCoordinate::SignLow => GroupCoordinate::SignLow,
        leo_asg::GroupCoordinate::Inferred => GroupCoordinate::Inferred,
    }
}

pub fn decode_address(value: &str, span: &Span) -> Result<Vec<u8>, AddressError> {
    if !value.starts_with("aleo") || value.len() != 63 {
        return Err(AddressError::invalid_address(value.as_ref(), span));
    }
    let (_, data, _) =
        bech32::decode(value.as_ref()).map_err(|_| AddressError::invalid_address(value.as_ref(), span))?;
    let bytes = Vec::from_base32(&data).map_err(|_| AddressError::invalid_address(value.as_ref(), span))?;
    Ok(bytes)
}

impl<'a> Program<'a> {
    pub(crate) fn enforce_const_value(&mut self, value: &ConstValue, span: &Span) -> Result<Value, ExpressionError> {
        Ok(match value {
            ConstValue::Address(value) => Value::Address(decode_address(value.as_ref(), span)?),
            ConstValue::Boolean(value) => Value::Boolean(*value),
            ConstValue::Char(value) => Value::Char(match value {
                CharValue::Scalar(x) => *x as u32,
                CharValue::NonScalar(x) => *x,
            }),
            ConstValue::Field(parsed) => Value::Field(snarkvm_ir::Field {
                values: parsed.magnitude().iter_u64_digits().collect(),
                negate: parsed.sign() == Sign::Minus,
            }),
            ConstValue::Group(value) => match value {
                GroupValue::Single(parsed) => Value::Group(Group::Single(snarkvm_ir::Field {
                    values: parsed.magnitude().iter_u64_digits().collect(),
                    negate: parsed.sign() == Sign::Minus,
                })),
                GroupValue::Tuple(left, right) => Value::Group(Group::Tuple(
                    asg_group_coordinate_to_ir(left),
                    asg_group_coordinate_to_ir(right),
                )),
            },
            ConstValue::Int(value) => Value::Integer(match *value {
                ConstInt::I8(x) => Integer::I8(x),
                ConstInt::I16(x) => Integer::I16(x),
                ConstInt::I32(x) => Integer::I32(x),
                ConstInt::I64(x) => Integer::I64(x),
                ConstInt::I128(x) => Integer::I128(x),
                ConstInt::U8(x) => Integer::U8(x),
                ConstInt::U16(x) => Integer::U16(x),
                ConstInt::U32(x) => Integer::U32(x),
                ConstInt::U64(x) => Integer::U64(x),
                ConstInt::U128(x) => Integer::U128(x),
            }),
            ConstValue::Tuple(values) => Value::Tuple(
                values
                    .iter()
                    .map(|x| self.enforce_const_value(x, span))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            ConstValue::Array(values) => Value::Array(
                values
                    .iter()
                    .map(|x| self.enforce_const_value(x, span))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        })
    }

    pub(crate) fn enforce_expression(&mut self, expression: &'a Expression<'a>) -> Result<Value, ExpressionError> {
        let span = &expression.span().cloned().unwrap_or_default();
        match expression {
            // Cast
            Expression::Cast(_) => unimplemented!("casts not implemented"),

            // Variables
            Expression::VariableRef(variable_ref) => self.evaluate_ref(variable_ref),

            // Values
            Expression::Constant(Constant { value, .. }) => self.enforce_const_value(value, span),

            // Binary operations
            Expression::Binary(BinaryExpression {
                left, right, operation, ..
            }) => {
                let (resolved_left, resolved_right) = self.enforce_binary_expression(left.get(), right.get())?;

                match operation {
                    BinaryOperation::Add => self.evaluate_add(resolved_left, resolved_right),
                    BinaryOperation::Sub => self.evaluate_sub(resolved_left, resolved_right),
                    BinaryOperation::Mul => self.evaluate_mul(resolved_left, resolved_right),
                    BinaryOperation::Div => self.evaluate_div(resolved_left, resolved_right),
                    BinaryOperation::Pow => self.evaluate_pow(resolved_left, resolved_right),
                    BinaryOperation::Or => self.evaluate_or(resolved_left, resolved_right),
                    BinaryOperation::And => self.evaluate_and(resolved_left, resolved_right),
                    BinaryOperation::Eq => self.evaluate_eq(resolved_left, resolved_right),
                    BinaryOperation::Ne => self.evaluate_ne(resolved_left, resolved_right),
                    BinaryOperation::Ge => self.evaluate_ge(resolved_left, resolved_right),
                    BinaryOperation::Gt => self.evaluate_gt(resolved_left, resolved_right),
                    BinaryOperation::Le => self.evaluate_le(resolved_left, resolved_right),
                    BinaryOperation::Lt => self.evaluate_lt(resolved_left, resolved_right),
                    _ => unimplemented!("unimplemented binary operator"),
                }
            }

            // Unary operations
            Expression::Unary(UnaryExpression { inner, operation, .. }) => match operation {
                UnaryOperation::Negate => {
                    let resolved_inner = self.enforce_expression(inner.get())?;
                    self.evaluate_negate(resolved_inner)
                }
                UnaryOperation::Not => {
                    let inner = self.enforce_expression(inner.get())?;
                    Ok(self.evaluate_not(inner)?)
                }
                _ => unimplemented!("unimplemented unary operator"),
            },

            Expression::Ternary(TernaryExpression {
                condition,
                if_true,
                if_false,
                ..
            }) => self.enforce_conditional_expression(condition.get(), if_true.get(), if_false.get()),

            // Arrays
            Expression::ArrayInline(ArrayInlineExpression { elements, .. }) => self.enforce_array(&elements[..]),
            Expression::ArrayInit(ArrayInitExpression { element, len, .. }) => {
                self.enforce_array_initializer(element.get(), *len)
            }
            Expression::ArrayAccess(ArrayAccessExpression { array, index, .. }) => {
                self.enforce_array_access(array.get(), index.get())
            }
            Expression::ArrayRangeAccess(ArrayRangeAccessExpression {
                array,
                left,
                right,
                length,
                ..
            }) => self.enforce_array_range_access(array.get(), left.get(), right.get(), *length),

            // Tuples
            Expression::TupleInit(TupleInitExpression { elements, .. }) => self.enforce_tuple(&elements[..]),
            Expression::TupleAccess(TupleAccessExpression { tuple_ref, index, .. }) => {
                self.enforce_tuple_access(tuple_ref.get(), *index)
            }

            // Circuits
            Expression::CircuitInit(expr) => self.enforce_circuit(expr, span),
            Expression::CircuitAccess(expr) => self.enforce_circuit_access(expr),

            // Functions
            Expression::Call(CallExpression {
                function,
                target,
                arguments,
                ..
            }) => self
                .enforce_function_call(function.get(), target.get(), &arguments[..])
                .map_err(|e| Box::new(e).into()),
        }
    }
}
