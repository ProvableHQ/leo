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
    private: bool,
    input_value: Option<InputValue>,
) -> Result<ConstrainedValue<F, G>, GroupError> {
    // Check that the parameter value is the correct type
    let group_option = match input_value {
        Some(input) => {
            if let InputValue::Group(group_string) = input {
                Some(group_string)
            } else {
                return Err(GroupError::Invalid(input.to_string()));
            }
        }
        None => None,
    };

    // Check visibility of parameter
    let group_value = if private {
        G::alloc(cs.ns(|| name), || group_option.ok_or(SynthesisError::AssignmentMissing))?
    } else {
        G::alloc_input(cs.ns(|| name), || group_option.ok_or(SynthesisError::AssignmentMissing))?
    };

    Ok(ConstrainedValue::Group(group_value))
}
