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
        utilities::{alloc::AllocGadget, boolean::Boolean},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn bool_from_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        name: String,
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

        let number = Boolean::alloc(cs.ns(|| name), || bool_value.ok_or(SynthesisError::AssignmentMissing))?;

        Ok(ConstrainedValue::Boolean(number))
    }

    pub(crate) fn evaluate_not(value: ConstrainedValue<F, G>) -> Result<ConstrainedValue<F, G>, BooleanError> {
        match value {
            ConstrainedValue::Boolean(boolean) => Ok(ConstrainedValue::Boolean(boolean.not())),
            value => Err(BooleanError::CannotEvaluate(format!("!{}", value))),
        }
    }

    pub(crate) fn enforce_or<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, BooleanError> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => {
                Ok(ConstrainedValue::Boolean(Boolean::or(cs, &left_bool, &right_bool)?))
            }
            (left_value, right_value) => Err(BooleanError::CannotEnforce(format!(
                "{} || {}",
                left_value, right_value
            ))),
        }
    }

    pub(crate) fn enforce_and<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        left: ConstrainedValue<F, G>,
        right: ConstrainedValue<F, G>,
    ) -> Result<ConstrainedValue<F, G>, BooleanError> {
        match (left, right) {
            (ConstrainedValue::Boolean(left_bool), ConstrainedValue::Boolean(right_bool)) => {
                Ok(ConstrainedValue::Boolean(Boolean::and(cs, &left_bool, &right_bool)?))
            }
            (left_value, right_value) => Err(BooleanError::CannotEnforce(format!(
                "{} && {}",
                left_value, right_value
            ))),
        }
    }
}
