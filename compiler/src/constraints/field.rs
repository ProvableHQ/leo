//! Methods to enforce constraints on field elements in a resolved Leo program.

use crate::{constraints::ConstrainedValue, errors::FieldError, types::InputValue, FieldType, GroupType};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::alloc::AllocGadget},
};

pub(crate) fn field_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    private: bool,
    input_value: Option<InputValue>,
) -> Result<ConstrainedValue<F, G>, FieldError> {
    // Check that the parameter value is the correct type
    let field_option = match input_value {
        Some(input) => {
            if let InputValue::Field(field_string) = input {
                Some(field_string)
            } else {
                return Err(FieldError::Invalid(input.to_string()));
            }
        }
        None => None,
    };

    // Check visibility of parameter
    let field_value = if private {
        FieldType::alloc(cs.ns(|| name), || field_option.ok_or(SynthesisError::AssignmentMissing))?
    } else {
        FieldType::alloc_input(cs.ns(|| name), || field_option.ok_or(SynthesisError::AssignmentMissing))?
    };

    Ok(ConstrainedValue::Field(field_value))
}
