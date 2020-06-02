//! A data type that represents a field value

use crate::errors::FieldElementError;

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
use std::{borrow::Borrow, str::FromStr};

pub enum FieldType<F: Field + PrimeField> {
    Constant(F),
    Allocated(FpGadget<F>),
}

impl<F: Field + PrimeField> FieldType<F> {
    pub fn constant(string: String) -> Result<Self, FieldElementError> {
        let value = F::from_str(&string).map_err(|_| FieldElementError::Invalid(string))?;
        Ok(FieldType::Constant(value))
    }

    pub fn add<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        unimplemented!()
    }

    pub fn sub<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        unimplemented!()
    }

    pub fn mul<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        unimplemented!()
    }

    pub fn div<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        unimplemented!()
    }

    pub fn pow<CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
    ) -> Result<Self, FieldElementError> {
        unimplemented!()
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

impl<F: Field + PrimeField> AllocGadget<F, F> for FieldType<F> {
    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<F>, CS: ConstraintSystem<F>>(
        cs: CS,
        f: Fn,
    ) -> Result<Self, SynthesisError> {
        unimplemented!()
    }

    fn alloc_input<
        Fn: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<F>,
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
