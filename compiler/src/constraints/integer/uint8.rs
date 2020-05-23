//! Methods to enforce constraints on uint8s in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::Integer,
};
use leo_gadgets::integers::uint8::UInt8;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::alloc::AllocGadget},
};

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
        CS: ConstraintSystem<F>,
    > ConstrainedProgram<P, F, FG, CS>
{
    pub(crate) fn u8_from_input(
        &mut self,
        cs: &mut CS,
        name: String,
        private: bool,
        integer_option: Option<usize>,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        // Type cast to u8 in rust.
        // If this fails should we return our own error?
        let u8_option = integer_option.map(|integer| integer as u8);

        // Check visibility of parameter
        let integer_value = if private {
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
