// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{find_input, input_to_integer_string, input_to_u8_vec, StateLeafValuesError};
use leo_core_ast::StateLeaf as TypedStateLeaf;

use std::convert::TryFrom;

static PATH_PARAMETER_STRING: &str = "path";
static MEMO_PARAMETER_STRING: &str = "memo";
static NETWORK_ID_PARAMETER_STRING: &str = "network_id";
static LEAF_RANDOMNESS_PARAMETER_STRING: &str = "leaf_randomness";

pub struct StateLeafValues {
    pub path: Vec<u8>,
    pub memo: Vec<u8>,
    pub network_id: u8,
    pub leaf_randomness: Vec<u8>,
}

impl TryFrom<&TypedStateLeaf> for StateLeafValues {
    type Error = StateLeafValuesError;

    fn try_from(state_leaf: &TypedStateLeaf) -> Result<Self, Self::Error> {
        let parameters = state_leaf.values();

        // Lookup path
        let path_value = find_input(PATH_PARAMETER_STRING.to_owned(), &parameters)?;
        let path = input_to_u8_vec(path_value)?;

        // Lookup memo
        let memo_value = find_input(MEMO_PARAMETER_STRING.to_owned(), &parameters)?;
        let memo = input_to_u8_vec(memo_value)?;

        // Lookup network id
        let network_id_value = find_input(NETWORK_ID_PARAMETER_STRING.to_owned(), &parameters)?;
        let network_id = input_to_integer_string(network_id_value)?.parse::<u8>()?;

        // Lookup leaf randomness
        let leaf_randomness_value = find_input(LEAF_RANDOMNESS_PARAMETER_STRING.to_owned(), &parameters)?;
        let leaf_randomness = input_to_u8_vec(leaf_randomness_value)?;

        Ok(Self {
            path,
            memo,
            network_id,
            leaf_randomness,
        })
    }
}
