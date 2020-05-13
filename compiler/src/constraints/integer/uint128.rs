//! Methods to enforce constraints on uint128s in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::{InputModel, Integer},
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, Group, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, eq::EqGadget, uint128::UInt128},
    },
};

impl<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    pub(crate) fn u128_from_integer(
        &mut self,
        cs: &mut CS,
        parameter_model: InputModel<F, G>,
        integer_option: Option<usize>,
    ) -> Result<ConstrainedValue<F, G>, IntegerError> {
        // Type cast to u128 in rust.
        // If this fails should we return our own error?
        let u128_option = integer_option.map(|integer| integer as u128);

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let integer_value = if parameter_model.private {
            UInt128::alloc(cs.ns(|| name), || {
                u128_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            UInt128::alloc_input(cs.ns(|| name), || {
                u128_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(ConstrainedValue::Integer(Integer::U128(integer_value)))
    }

    pub(crate) fn enforce_u128_eq(
        cs: &mut CS,
        left: UInt128,
        right: UInt128,
    ) -> Result<(), IntegerError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce u128 equal")), &right)?)
    }

    pub(crate) fn enforce_u128_add(
        cs: &mut CS,
        left: UInt128,
        right: UInt128,
    ) -> Result<UInt128, IntegerError> {
        Ok(UInt128::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )?)
    }

    pub(crate) fn enforce_u128_sub(
        cs: &mut CS,
        left: UInt128,
        right: UInt128,
    ) -> Result<UInt128, IntegerError> {
        Ok(left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }

    pub(crate) fn enforce_u128_mul(
        cs: &mut CS,
        left: UInt128,
        right: UInt128,
    ) -> Result<UInt128, IntegerError> {
        Ok(left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u128_div(
        cs: &mut CS,
        left: UInt128,
        right: UInt128,
    ) -> Result<UInt128, IntegerError> {
        Ok(left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u128_pow(
        cs: &mut CS,
        left: UInt128,
        right: UInt128,
    ) -> Result<UInt128, IntegerError> {
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
