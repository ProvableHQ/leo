use crate::{input_to_integer_string, input_to_u8_vec, StateValuesError};

use leo_typed::{InputValue, State as TypedState};
use std::convert::TryFrom;

static LEAF_INDEX_PARAMETER_STRING: &str = "leaf_index";
static ROOT_PARAMETER_STRING: &str = "root";

pub struct StateValues {
    pub leaf_index: u32,
    pub root: Vec<u8>,
}

impl TryFrom<&TypedState> for StateValues {
    type Error = StateValuesError;

    fn try_from(state: &TypedState) -> Result<Self, Self::Error> {
        // Lookup leaf index
        let leaf_index_value = get_parameter_value(LEAF_INDEX_PARAMETER_STRING.to_owned(), state).unwrap();
        let leaf_index = input_to_integer_string(leaf_index_value)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        // Lookup root
        let root_value = get_parameter_value(ROOT_PARAMETER_STRING.to_owned(), state).unwrap();
        let root = input_to_u8_vec(root_value).unwrap();

        Ok(Self { leaf_index, root })
    }
}

fn get_parameter_value(name: String, state: &TypedState) -> Result<InputValue, StateValuesError> {
    let parameters = state.values();
    let matched_parameter = parameters
        .iter()
        .find(|(parameter, _value)| parameter.variable.name == name);

    match matched_parameter {
        Some((_parameter, value_option)) => match value_option {
            Some(value) => Ok(value.clone()),
            None => Err(StateValuesError::MissingParameter(name)),
        },
        None => Err(StateValuesError::MissingParameter(name)),
    }
}
