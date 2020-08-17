use crate::{find_input, input_to_integer_string, input_to_u8_vec, StateValuesError};
use leo_typed::State as TypedState;

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
        let parameters = state.values();

        // Lookup leaf index
        let leaf_index_value = find_input(LEAF_INDEX_PARAMETER_STRING.to_owned(), &parameters)?;
        let leaf_index = input_to_integer_string(leaf_index_value)?.parse::<u32>()?;

        // Lookup root
        let root_value = find_input(ROOT_PARAMETER_STRING.to_owned(), &parameters)?;
        let root = input_to_u8_vec(root_value)?;

        Ok(Self { leaf_index, root })
    }
}
