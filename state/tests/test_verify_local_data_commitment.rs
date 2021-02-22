// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use leo_ast::Input;
use leo_input::LeoInputParser;
use leo_state::verify_local_data_commitment;

use snarkvm_dpc::base_dpc::{instantiated::*, record_payload::RecordPayload, DPC};
use snarkvm_models::{
    algorithms::{CommitmentScheme, CRH},
    dpc::Record,
    objects::AccountScheme,
};
use snarkvm_objects::Account;
use snarkvm_utilities::{bytes::ToBytes, to_bytes};

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use snarkvm_models::dpc::DPCScheme;
use snarkvm_storage::Ledger;

// TODO (Collin): Update input to reflect new parameter ordering.
#[test]
#[ignore]
fn test_verify_local_data_commitment_from_file() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    // Generate parameters for the record commitment scheme
    let system_parameters = InstantiatedDPC::generate_system_parameters(&mut rng).unwrap();

    // Load test record state file from `inputs/test.state`
    let file_bytes = include_bytes!("inputs/test_state.state");
    let file_string = String::from_utf8_lossy(file_bytes);
    let file = LeoInputParser::parse_file(&file_string).unwrap();

    let mut program_input = Input::new();
    program_input.parse_state(file).unwrap();

    // check record state is correct by verifying commitment
    let result = verify_local_data_commitment(&system_parameters, &program_input).unwrap();

    assert!(result);
}

#[test]
#[ignore]
fn test_generate_values_from_dpc() {
    type L = Ledger<Tx, CommitmentMerkleParameters>;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    // Specify network_id
    let network_id: u8 = 0;

    // Generate parameters for the ledger, commitment schemes, CRH, and the
    // "always-accept" program.
    let system_parameters = InstantiatedDPC::generate_system_parameters(&mut rng).unwrap();
    let noop_program_snark_pp =
        InstantiatedDPC::generate_noop_program_snark_parameters(&system_parameters, &mut rng).unwrap();

    let noop_program_id = to_bytes![
        ProgramVerificationKeyCRH::hash(
            &system_parameters.program_verification_key_crh,
            &to_bytes![noop_program_snark_pp.verification_key].unwrap()
        )
        .unwrap()
    ]
    .unwrap();

    let signature_parameters = &system_parameters.account_signature;
    let commitment_parameters = &system_parameters.account_commitment;
    let encryption_parameters = &system_parameters.account_encryption;

    // Generate metadata and an account for a dummy initial record.
    let dummy_account = Account::new(
        signature_parameters,
        commitment_parameters,
        encryption_parameters,
        &mut rng,
    )
    .unwrap();

    let sn_nonce = SerialNumberNonce::hash(&system_parameters.serial_number_nonce, &[0u8; 1]).unwrap();
    let value = rng.gen();
    let payload: [u8; 32] = rng.gen();

    let old_record = DPC::generate_record(
        &system_parameters,
        sn_nonce,
        dummy_account.address,
        false,
        value,
        RecordPayload::from_bytes(&payload),
        noop_program_id.clone(),
        noop_program_id.clone(),
        &mut rng,
    )
    .unwrap();

    // Set the input records for our transaction to be the initial dummy records.
    let old_records = vec![old_record; NUM_INPUT_RECORDS];
    let old_account_private_keys = vec![dummy_account.private_key; NUM_INPUT_RECORDS];

    // Construct new records.

    // Create an account for an actual new record.

    let new_account = Account::new(
        signature_parameters,
        commitment_parameters,
        encryption_parameters,
        &mut rng,
    )
    .unwrap();

    // Set the new record's program to be the "always-accept" program.

    let new_record_owners = vec![new_account.address; NUM_OUTPUT_RECORDS];
    let new_is_dummy_flags = vec![false; NUM_OUTPUT_RECORDS];
    let new_values = vec![10; NUM_OUTPUT_RECORDS];
    let new_payloads = vec![RecordPayload::default(); NUM_OUTPUT_RECORDS];
    let new_birth_program_ids = vec![noop_program_id.clone(); NUM_OUTPUT_RECORDS];
    let new_death_program_ids = vec![noop_program_id; NUM_OUTPUT_RECORDS];
    let memo = [0u8; 32];

    let context = <InstantiatedDPC as DPCScheme<L>>::execute_offline(
        system_parameters.clone(),
        old_records,
        old_account_private_keys,
        new_record_owners,
        &new_is_dummy_flags,
        &new_values,
        new_payloads,
        new_birth_program_ids,
        new_death_program_ids,
        memo,
        network_id,
        &mut rng,
    )
    .unwrap();

    let local_data = context.into_local_data();
    let leaf_index = 0;
    let record = &local_data.old_records[leaf_index];

    let root = local_data.local_data_merkle_tree.root();

    let serial_number = local_data.old_serial_numbers[0];
    let serial_number_bytes = to_bytes![serial_number].unwrap();

    let memorandum = local_data.memorandum;
    let network_id = local_data.network_id;
    let input_bytes = to_bytes![serial_number, record.commitment(), memorandum, network_id].unwrap();
    let leaf_randomness = local_data.local_data_commitment_randomizers[0];

    let old_record_leaf = <LocalDataCommitment as CommitmentScheme>::commit(
        &system_parameters.local_data_commitment,
        &input_bytes,
        &leaf_randomness,
    )
    .unwrap();

    // generate the path

    let path = local_data
        .local_data_merkle_tree
        .generate_proof(&old_record_leaf)
        .unwrap();

    println!("////////////////////////////////////////////////////");
    println!();
    println!("[state]");
    println!("leaf index {}", leaf_index);
    println!("root {:?}", to_bytes![root].unwrap());
    println!();
    println!("[record]");
    println!(
        "serial number {:?} len {}",
        serial_number_bytes,
        serial_number_bytes.len()
    );
    println!("commitment {:?}", to_bytes![record.commitment()].unwrap());
    println!("owner {}", record.owner());
    println!("is_dummy {:?}", record.is_dummy());
    println!("value {:?}", record.value());
    println!("payload {:?}", record.payload());
    println!("birth_program_id {:?}", record.birth_program_id());
    println!("death_program_id {:?}", record.death_program_id());
    println!(
        "serial number nonce {:?}",
        to_bytes![record.serial_number_nonce()].unwrap()
    );
    println!(
        "commitment randomness {:?}",
        to_bytes![record.commitment_randomness()].unwrap()
    );
    println!();
    println!("[state_leaf]");
    println!("path {:?}", to_bytes![path].unwrap());
    println!("memo {:?}", memorandum);
    println!("network id {:?}", network_id);
    println!("leaf randomness {:?}", to_bytes![leaf_randomness].unwrap());
    println!();
    println!("////////////////////////////////////////////////////");
}
