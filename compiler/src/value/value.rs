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

//! The in memory stored value for a defined name in a compiled Leo program.

use crate::errors::ValueError;
use crate::Address;
use crate::FieldType;
use crate::GroupType;
use crate::Integer;
use leo_asg::Circuit;
use leo_asg::Identifier;
use leo_asg::Span;
use leo_asg::Type;

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::traits::utilities::boolean::Boolean;
use snarkvm_gadgets::traits::utilities::eq::ConditionalEqGadget;
use snarkvm_gadgets::traits::utilities::select::CondSelectGadget;
use snarkvm_r1cs::ConstraintSystem;
use snarkvm_r1cs::SynthesisError;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ConstrainedCircuitMember<'a, F: PrimeField, G: GroupType<F>>(pub Identifier, pub ConstrainedValue<'a, F, G>);

#[derive(Clone, PartialEq, Eq)]
pub enum ConstrainedValue<'a, F: PrimeField, G: GroupType<F>> {
    // Data types
    Address(Address),
    Boolean(Boolean),
    Field(FieldType<F>),
    Group(G),
    Integer(Integer),

    // Arrays
    Array(Vec<ConstrainedValue<'a, F, G>>),

    // Tuples
    Tuple(Vec<ConstrainedValue<'a, F, G>>),

    // Circuits
    CircuitExpression(&'a Circuit<'a>, Vec<ConstrainedCircuitMember<'a, F, G>>),
}

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedValue<'a, F, G> {
    pub(crate) fn to_type(&self, span: &Span) -> Result<Type<'a>, ValueError> {
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
            ConstrainedValue::CircuitExpression(id, _members) => Type::Circuit(*id),
        })
    }
}

impl<'a, F: PrimeField, G: GroupType<F>> fmt::Display for ConstrainedValue<'a, F, G> {
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
            ConstrainedValue::CircuitExpression(ref circuit, ref members) => {
                write!(f, "{} {{", circuit.name.borrow())?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.0, member.1)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

impl<'a, F: PrimeField, G: GroupType<F>> fmt::Debug for ConstrainedValue<'a, F, G> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a, F: PrimeField, G: GroupType<F>> ConditionalEqGadget<F> for ConstrainedValue<'a, F, G> {
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

impl<'a, F: PrimeField, G: GroupType<F>> CondSelectGadget<F> for ConstrainedValue<'a, F, G> {
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
                ConstrainedValue::CircuitExpression(identifier, members_1),
                ConstrainedValue::CircuitExpression(_identifier, members_2),
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

                ConstrainedValue::CircuitExpression(*identifier, members)
            }
            (_, _) => return Err(SynthesisError::Unsatisfiable),
        })
    }

    fn cost() -> usize {
        unimplemented!() //lower bound 1, upper bound 128 or length of static array
    }
}

impl<'a, F: PrimeField, G: GroupType<F>> CondSelectGadget<F> for ConstrainedCircuitMember<'a, F, G> {
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
