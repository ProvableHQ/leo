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

use crate::{verify_record_commitment, LocalDataVerificationError, StateLeafValues, StateValues};
use leo_ast::Input as AstInput;

use snarkos_algorithms::commitment_tree::CommitmentMerklePath;
use snarkos_dpc::base_dpc::{
    instantiated::{Components, LocalDataCRH, LocalDataCommitment},
    parameters::SystemParameters,
};
use snarkos_models::{
    algorithms::{CommitmentScheme, CRH},
    dpc::DPCComponents,
};
use snarkos_utilities::{bytes::ToBytes, to_bytes, FromBytes};

use std::convert::TryFrom;

pub fn verify_local_data_commitment(
    system_parameters: &SystemParameters<Components>,
    ast_input: &AstInput,
) -> Result<bool, LocalDataVerificationError> {
    // verify record commitment
    let typed_record = ast_input.get_record();
    let dpc_record_values = verify_record_commitment(system_parameters, typed_record)?;
    let record_commitment: Vec<u8> = dpc_record_values.commitment;
    let record_serial_number: Vec<u8> = dpc_record_values.serial_number;

    // parse typed state values
    let typed_state = ast_input.get_state();
    let state_values = StateValues::try_from(typed_state)?;
    let leaf_index: u32 = state_values.leaf_index;
    let root: Vec<u8> = state_values.root;

    // parse typed state leaf values
    let typed_state_leaf = ast_input.get_state_leaf();
    let state_leaf_values = StateLeafValues::try_from(typed_state_leaf)?;
    let path: Vec<u8> = state_leaf_values.path;
    let memo: Vec<u8> = state_leaf_values.memo;
    let network_id: u8 = state_leaf_values.network_id;
    let leaf_randomness: Vec<u8> = state_leaf_values.leaf_randomness;

    // Select local data commitment input bytes
    let is_death = leaf_index < (Components::NUM_INPUT_RECORDS as u32);
    let input_bytes = if is_death {
        to_bytes![record_serial_number, record_commitment, memo, network_id]?
    } else {
        to_bytes![record_commitment, memo, network_id]?
    };

    // Construct local data commitment leaf
    let local_data_leaf_randomness = <LocalDataCommitment as CommitmentScheme>::Randomness::read(&leaf_randomness[..])?;
    let local_data_commitment_leaf = LocalDataCommitment::commit(
        &system_parameters.local_data_commitment,
        &input_bytes,
        &local_data_leaf_randomness,
    )?;

    // Construct record commitment merkle path
    let local_data_merkle_path = CommitmentMerklePath::<LocalDataCommitment, LocalDataCRH>::read(&path[..])?;

    // Check record commitment merkle path is valid for the given local data commitment root
    let local_data_commitment_root = <LocalDataCRH as CRH>::Output::read(&root[..])?;
    let result = local_data_merkle_path.verify(
        &system_parameters.local_data_crh,
        &local_data_commitment_root,
        &local_data_commitment_leaf,
    )?;

    Ok(result)
}
