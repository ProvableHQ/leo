use crate::{input_to_integer_string, input_to_nested_u8_vec, input_to_u8_vec, StateLeafValuesError};

use leo_typed::{InputValue, StateLeaf as TypedStateLeaf};
use std::convert::TryFrom;

static PATH_PARAMETER_STRING: &str = "path";
static MEMO_PARAMETER_STRING: &str = "memo";
static NETWORK_ID_PARAMETER_STRING: &str = "network_id";
static LEAF_RANDOMNESS_PARAMETER_STRING: &str = "leaf_randomness";

pub struct StateLeafValues {
    pub path: Vec<Vec<u8>>,
    pub memo: Vec<u8>,
    pub network_id: u8,
    pub leaf_randomness: Vec<u8>,
}

impl TryFrom<&TypedStateLeaf> for StateLeafValues {
    type Error = StateLeafValuesError;

    fn try_from(state_leaf: &TypedStateLeaf) -> Result<Self, Self::Error> {
        // Lookup path
        let path_value = get_parameter_value(PATH_PARAMETER_STRING.to_owned(), state_leaf)?;
        let path = input_to_nested_u8_vec(path_value)?;

        // Lookup memo
        let memo_value = get_parameter_value(MEMO_PARAMETER_STRING.to_owned(), state_leaf)?;
        let memo = input_to_u8_vec(memo_value)?;

        // Lookup network id
        let network_id_value = get_parameter_value(NETWORK_ID_PARAMETER_STRING.to_owned(), state_leaf)?;
        let network_id = input_to_integer_string(network_id_value)?.parse::<u8>()?;

        // Lookup leaf randomness
        let leaf_randomness_value = get_parameter_value(LEAF_RANDOMNESS_PARAMETER_STRING.to_owned(), state_leaf)?;
        let leaf_randomness = input_to_u8_vec(leaf_randomness_value)?;

        Ok(Self {
            path,
            memo,
            network_id,
            leaf_randomness,
        })
    }
}

fn get_parameter_value(name: String, state: &TypedStateLeaf) -> Result<InputValue, StateLeafValuesError> {
    let parameters = state.values();
    let matched_parameter = parameters
        .iter()
        .find(|(parameter, _value)| parameter.variable.name == name);

    match matched_parameter {
        Some((_parameter, value_option)) => match value_option {
            Some(value) => Ok(value.clone()),
            None => Err(StateLeafValuesError::MissingParameter(name)),
        },
        None => Err(StateLeafValuesError::MissingParameter(name)),
    }
}
