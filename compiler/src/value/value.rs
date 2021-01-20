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

//! The in memory stored value for a defined name in a compiled Leo program.

use crate::{
    boolean::input::{allocate_bool},
    errors::{FieldError, ValueError},
    Address,
    FieldType,
    GroupType,
    Integer,
};
use leo_asg::{CircuitBody, Identifier, Span, Type};
use leo_core::Value;

use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, eq::ConditionalEqGadget, select::CondSelectGadget},
    },
};
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct ConstrainedCircuitMember<F: Field + PrimeField, G: GroupType<F>>(pub Identifier, pub ConstrainedValue<F, G>);

#[derive(Clone, PartialEq, Eq)]
pub enum ConstrainedValue<F: Field + PrimeField, G: GroupType<F>> {
    // Data types
    Address(Address),
    Boolean(Boolean),
    Field(FieldType<F>),
    Group(G),
    Integer(Integer),

    // Arrays
    Array(Vec<ConstrainedValue<F, G>>),

    // Tuples
    Tuple(Vec<ConstrainedValue<F, G>>),

    // Circuits
    CircuitExpression(Arc<CircuitBody>, Uuid, Vec<ConstrainedCircuitMember<F, G>>),

    // Modifiers
    Mutable(Box<ConstrainedValue<F, G>>),
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedValue<F, G> {

    pub(crate) fn to_type(&self, span: &Span) -> Result<Type, ValueError> {
        Ok(match self {
            // Data types
            ConstrainedValue::Address(_address) => Type::Address,
            ConstrainedValue::Boolean(_bool) => Type::Boolean,
            ConstrainedValue::Field(_field) => Type::Field,
            ConstrainedValue::Group(_group) => Type::Group,
            ConstrainedValue::Integer(integer) => Type::Integer(integer.get_type()),

            // Data type wrappers
            ConstrainedValue::Array(array) => {
                let array_type = array[0].to_type(span)?;

                Type::Array(Box::new(array_type), array.len())
            }
            ConstrainedValue::Tuple(tuple) => {
                let mut types = Vec::with_capacity(tuple.len());

                for value in tuple {
                    let type_ = value.to_type(span)?;
                    types.push(type_)
                }

                Type::Tuple(types)
            }
            ConstrainedValue::CircuitExpression(id, _, _members) => Type::Circuit(id.circuit.clone()),
            ConstrainedValue::Mutable(value) => return value.to_type(span),
            value => return Err(ValueError::implicit(value.to_string(), span.to_owned())),
        })
    }

    /// Returns the `ConstrainedValue` in intermediate `Value` format (for core circuits)
    pub(crate) fn to_value(&self) -> Value {
        match self.clone() {
            ConstrainedValue::Boolean(boolean) => Value::Boolean(boolean),
            ConstrainedValue::Integer(integer) => match integer {
                Integer::U8(u8) => Value::U8(u8),
                Integer::U16(u16) => Value::U16(u16),
                Integer::U32(u32) => Value::U32(u32),
                Integer::U64(u64) => Value::U64(u64),
                Integer::U128(u128) => Value::U128(u128),

                Integer::I8(i8) => Value::I8(i8),
                Integer::I16(i16) => Value::I16(i16),
                Integer::I32(i32) => Value::I32(i32),
                Integer::I64(i64) => Value::I64(i64),
                Integer::I128(i128) => Value::I128(i128),
            },
            ConstrainedValue::Array(array) => {
                let array_value = array.into_iter().map(|element| element.to_value()).collect();

                Value::Array(array_value)
            }
            ConstrainedValue::Tuple(tuple) => {
                let tuple_value = tuple.into_iter().map(|element| element.to_value()).collect();

                Value::Tuple(tuple_value)
            }
            _ => unimplemented!(),
        }
    }

    ///
    /// Modifies the `self` [ConstrainedValue] so there are no `mut` keywords wrapping the `self` value.
    ///
    pub(crate) fn get_inner_mut(&mut self) {
        if let ConstrainedValue::Mutable(inner) = self {
            // Recursively remove `mut` keywords.
            inner.get_inner_mut();

            // Modify the value.
            *self = *inner.clone()
        }
    }

    pub(crate) fn allocate_value<CS: ConstraintSystem<F>>(
        &mut self,
        mut cs: CS,
        span: &Span,
    ) -> Result<(), ValueError> {
        match self {
            // Data types
            ConstrainedValue::Address(_address) => {
                // allow `let address()` even though addresses are constant
            }
            ConstrainedValue::Boolean(boolean) => {
                let option = boolean.get_value();
                let name = option
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "[allocated]".to_string());

                *boolean = allocate_bool(&mut cs, &name, option, span)?;
            }
            ConstrainedValue::Field(field) => {
                let gadget = field
                    .allocated(cs.ns(|| format!("allocate field {}:{}", span.line, span.start)))
                    .map_err(|error| ValueError::FieldError(FieldError::synthesis_error(error, span.to_owned())))?;

                *field = FieldType::Allocated(gadget)
            }
            ConstrainedValue::Group(group) => {
                *group = group.to_allocated(cs, span)?;
            }
            ConstrainedValue::Integer(integer) => {
                let integer_type = integer.get_type();
                let option = integer.get_value();
                let name = option.clone().unwrap_or_else(|| "[allocated]".to_string());

                *integer = Integer::allocate_type(&mut cs, integer_type, &name, option, span)?;
            }

            // Data type wrappers
            ConstrainedValue::Array(array) => {
                array.iter_mut().enumerate().try_for_each(|(i, value)| {
                    let unique_name = format!("allocate array member {} {}:{}", i, span.line, span.start);

                    value.allocate_value(cs.ns(|| unique_name), span)
                })?;
            }
            ConstrainedValue::Tuple(tuple) => {
                tuple.iter_mut().enumerate().try_for_each(|(i, value)| {
                    let unique_name = format!("allocate tuple member {} {}:{}", i, span.line, span.start);

                    value.allocate_value(cs.ns(|| unique_name), span)
                })?;
            }
            ConstrainedValue::CircuitExpression(_id, _, members) => {
                members.iter_mut().enumerate().try_for_each(|(i, member)| {
                    let unique_name = format!("allocate circuit member {} {}:{}", i, span.line, span.start);

                    member.1.allocate_value(cs.ns(|| unique_name), span)
                })?;
            }
            ConstrainedValue::Mutable(value) => {
                value.allocate_value(cs, span)?;
            }
        }

        Ok(())
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> fmt::Display for ConstrainedValue<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Data types
            ConstrainedValue::Address(ref value) => write!(f, "{}", value),
            ConstrainedValue::Boolean(ref value) => write!(
                f,
                "{}",
                value
                    .get_value()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "[allocated]".to_string())
            ),
            ConstrainedValue::Field(ref value) => write!(f, "{:?}", value),
            ConstrainedValue::Group(ref value) => write!(f, "{:?}", value),
            ConstrainedValue::Integer(ref value) => write!(f, "{}", value),

            // Data type wrappers
            ConstrainedValue::Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ConstrainedValue::Tuple(ref tuple) => {
                let values = tuple.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

                write!(f, "({})", values)
            }
            ConstrainedValue::CircuitExpression(ref circuit, _, ref members) => {
                write!(f, "{} {{", circuit.circuit.name.borrow())?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.0, member.1)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            ConstrainedValue::Mutable(ref value) => write!(f, "{}", value),
        }
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> fmt::Debug for ConstrainedValue<F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConditionalEqGadget<F> for ConstrainedValue<F, G> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            (ConstrainedValue::Address(address_1), ConstrainedValue::Address(address_2)) => {
                address_1.conditional_enforce_equal(cs, address_2, condition)
            }
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                bool_1.conditional_enforce_equal(cs, bool_2, condition)
            }
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
                field_1.conditional_enforce_equal(cs, field_2, condition)
            }
            (ConstrainedValue::Group(group_1), ConstrainedValue::Group(group_2)) => {
                group_1.conditional_enforce_equal(cs, group_2, condition)
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                num_1.conditional_enforce_equal(cs, num_2, condition)
            }
            (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
                for (i, (left, right)) in arr_1.iter().zip(arr_2.iter()).enumerate() {
                    left.conditional_enforce_equal(cs.ns(|| format!("array[{}]", i)), right, condition)?;
                }
                Ok(())
            }
            (ConstrainedValue::Tuple(tuple_1), ConstrainedValue::Tuple(tuple_2)) => {
                for (i, (left, right)) in tuple_1.iter().zip(tuple_2.iter()).enumerate() {
                    left.conditional_enforce_equal(cs.ns(|| format!("tuple index {}", i)), right, condition)?;
                }
                Ok(())
            }
            (_, _) => Err(SynthesisError::Unsatisfiable),
        }
    }

    fn cost() -> usize {
        unimplemented!()
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> CondSelectGadget<F> for ConstrainedValue<F, G> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        Ok(match (first, second) {
            (ConstrainedValue::Address(address_1), ConstrainedValue::Address(address_2)) => {
                ConstrainedValue::Address(Address::conditionally_select(cs, cond, address_1, address_2)?)
            }
            (ConstrainedValue::Boolean(bool_1), ConstrainedValue::Boolean(bool_2)) => {
                ConstrainedValue::Boolean(Boolean::conditionally_select(cs, cond, bool_1, bool_2)?)
            }
            (ConstrainedValue::Field(field_1), ConstrainedValue::Field(field_2)) => {
                ConstrainedValue::Field(FieldType::conditionally_select(cs, cond, field_1, field_2)?)
            }
            (ConstrainedValue::Group(group_1), ConstrainedValue::Group(group_2)) => {
                ConstrainedValue::Group(G::conditionally_select(cs, cond, group_1, group_2)?)
            }
            (ConstrainedValue::Integer(num_1), ConstrainedValue::Integer(num_2)) => {
                ConstrainedValue::Integer(Integer::conditionally_select(cs, cond, num_1, num_2)?)
            }
            (ConstrainedValue::Array(arr_1), ConstrainedValue::Array(arr_2)) => {
                let mut array = Vec::with_capacity(arr_1.len());

                for (i, (first, second)) in arr_1.iter().zip(arr_2.iter()).enumerate() {
                    array.push(Self::conditionally_select(
                        cs.ns(|| format!("array[{}]", i)),
                        cond,
                        first,
                        second,
                    )?);
                }

                ConstrainedValue::Array(array)
            }
            (ConstrainedValue::Tuple(tuple_1), ConstrainedValue::Tuple(tuple_2)) => {
                let mut array = Vec::with_capacity(tuple_1.len());

                for (i, (first, second)) in tuple_1.iter().zip(tuple_2.iter()).enumerate() {
                    array.push(Self::conditionally_select(
                        cs.ns(|| format!("tuple index {}", i)),
                        cond,
                        first,
                        second,
                    )?);
                }

                ConstrainedValue::Tuple(array)
            }
            (
                ConstrainedValue::CircuitExpression(identifier, _, members_1),
                ConstrainedValue::CircuitExpression(_identifier, _, members_2),
            ) => {
                let mut members = Vec::with_capacity(members_1.len());

                for (i, (first, second)) in members_1.iter().zip(members_2.iter()).enumerate() {
                    members.push(ConstrainedCircuitMember::conditionally_select(
                        cs.ns(|| format!("circuit member[{}]", i)),
                        cond,
                        first,
                        second,
                    )?);
                }

                ConstrainedValue::CircuitExpression(identifier.clone(), Uuid::new_v4(), members)
            }
            (ConstrainedValue::Mutable(first), _) => Self::conditionally_select(cs, cond, first, second)?,
            (_, ConstrainedValue::Mutable(second)) => Self::conditionally_select(cs, cond, first, second)?,
            (_, _) => return Err(SynthesisError::Unsatisfiable),
        })
    }

    fn cost() -> usize {
        unimplemented!() //lower bound 1, upper bound 128 or length of static array
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> CondSelectGadget<F> for ConstrainedCircuitMember<F, G> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        // identifiers will be the same
        let value = ConstrainedValue::conditionally_select(cs, cond, &first.1, &second.1)?;

        Ok(ConstrainedCircuitMember(first.0.clone(), value))
    }

    fn cost() -> usize {
        unimplemented!()
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> From<Value> for ConstrainedValue<F, G> {
    fn from(v: Value) -> Self {
        match v {
            Value::Boolean(boolean) => ConstrainedValue::Boolean(boolean),
            Value::U8(u8) => ConstrainedValue::Integer(Integer::U8(u8)),
            Value::U16(u16) => ConstrainedValue::Integer(Integer::U16(u16)),
            Value::U32(u32) => ConstrainedValue::Integer(Integer::U32(u32)),
            Value::U64(u64) => ConstrainedValue::Integer(Integer::U64(u64)),
            Value::U128(u128) => ConstrainedValue::Integer(Integer::U128(u128)),

            Value::I8(i8) => ConstrainedValue::Integer(Integer::I8(i8)),
            Value::I16(i16) => ConstrainedValue::Integer(Integer::I16(i16)),
            Value::I32(i32) => ConstrainedValue::Integer(Integer::I32(i32)),
            Value::I64(i64) => ConstrainedValue::Integer(Integer::I64(i64)),
            Value::I128(i128) => ConstrainedValue::Integer(Integer::I128(i128)),

            Value::Array(array) => ConstrainedValue::Array(array.into_iter().map(ConstrainedValue::from).collect()),
            Value::Tuple(tuple) => ConstrainedValue::Tuple(tuple.into_iter().map(ConstrainedValue::from).collect()),
        }
    }
}
