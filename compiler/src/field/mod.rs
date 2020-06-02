//! A data type that represents a field value

use crate::errors::FieldError;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        curves::FpGadget,
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::Boolean,
            eq::{ConditionalEqGadget, EqGadget},
            select::CondSelectGadget,
            uint8::UInt8,
            ToBitsGadget, ToBytesGadget,
        },
    },
};
use std::borrow::Borrow;
use snarkos_models::gadgets::curves::FieldGadget;

#[derive(Clone, Debug)]
pub enum FieldType<F: Field + PrimeField> {
    Constant(F),
    Allocated(FpGadget<F>),
}

impl<F: Field + PrimeField> FieldType<F> {
    pub fn constant(string: String) -> Result<Self, FieldError> {
        let value = F::from_str(&string).map_err(|_| FieldError::Invalid(string))?;

        Ok(FieldType::Constant(value))
    }

    pub fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, FieldError> {
        match (self, other) {
            (FieldType::Constant(self_value), FieldType::Constant(other_value)) => {
                Ok(FieldType::Constant(self_value.add(other_value)))
            }

            (FieldType::Allocated(self_value), FieldType::Allocated(other_value)) => {
                let result = self_value.add(cs, other_value)?;

                Ok(FieldType::Allocated(result))
            }

            (
                FieldType::Constant(constant_value),
                FieldType::Allocated(allocated_value),
            )
            | (
                FieldType::Allocated(allocated_value),
                FieldType::Constant(constant_value),
            ) => Ok(FieldType::Allocated(
                allocated_value.add_constant(cs, constant_value)?,
            )),
        }
    }

    pub fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, FieldError> {
        match (self, other) {
            (FieldType::Constant(self_value), FieldType::Constant(other_value)) => {
                Ok(FieldType::Constant(self_value.sub(other_value)))
            }

            (FieldType::Allocated(self_value), FieldType::Allocated(other_value)) => {
                let result = self_value.sub(cs, other_value)?;

                Ok(FieldType::Allocated(result))
            }

            (
                FieldType::Constant(constant_value),
                FieldType::Allocated(allocated_value),
            )
            | (
                FieldType::Allocated(allocated_value),
                FieldType::Constant(constant_value),
            ) => Ok(FieldType::Allocated(
                allocated_value.sub_constant(cs, constant_value)?,
            )),
        }
    }

    pub fn mul<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, FieldError> {
        match (self, other) {
            (FieldType::Constant(self_value), FieldType::Constant(other_value)) => {
                Ok(FieldType::Constant(self_value.mul(other_value)))
            }

            (FieldType::Allocated(self_value), FieldType::Allocated(other_value)) => {
                let result = self_value.mul(cs, other_value)?;

                Ok(FieldType::Allocated(result))
            }

            (
                FieldType::Constant(constant_value),
                FieldType::Allocated(allocated_value),
            )
            | (
                FieldType::Allocated(allocated_value),
                FieldType::Constant(constant_value),
            ) => Ok(FieldType::Allocated(
                allocated_value.mul_by_constant(cs, constant_value)?,
            )),
        }
    }

    pub fn div<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, FieldError> {
        let inverse = match other {
            FieldType::Constant(constant) => {
                let constant_inverse = constant.inverse().ok_or(FieldError::NoInverse(constant.to_string()))?;
                FieldType::Constant(constant_inverse)
            }
            FieldType::Allocated(allocated) => {
                let allocated_inverse = allocated.inverse(&mut cs)?;
                FieldType::Allocated(allocated_inverse)
            }
        };

        self.mul(cs, &inverse)
    }
}

impl<F: Field + PrimeField> Eq for FieldType<F> {}

impl<F: Field + PrimeField> PartialEq for FieldType<F> {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl<F: Field + PrimeField> EqGadget<F> for FieldType<F> {}

impl<F: Field + PrimeField> ConditionalEqGadget<F> for FieldType<F> {
    fn conditional_enforce_equal<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        unimplemented!()
    }

    fn cost() -> usize {
        unimplemented!()
    }
}

impl<F: Field + PrimeField> AllocGadget<String, F> for FieldType<F> {
    fn alloc<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<String>,
        CS: ConstraintSystem<F>,
    >(
        cs: CS,
        f: Fn,
    ) -> Result<Self, SynthesisError> {
        unimplemented!()
    }

    fn alloc_input<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<String>,
        CS: ConstraintSystem<F>,
    >(
        cs: CS,
        f: Fn,
    ) -> Result<Self, SynthesisError> {
        unimplemented!()
    }
}

impl<F: Field + PrimeField> CondSelectGadget<F> for FieldType<F> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        unimplemented!()
    }

    fn cost() -> usize {
        unimplemented!()
    }
}

impl<F: Field + PrimeField> ToBitsGadget<F> for FieldType<F> {
    fn to_bits<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        unimplemented!()
    }

    fn to_bits_strict<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
    ) -> Result<Vec<Boolean>, SynthesisError> {
        unimplemented!()
    }
}

impl<F: Field + PrimeField> ToBytesGadget<F> for FieldType<F> {
    fn to_bytes<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        unimplemented!()
    }

    fn to_bytes_strict<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
    ) -> Result<Vec<UInt8>, SynthesisError> {
        unimplemented!()
    }
}
