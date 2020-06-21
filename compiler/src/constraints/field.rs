//! Methods to enforce constraints on field elements in a resolved Leo program.

use crate::{constraints::ConstrainedValue, errors::FieldError, FieldType, GroupType};
use leo_types::{InputValue, Span};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::alloc::AllocGadget},
};

pub(crate) fn field_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, FieldError> {
    // Check that the parameter value is the correct type
    let field_option = match input_value {
        Some(input) => {
            if let InputValue::Field(string) = input {
                Some(string)
            } else {
                return Err(FieldError::invalid_field(input.to_string(), span));
            }
        }
        None => None,
    };

    let field_name = format!("{}: field", name);
    let field_name_unique = format!("`{}` {}:{}", field_name, span.line, span.start);
    let field_value = FieldType::alloc(cs.ns(|| field_name_unique), || {
        field_option.ok_or(SynthesisError::AssignmentMissing)
    })
    .map_err(|_| FieldError::missing_field(field_name, span))?;

    Ok(ConstrainedValue::Field(field_value))
}
