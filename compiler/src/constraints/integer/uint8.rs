//! Methods to enforce constraints on uint8s in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::{InputModel, Integer},
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Group, Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, eq::EqGadget, uint8::UInt8},
    },
};

impl<G: Group, F: Field + PrimeField, CS: ConstraintSystem<F>> ConstrainedProgram<G, F, CS> {
    pub(crate) fn u8_from_input(
        &mut self,
        cs: &mut CS,
        parameter_model: InputModel<G, F>,
        integer_option: Option<usize>,
    ) -> Result<ConstrainedValue<G, F>, IntegerError> {
        // Type cast to u8 in rust.
        // If this fails should we return our own error?
        let u8_option = integer_option.map(|integer| integer as u8);

        // Check visibility of parameter
        let name = parameter_model.variable.name.clone();
        let integer_value = if parameter_model.private {
            UInt8::alloc(cs.ns(|| name), || {
                u8_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            UInt8::alloc_input(cs.ns(|| name), || {
                u8_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(ConstrainedValue::Integer(Integer::U8(integer_value)))
    }

    pub(crate) fn enforce_u8_eq(
        cs: &mut CS,
        left: UInt8,
        right: UInt8,
    ) -> Result<(), IntegerError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce u8 equal")), &right)?)
    }

    pub(crate) fn enforce_u8_add(
        cs: &mut CS,
        left: UInt8,
        right: UInt8,
    ) -> Result<UInt8, IntegerError> {
        Ok(UInt8::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )?)
    }

    pub(crate) fn enforce_u8_sub(
        cs: &mut CS,
        left: UInt8,
        right: UInt8,
    ) -> Result<UInt8, IntegerError> {
        Ok(left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }

    pub(crate) fn enforce_u8_mul(
        cs: &mut CS,
        left: UInt8,
        right: UInt8,
    ) -> Result<UInt8, IntegerError> {
        Ok(left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u8_div(
        cs: &mut CS,
        left: UInt8,
        right: UInt8,
    ) -> Result<UInt8, IntegerError> {
        Ok(left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u8_pow(
        cs: &mut CS,
        left: UInt8,
        right: UInt8,
    ) -> Result<UInt8, IntegerError> {
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
