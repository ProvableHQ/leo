use crate::{errors::GroupError, ConstrainedValue, GroupType};
use leo_types::{InputValue, Span};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub(crate) fn group_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, GroupError> {
    // Check that the parameter value is the correct type
    let group_option = match input_value {
        Some(input) => {
            if let InputValue::Group(string) = input {
                Some(string)
            } else {
                return Err(GroupError::invalid_group(input.to_string(), span));
            }
        }
        None => None,
    };

    let group_name = format!("{}: group", name);
    let group_name_unique = format!("`{}` {}:{}", group_name, span.line, span.start);
    let group_value = G::alloc(cs.ns(|| group_name_unique), || {
        group_option.ok_or(SynthesisError::AssignmentMissing)
    })
    .map_err(|_| GroupError::missing_group(group_name, span))?;

    Ok(ConstrainedValue::Group(group_value))
}
