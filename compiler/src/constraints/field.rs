//! Methods to enforce constraints on field elements in a resolved Leo program.

use crate::{constraints::ConstrainedValue, errors::FieldError, FieldType, GroupType};
use leo_types::InputValue;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::alloc::AllocGadget},
};

pub(crate) fn field_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
) -> Result<ConstrainedValue<F, G>, FieldError> {
    // Check that the parameter value is the correct type
    let field_option = match input_value {
        Some(input) => {
            if let InputValue::Field(ast) = input {
                Some(ast.number.value)
            } else {
                return Err(FieldError::Invalid(input.to_string()));
            }
        }
        None => None,
    };

    let field_value = FieldType::alloc(cs.ns(|| name), || field_option.ok_or(SynthesisError::AssignmentMissing))?;

    Ok(ConstrainedValue::Field(field_value))
}
