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

use crate::number_string_typing;
use leo_errors::{CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_gadgets::{
    bits::{ToBitsBEGadget, ToBytesGadget},
    boolean::Boolean,
    fields::FpGadget,
    integers::uint::UInt8,
    traits::{
        alloc::AllocGadget,
        bits::comparator::{ComparatorGadget, EvaluateLtGadget},
        eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget, NEqGadget},
        fields::FieldGadget,
        select::CondSelectGadget,
    },
};
use snarkvm_r1cs::{ConstraintSystem, SynthesisError};
use std::{borrow::Borrow, cmp::Ordering};

#[derive(Clone, Debug)]
pub struct FieldType<F: PrimeField>(FpGadget<F>);

impl<F: PrimeField> FieldType<F> {
    /// Returns the value of the field.
    pub fn get_value(&self) -> Option<F> {
        self.0.get_value()
    }

    /// Returns a new `FieldType` from the given `String` or returns a `FieldError`.
    pub fn constant<CS: ConstraintSystem<F>>(cs: CS, string: String, span: &Span) -> Result<Self> {
        let number_info = number_string_typing(&string);

        let value = match number_info {
            (number, neg) if neg => {
                -F::from_str(&number).map_err(|_| CompilerError::field_value_invalid_field(string.clone(), span))?
            }
            (number, _) => {
                F::from_str(&number).map_err(|_| CompilerError::field_value_invalid_field(string.clone(), span))?
            }
        };

        let value = FpGadget::alloc_constant(cs, || Ok(value))
            .map_err(|_| CompilerError::field_value_invalid_field(string, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `negate` function.
    pub fn negate<CS: ConstraintSystem<F>>(&self, cs: CS, span: &Span) -> Result<Self> {
        let result = self
            .0
            .negate(cs)
            .map_err(|e| CompilerError::field_value_negate_operation(e, span))?;

        Ok(FieldType(result))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `add` function.
    pub fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self> {
        let value = self
            .0
            .add(cs, &other.0)
            .map_err(|e| CompilerError::field_value_binary_operation("+", e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `sub` function.
    pub fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self> {
        let value = self
            .0
            .sub(cs, &other.0)
            .map_err(|e| CompilerError::field_value_binary_operation("-", e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `mul` function.
    pub fn mul<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self> {
        let value = self
            .0
            .mul(cs, &other.0)
            .map_err(|e| CompilerError::field_value_binary_operation("*", e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `inverse` function.
    pub fn inverse<CS: ConstraintSystem<F>>(&self, cs: CS, span: &Span) -> Result<Self> {
        let value = self
            .0
            .inverse(cs)
            .map_err(|e| CompilerError::field_value_binary_operation("inv", e, span))?;

        Ok(FieldType(value))
    }

    /// Returns a new `FieldType` by calling the `FpGadget` `div` function.
    pub fn div<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self, span: &Span) -> Result<Self> {
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

impl<F: PrimeField> EvaluateLtGadget<F> for FieldType<F> {
    fn less_than<CS: ConstraintSystem<F>>(&self, _cs: CS, _other: &Self) -> Result<Boolean, SynthesisError> {
        unimplemented!()
    }
}

impl<F: PrimeField> ComparatorGadget<F> for FieldType<F> {}

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
        match self.get_value().ok_or(std::fmt::Error) {
            Ok(value) => write!(f, "{}", value),
            value => write!(f, "{:?}", value),
        }
    }
}
