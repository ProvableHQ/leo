use crate::{errors::GroupError, ConstrainedValue, GroupType};
use leo_types::InputValue;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub(crate) fn group_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
) -> Result<ConstrainedValue<F, G>, GroupError> {
    // Check that the parameter value is the correct type
    let group_option = match input_value {
        Some(input) => {
            if let InputValue::Group(ast) = input {
                Some(ast.value.to_string())
            } else {
                return Err(GroupError::Invalid(input.to_string()));
            }
        }
        None => None,
    };

    let group_value = G::alloc(cs.ns(|| name), || group_option.ok_or(SynthesisError::AssignmentMissing))?;

    Ok(ConstrainedValue::Group(group_value))
}
