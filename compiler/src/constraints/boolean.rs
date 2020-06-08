//! Methods to enforce constraints on booleans in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::BooleanError,
    GroupType,
};
use leo_types::InputValue;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, boolean::Boolean, eq::EqGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    pub(crate) fn bool_from_input(
        &mut self,
        cs: &mut CS,
        name: String,
        private: bool,
        input_value: Option<InputValue>,
    ) -> Result<ConstrainedValue<F, G>, BooleanError> {
        // Check that the input value is the correct type
        let bool_value = match input_value {
            Some(input) => {
                if let InputValue::Boolean(bool) = input {
                    Some(bool)
                } else {
                    return Err(BooleanError::InvalidBoolean(input.to_string()));
                }
            }
            None => None,
        };

        // Check visibility of input
        let number = if private {
            Boolean::alloc(cs.ns(|| name), || {
                bool_value.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            Boolean::alloc_input(cs.ns(|| name), || {
                bool_value.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(ConstrainedValue::Boolean(number))
    }

    pub(crate) fn get_boolean_constant(bool: Boolean) -> ConstrainedValue<F, G> {
        ConstrainedValue::Boolean(bool)
    }

    pub(crate) fn evaluate_not(
        value: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, BooleanError> {
        match value {
            ConstrainedValue::Boolean(boolean) => Ok(ConstrainedValue::Boolean(boolean.not())),
            value => Err(BooleanError::CannotEvaluate(format!("!{}", value))),
        }
    }

    pub(crate) fn enforce_or(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, BooleanError> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => Ok(
                ConstrainedValue::Boolean(Boolean::or(cs, &left_bool, &right_bool)?),
            ),
            (left_value, right_value) => Err(BooleanError::CannotEnforce(format!(
                "{} || {}",
                left_value, right_value
            ))),
        }
    }

    pub(crate) fn enforce_and(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, BooleanError> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => Ok(
                ConstrainedValue::Boolean(Boolean::and(cs, &left_bool, &right_bool)?),
            ),
            (left_value, right_value) => Err(BooleanError::CannotEnforce(format!(
                "{} && {}",
                left_value, right_value
            ))),
        }
    }

    pub(crate) fn boolean_eq(left: Boolean, right: Boolean) -> ConstrainedValue<F, G> {
        ConstrainedValue::Boolean(Boolean::Constant(left.eq(&right)))
    }

    pub(crate) fn enforce_boolean_eq(
        &mut self,
        cs: &mut CS,
        left: Boolean,
        right: Boolean,
    ) -> Result<(), BooleanError> {
        Ok(left.enforce_equal(cs.ns(|| format!("enforce bool equal")), &right)?)
    }
}
