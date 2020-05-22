//! Methods to enforce constraints on uint16s in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::IntegerError,
    types::Integer,
};
use leo_gadgets::integers::uint16::UInt16;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, eq::EqGadget},
    },
};

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField + std::borrow::Borrow<P::BaseField>,
        FG: FieldGadget<P::BaseField, F>,
        CS: ConstraintSystem<F>,
    > ConstrainedProgram<P, F, FG, CS>
{
    pub(crate) fn u16_from_input(
        &mut self,
        cs: &mut CS,
        name: String,
        private: bool,
        integer_option: Option<usize>,
    ) -> Result<ConstrainedValue<P, F, FG>, IntegerError> {
        // Type cast to u16 in rust.
        // If this fails should we return our own error?
        let u16_option = integer_option.map(|integer| integer as u16);

        // Check visibility of parameter
        let integer_value = if private {
            UInt16::alloc(cs.ns(|| name), || {
                u16_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            UInt16::alloc_input(cs.ns(|| name), || {
                u16_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(ConstrainedValue::Integer(Integer::U16(integer_value)))
    }

    pub(crate) fn enforce_u16_eq(
        cs: &mut CS,
        left: UInt16,
        right: UInt16,
    ) -> Result<(), IntegerError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce u16 equal")), &right)?)
    }

    pub(crate) fn enforce_u16_add(
        cs: &mut CS,
        left: UInt16,
        right: UInt16,
    ) -> Result<UInt16, IntegerError> {
        Ok(UInt16::addmany(
            cs.ns(|| format!("enforce {} + {}", left.value.unwrap(), right.value.unwrap())),
            &[left, right],
        )?)
    }

    pub(crate) fn enforce_u16_sub(
        cs: &mut CS,
        left: UInt16,
        right: UInt16,
    ) -> Result<UInt16, IntegerError> {
        Ok(left.sub(
            cs.ns(|| format!("enforce {} - {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }

    pub(crate) fn enforce_u16_mul(
        cs: &mut CS,
        left: UInt16,
        right: UInt16,
    ) -> Result<UInt16, IntegerError> {
        Ok(left.mul(
            cs.ns(|| format!("enforce {} * {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u16_div(
        cs: &mut CS,
        left: UInt16,
        right: UInt16,
    ) -> Result<UInt16, IntegerError> {
        Ok(left.div(
            cs.ns(|| format!("enforce {} / {}", left.value.unwrap(), right.value.unwrap())),
            &right,
        )?)
    }
    pub(crate) fn enforce_u16_pow(
        cs: &mut CS,
        left: UInt16,
        right: UInt16,
    ) -> Result<UInt16, IntegerError> {
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
