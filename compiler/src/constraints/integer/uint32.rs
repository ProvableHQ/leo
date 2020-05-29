//! Methods to enforce constraints on uint32s in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::Integer,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, eq::EqGadget, uint32::UInt32},
    },
};

impl<F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<F, CS> {
    pub(crate) fn u32_from_input(
        &mut self,
        cs: &mut CS,
        name: String,
        private: bool,
        integer_option: Option<usize>,
    ) -> Result<ConstrainedValue<F>, IntegerError> {
        // Type cast to integers.u32 in rust.
        // If this fails should we return our own error?
        let u32_option = integer_option.map(|integer| integer as u32);

        // Check visibility of parameter
        let integer_value = if private {
            UInt32::alloc(cs.ns(|| name), || {
                u32_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            UInt32::alloc_input(cs.ns(|| name), || {
                u32_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(ConstrainedValue::Integer(Integer::U32(integer_value)))
    }

    pub(crate) fn enforce_u32_eq(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<(), IntegerError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce integers.u32 equal")), &right)?)
    }

    pub(crate) fn enforce_u32_add(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(UInt32::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )?)
    }

    pub(crate) fn enforce_u32_sub(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }

    pub(crate) fn enforce_u32_mul(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u32_div(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u32_pow(
        cs: &mut CS,
        left: UInt32,
        right: UInt32,
    ) -> Result<UInt32, IntegerError> {
        Ok(left.pow(
            cs.ns(|| {
                format!(
                    "enforce {} ** {}",
                    left.value.unwrap(),
                    right.value.unwrap()
                )
            }),
            &right,
        )?)
    }
}
