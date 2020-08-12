use crate::{verify_record_commitment::verify_record_commitment, LocalDataVerificationError};
use leo_typed::Input as TypedInput;

use snarkos_algorithms::commitment_tree::CommitmentMerklePath;
use snarkos_dpc::base_dpc::instantiated::{Components, LocalDataCRH, LocalDataCommitment, RecordCommitment};
use snarkos_models::{
    algorithms::{CommitmentScheme, CRH},
    curves::Fp256,
    dpc::DPCComponents,
};
use snarkos_utilities::{bytes::ToBytes, to_bytes, FromBytes};

pub fn verify_local_data_commitment(
    typed_input: &TypedInput,
    record_commitment_params: RecordCommitment,
    local_data_commitment_params: LocalDataCommitment,
    local_data_crh_params: LocalDataCRH,
) -> Result<bool, LocalDataVerificationError> {
    // verify record commitment
    let typed_record = typed_input.get_record();
    let dpc_record_values = verify_record_commitment(typed_record, record_commitment_params).unwrap();

    let record_commitment: Vec<u8> = dpc_record_values.commitment;
    let record_serial_number: Vec<u8> = dpc_record_values.serial_number;

    // parse typed state values
    let leaf_index: u32 = 0;
    let root: Vec<u8> = vec![];

    // parse typed state leaf values
    let _path: Vec<Vec<u8>> = vec![];
    let memo: Vec<u8> = vec![];
    let network_id: Vec<u8> = vec![];
    let leaf_randomness: Vec<u8> = vec![];

    // Select local data commitment input bytes
    let is_death = leaf_index < (Components::NUM_INPUT_RECORDS as u32);

    let input_bytes = if is_death {
        to_bytes![record_serial_number, record_commitment, memo, network_id].unwrap()
    } else {
        to_bytes![record_commitment, memo, network_id].unwrap()
    };

    // Construct local data commitment leaf
    let leaf_randomness = Fp256::read(&leaf_randomness[..]).unwrap();

    let local_data_commitment_leaf =
        LocalDataCommitment::commit(&local_data_commitment_params, &input_bytes, &leaf_randomness).unwrap();

    // Construct record commitment merkle path

    // let local_data_merkle_path = CommitmentMerklePath::from(path); // Ideally we want something like this

    // Initialize failing blank values for now
    let leaves = (
        <LocalDataCommitment as CommitmentScheme>::Output::default(),
        <LocalDataCommitment as CommitmentScheme>::Output::default(),
    );

    let inner_hashes = (
        <LocalDataCRH as CRH>::Output::default(),
        <LocalDataCRH as CRH>::Output::default(),
    );

    let local_data_merkle_path = CommitmentMerklePath::<LocalDataCommitment, LocalDataCRH> {
        leaves,
        inner_hashes,
        parameters: local_data_crh_params,
    };

    // Check record commitment merkle path is valid for the given local data commitment root
    let local_data_commitment_root = Fp256::read(&root[..]).unwrap();

    let result = local_data_merkle_path
        .verify(&local_data_commitment_root, &local_data_commitment_leaf)
        .unwrap();

    Ok(result)
}
