//! Methods to enforce constraints on uint64s in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::Integer,
    GroupType,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            eq::EqGadget,
            uint::{UInt, UInt64},
        },
    },
};

impl<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    pub(crate) fn u64_from_input(
        &mut self,
        cs: &mut CS,
        name: String,
        private: bool,
        integer_option: Option<usize>,
    ) -> Result<ConstrainedValue<F, G>, IntegerError> {
        // Type cast to u64 in rust.
        // If this fails should we return our own error?
        let u64_option = integer_option.map(|integer| integer as u64);

        // Check visibility of parameter
        let integer_value = if private {
            UInt64::alloc(cs.ns(|| name), || {
                u64_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            UInt64::alloc_input(cs.ns(|| name), || {
                u64_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(ConstrainedValue::Integer(Integer::U64(integer_value)))
    }

    pub(crate) fn enforce_u64_eq(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<(), IntegerError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce u64 equal")), &right)?)
    }

    pub(crate) fn enforce_u64_add(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(UInt64::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )?)
    }

    pub(crate) fn enforce_u64_sub(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }

    pub(crate) fn enforce_u64_mul(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u64_div(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
        Ok(left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u64_pow(
        cs: &mut CS,
        left: UInt64,
        right: UInt64,
    ) -> Result<UInt64, IntegerError> {
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
