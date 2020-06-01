use crate::errors::GroupError;
use crate::{ConstrainedValue, GroupType, InputValue};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::r1cs::ConstraintSystem;

pub(crate) fn group_from_input<
    NativeF: Field,
    F: Field + PrimeField,
    GType: GroupType<NativeF, F>,
    CS: ConstraintSystem<F>,
>(
    cs: &mut CS,
    name: String,
    private: bool,
    input_value: Option<InputValue<F>>,
) -> Result<ConstrainedValue<NativeF, F, GType>, GroupError> {
    // Check that the parameter value is the correct type
    let group_option = match input_value {
        Some(input) => {
            if let InputValue::Group(group_string) = input {
                Some(group_string)
            } else {
                return Err(GroupError::InvalidGroup(input.to_string()));
            }
        }
        None => None,
    };

    // Check visibility of parameter
    let group_value = if private {
        GType::alloc(cs.ns(|| name), || {
            group_option.ok_or(SynthesisError::AssignmentMissing)
        })?
    } else {
        GType::alloc_input(cs.ns(|| name), || {
            group_option.ok_or(SynthesisError::AssignmentMissing)
        })?
    };

    Ok(ConstrainedValue::Group(group_value))
}
