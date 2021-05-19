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

//! A data type that represents a field value

use crate::{errors::FieldError, number_string_typing};
use leo_ast::Span;

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::{
    fields::{AllocatedFp, FpGadget},
    traits::{
        fields::FieldGadget,
        utilities::{
            alloc::AllocGadget,
            boolean::Boolean,
            eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
            select::CondSelectGadget,
            uint::UInt8,
            ToBitsBEGadget,
            ToBytesGadget,
        },
    },
};
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};

use snarkvm_gadgets::utilities::eq::NEqGadget;
use std::{borrow::Borrow, cmp::Ordering};

#[derive(Clone, Debug)]
pub struct FieldType<F: PrimeField>(FpGadget<F>);

impl<F: PrimeField> FieldType<F> {
    /// Returns the value of the field.
    pub fn get_value(&self) -> Option<F> {
        self.0.get_value()
    }

    /// Returns a new `FieldType` from the given `String` or returns a `FieldError`.
    pub fn constant<CS: ConstraintSystem<F>>(_cs: CS, string: String, span: &Span) -> Result<Self, FieldError> {
        let number_info = number_string_typing(&string);

        let value = match number_info {
            (number, neg) if neg => {
                -F::from_str(&number).map_err(|_| FieldError::invalid_field(string.clone(), span))?
            }
            (number, _) => F::from_str(&number).map_err(|_| FieldError::invalid_field(string.clone(), span))?,
        };

        let value = FpGadget::alloc_constant(_cs, || Ok(value)).map_err(|_| FieldError::invalid_field(string, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `negate` function.
    pub fn negate<CS: ConstraintSystem<F>>(&self, cs: CS, span: &Span) -> Result<Self, FieldError> {
        let result = self.0.negate(cs).map_err(|e| FieldError::negate_operation(e, span))?;

        Ok(FieldType(result))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `add` function.
    pub fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        let value = self
            .0
            .add(cs, &other.0)
            .map_err(|e| FieldError::binary_operation("+".to_string(), e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `sub` function.
    pub fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        let value = self
            .0
            .sub(cs, &other.0)
            .map_err(|e| FieldError::binary_operation("-".to_string(), e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `mul` function.
    pub fn mul<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        let value = self
            .0
            .mul(cs, &other.0)
            .map_err(|e| FieldError::binary_operation("*".to_string(), e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `inverse` function.
    pub fn inverse<CS: ConstraintSystem<F>>(&self, cs: CS, span: &Span) -> Result<Self, FieldError> {
        let value = self
            .0
            .inverse(cs)
            .map_err(|e| FieldError::binary_operation("inv".to_string(), e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `div` function.
    pub fn div<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        let inverse = other.inverse(cs.ns(|| "division inverse"), span)?;

        self.mul(cs, &inverse, span)
    }

    pub fn alloc_helper<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>>(
        value_gen: Fn,
    ) -> Result<F, SynthesisError> {
        let field_string = match value_gen() {
            Ok(value) => {
                let string_value = value.borrow().clone();
                Ok(string_value)
            }
            _ => Err(SynthesisError::AssignmentMissing),
        }?;

        F::from_str(&field_string).map_err(|_| SynthesisError::AssignmentMissing)
    }
}

impl<F: PrimeField> AllocGadget<String, F> for FieldType<F> {
    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = FpGadget::alloc(cs, || Self::alloc_helper(value_gen))?;

        Ok(FieldType(value))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = FpGadget::alloc_input(cs, || Self::alloc_helper(value_gen))?;

        Ok(FieldType(value))
    }
}

impl<F: PrimeField> PartialEq for FieldType<F> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<F: PrimeField> Eq for FieldType<F> {}

impl<F: PrimeField> PartialOrd for FieldType<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = self.get_value();
        let other_value = other.get_value();

        Option::from(self_value.cmp(&other_value))
    }
}

impl<F: PrimeField> EvaluateEqGadget<F> for FieldType<F> {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        self.0.is_eq(cs, &other.0)
    }
}

impl<F: PrimeField> EqGadget<F> for FieldType<F> {}

impl<F: PrimeField> ConditionalEqGadget<F> for FieldType<F> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        self.0.conditional_enforce_equal(cs, &other.0, condition)
    }

    fn cost() -> usize {
        2 * <FpGadget<F> as ConditionalEqGadget<F>>::cost()
    }
}

impl<F: PrimeField> NEqGadget<F> for FieldType<F> {
    fn enforce_not_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), SynthesisError> {
        self.0.enforce_not_equal(cs, &other.0)
    }

    fn cost() -> usize {
        <FpGadget<F> as NEqGadget<F>>::cost()
    }
}

impl<F: PrimeField> CondSelectGadget<F> for FieldType<F> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        let value = FpGadget::conditionally_select(cs, cond, &first.0, &second.0)?;

        Ok(FieldType(value))
    }

    fn cost() -> usize {
        2 * <FpGadget<F> as CondSelectGadget<F>>::cost()
    }
}

impl<F: PrimeField> ToBitsBEGadget<F> for FieldType<F> {
    fn to_bits_be<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        self.0.to_bits_be(cs)
    }

    fn to_bits_be_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        self.0.to_bits_be(cs)
    }
}

impl<F: PrimeField> ToBytesGadget<F> for FieldType<F> {
    fn to_bytes<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.0.to_bytes(cs)
    }

    fn to_bytes_strict<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        self.0.to_bytes_strict(cs)
    }
}

impl<F: PrimeField> std::fmt::Display for FieldType<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.get_value().ok_or(std::fmt::Error))
    }
}

#[derive(Clone, Debug)]
pub enum OldFieldType<F: PrimeField> {
    Constant(F),
    Allocated(FpGadget<F>),
}

impl<F: PrimeField> OldFieldType<F> {
    pub fn get_value(&self) -> Option<F> {
        match self {
            OldFieldType::Constant(field) => Some(*field),
            OldFieldType::Allocated(gadget) => gadget.get_value(),
        }
    }

    pub fn constant(string: String, span: &Span) -> Result<Self, FieldError> {
        let number_info = number_string_typing(&string);

        let value = match number_info {
            (number, neg) if neg => -F::from_str(&number).map_err(|_| FieldError::invalid_field(string, span))?,
            (number, _) => F::from_str(&number).map_err(|_| FieldError::invalid_field(string, span))?,
        };

        Ok(OldFieldType::Constant(value))
    }

    pub fn negate<CS: ConstraintSystem<F>>(&self, cs: CS, span: &Span) -> Result<Self, FieldError> {
        match self {
            OldFieldType::Constant(field) => Ok(OldFieldType::Constant(field.neg())),
            OldFieldType::Allocated(field) => {
                let result = field.negate(cs).map_err(|e| FieldError::negate_operation(e, span))?;

                Ok(OldFieldType::Allocated(result))
            }
        }
    }

    pub fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        match (self, other) {
            (OldFieldType::Constant(self_value), OldFieldType::Constant(other_value)) => {
                Ok(OldFieldType::Constant(self_value.add(other_value)))
            }

            (OldFieldType::Allocated(self_value), OldFieldType::Allocated(other_value)) => {
                let result = self_value
                    .add(cs, other_value)
                    .map_err(|e| FieldError::binary_operation("+".to_string(), e, span))?;

                Ok(OldFieldType::Allocated(result))
            }

            (OldFieldType::Constant(constant_value), OldFieldType::Allocated(allocated_value))
            | (OldFieldType::Allocated(allocated_value), OldFieldType::Constant(constant_value)) => {
                Ok(OldFieldType::Allocated(
                    allocated_value
                        .add_constant(cs, constant_value)
                        .map_err(|e| FieldError::binary_operation("+".to_string(), e, span))?,
                ))
            }
        }
    }

    pub fn sub<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        match (self, other) {
            (OldFieldType::Constant(self_value), OldFieldType::Constant(other_value)) => {
                Ok(OldFieldType::Constant(self_value.sub(other_value)))
            }

            (OldFieldType::Allocated(self_value), OldFieldType::Allocated(other_value)) => {
                let result = self_value
                    .sub(cs, other_value)
                    .map_err(|e| FieldError::binary_operation("-".to_string(), e, span))?;

                Ok(OldFieldType::Allocated(result))
            }

            (OldFieldType::Constant(constant_value), OldFieldType::Allocated(allocated_value)) => {
                let result = allocated_value
                    .sub_constant(cs.ns(|| "field_sub_constant"), constant_value)
                    .map_err(|e| FieldError::binary_operation("-".to_string(), e, span))?
                    .negate(cs.ns(|| "negate"))
                    .map_err(|e| FieldError::binary_operation("-".to_string(), e, span))?;

                Ok(OldFieldType::Allocated(result))
            }

            (OldFieldType::Allocated(allocated_value), OldFieldType::Constant(constant_value)) => {
                Ok(OldFieldType::Allocated(
                    allocated_value
                        .sub_constant(cs, constant_value)
                        .map_err(|e| FieldError::binary_operation("-".to_string(), e, span))?,
                ))
            }
        }
    }

    pub fn mul<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        match (self, other) {
            (OldFieldType::Constant(self_value), OldFieldType::Constant(other_value)) => {
                Ok(OldFieldType::Constant(self_value.mul(other_value)))
            }

            (OldFieldType::Allocated(self_value), OldFieldType::Allocated(other_value)) => {
                let result = self_value
                    .mul(cs, other_value)
                    .map_err(|e| FieldError::binary_operation("*".to_string(), e, span))?;

                Ok(OldFieldType::Allocated(result))
            }

            (OldFieldType::Constant(constant_value), OldFieldType::Allocated(allocated_value))
            | (OldFieldType::Allocated(allocated_value), OldFieldType::Constant(constant_value)) => {
                Ok(OldFieldType::Allocated(
                    allocated_value
                        .mul_by_constant(cs, constant_value)
                        .map_err(|e| FieldError::binary_operation("*".to_string(), e, span))?,
                ))
            }
        }
    }

    pub fn div<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self, span: &Span) -> Result<Self, FieldError> {
        let inverse = match other {
            OldFieldType::Constant(constant) => {
                let constant_inverse = constant
                    .inverse()
                    .ok_or_else(|| FieldError::no_inverse(constant.to_string(), span))?;

                OldFieldType::Constant(constant_inverse)
            }
            OldFieldType::Allocated(allocated) => {
                let allocated_inverse = allocated
                    .inverse(&mut cs)
                    .map_err(|e| FieldError::binary_operation("+".to_string(), e, span))?;

                OldFieldType::Allocated(allocated_inverse)
            }
        };

        self.mul(cs, &inverse, span)
    }

    pub fn alloc_helper<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>>(
        value_gen: Fn,
    ) -> Result<F, SynthesisError> {
        let field_string = match value_gen() {
            Ok(value) => {
                let string_value = value.borrow().clone();
                Ok(string_value)
            }
            _ => Err(SynthesisError::AssignmentMissing),
        }?;

        F::from_str(&field_string).map_err(|_| SynthesisError::AssignmentMissing)
    }

    pub fn allocated<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<FpGadget<F>, SynthesisError> {
        match self {
            OldFieldType::Constant(constant) => {
                FpGadget::alloc(&mut cs.ns(|| format!("{:?}", constant)), || Ok(constant))
            }
            OldFieldType::Allocated(allocated) => FpGadget::alloc(&mut cs.ns(|| format!("{:?}", allocated)), || {
                allocated.get_value().ok_or(SynthesisError::AssignmentMissing)
            }),
        }
    }
}

impl<F: PrimeField> AllocGadget<String, F> for OldFieldType<F> {
    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = FpGadget::alloc(cs, || Self::alloc_helper(value_gen))?;

        Ok(OldFieldType::Allocated(value))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<F>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = FpGadget::alloc_input(cs, || Self::alloc_helper(value_gen))?;

        Ok(OldFieldType::Allocated(value))
    }
}

impl<F: PrimeField> PartialEq for OldFieldType<F> {
    fn eq(&self, other: &Self) -> bool {
        let self_value = self.get_value();
        let other_value = other.get_value();

        self_value.is_some() && other_value.is_some() && self_value.eq(&other_value)
    }
}

impl<F: PrimeField> Eq for OldFieldType<F> {}

impl<F: PrimeField> PartialOrd for OldFieldType<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = self.get_value();
        let other_value = other.get_value();

        Option::from(self_value.cmp(&other_value))
    }
}

impl<F: PrimeField> EvaluateEqGadget<F> for OldFieldType<F> {
    fn evaluate_equal<CS: ConstraintSystem<F>>(&self, mut _cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        match (self, other) {
            (OldFieldType::Constant(first), OldFieldType::Constant(second)) => Ok(Boolean::constant(first.eq(second))),
            _ => unimplemented!("field equality not implemented yet"), // (FieldType::Allocated(first), FieldType::Allocated(second)) => first.is_eq(cs, second),
                                                                       // (FieldType::Constant(constant_value), FieldType::Allocated(allocated_value))
                                                                       // | (FieldType::Allocated(allocated_value), FieldType::Constant(constant_value)) => {
                                                                       //     let allocated_constant_value =
                                                                       //         FpGadget::alloc(&mut cs.ns(|| format!("alloc constant for eq")), || Ok(constant_value))?;
                                                                       //     allocated_value.is_eq(cs, &allocated_constant_value)
                                                                       // }
        }
    }
}

impl<F: PrimeField> EqGadget<F> for OldFieldType<F> {}

impl<F: PrimeField> ConditionalEqGadget<F> for OldFieldType<F> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            // c - c
            (OldFieldType::Constant(self_value), OldFieldType::Constant(other_value)) => {
                if self_value == other_value {
                    return Ok(());
                }
                Err(SynthesisError::AssignmentMissing)
            }
            // a - a
            (OldFieldType::Allocated(self_value), OldFieldType::Allocated(other_value)) => {
                self_value.conditional_enforce_equal(cs, other_value, condition)
            }
            // c - a = a - c
            (OldFieldType::Constant(constant_value), OldFieldType::Allocated(allocated_value))
            | (OldFieldType::Allocated(allocated_value), OldFieldType::Constant(constant_value)) => {
                let constant_gadget = FpGadget::from(AllocatedFp::from(&mut cs, constant_value));

                constant_gadget.conditional_enforce_equal(cs, allocated_value, condition)
            }
        }
    }

    fn cost() -> usize {
        2 * <FpGadget<F> as ConditionalEqGadget<F>>::cost()
    }
}

impl<F: PrimeField> CondSelectGadget<F> for OldFieldType<F> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        if let Boolean::Constant(cond) = *cond {
            if cond { Ok(first.clone()) } else { Ok(second.clone()) }
        } else {
            let first_gadget = first.allocated(&mut cs)?;
            let second_gadget = second.allocated(&mut cs)?;
            let result = FpGadget::conditionally_select(cs, cond, &first_gadget, &second_gadget)?;

            Ok(OldFieldType::Allocated(result))
        }
    }

    fn cost() -> usize {
        2 * <FpGadget<F> as CondSelectGadget<F>>::cost()
    }
}

impl<F: PrimeField> ToBitsBEGadget<F> for OldFieldType<F> {
    fn to_bits_be<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits_be(cs)
    }

    fn to_bits_be_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits_be_strict(cs)
    }
}

impl<F: PrimeField> ToBytesGadget<F> for OldFieldType<F> {
    fn to_bytes<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes(cs)
    }

    fn to_bytes_strict<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes_strict(cs)
    }
}

impl<F: PrimeField> std::fmt::Display for OldFieldType<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.get_value().ok_or(std::fmt::Error))
    }
}
