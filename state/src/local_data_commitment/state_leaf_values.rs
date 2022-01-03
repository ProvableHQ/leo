// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{find_input, input_to_bytes, input_to_integer_string};
use leo_ast::StateLeaf as AstStateLeaf;
use leo_errors::{LeoError, Result, StateError};

use std::convert::TryFrom;

static PATH_PARAMETER_STRING: &str = "path";
static MEMO_PARAMETER_STRING: &str = "memo";
static NETWORK_ID_PARAMETER_STRING: &str = "network_id";
static LEAF_RANDOMNESS_PARAMETER_STRING: &str = "leaf_randomness";

/// The serialized values included in the state leaf.
/// A new [`StateLeafValues`] type can be constructed from an [`AstStateLeaf`] type.
pub struct StateLeafValues {
    pub path: Vec<u8>,
    pub memo: Vec<u8>,
    pub network_id: u8,
    pub leaf_randomness: Vec<u8>,
}

impl TryFrom<&AstStateLeaf> for StateLeafValues {
    type Error = LeoError;

    fn try_from(ast_state_leaf: &AstStateLeaf) -> Result<Self> {
        let parameters = ast_state_leaf.values();

        // Lookup path
        let path_value = find_input(PATH_PARAMETER_STRING.to_owned(), &parameters)?;
        let path = input_to_bytes(path_value)?;

        // Lookup memo
        let memo_value = find_input(MEMO_PARAMETER_STRING.to_owned(), &parameters)?;
        let memo = input_to_bytes(memo_value)?;

        // Lookup network id
        let network_id_value = find_input(NETWORK_ID_PARAMETER_STRING.to_owned(), &parameters)?;
        let network_id = input_to_integer_string(network_id_value)?
            .parse::<u8>()
            .map_err(StateError::parse_int_error)?;

        // Lookup leaf randomness
        let leaf_randomness_value = find_input(LEAF_RANDOMNESS_PARAMETER_STRING.to_owned(), &parameters)?;
        let leaf_randomness = input_to_bytes(leaf_randomness_value)?;

        Ok(Self {
            path,
            memo,
            network_id,
            leaf_randomness,
        })
    }
}
