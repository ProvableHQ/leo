use snarkos_curves::edwards_bls12::{EdwardsParameters, EdwardsProjective as EdwardsBls};
use snarkos_dpc::base_dpc::{
    instantiated::*,
    record_encryption::*,
    record_payload::RecordPayload,
    record_serializer::*,
    BaseDPCComponents,
    ExecuteContext,
    DPC,
};
use snarkos_models::{
    algorithms::{CommitmentScheme, CRH},
    dpc::{Record, RecordSerializerScheme},
    objects::AccountScheme,
};
use snarkos_objects::{
    Account,
    AccountViewKey,
    Block,
    BlockHeader,
    BlockHeaderHash,
    DPCTransactions,
    MerkleRootHash,
    PedersenMerkleRootHash,
    ProofOfSuccinctWork,
};
use snarkos_utilities::{bytes::ToBytes, rand::UniformRand, to_bytes};

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use snarkos_algorithms::commitment_tree::CommitmentMerklePath;
use snarkos_dpc::{DummyProgram, NoopProgram};
use snarkos_models::{
    algorithms::MerkleParameters,
    dpc::{DPCScheme, Program},
    objects::LedgerScheme,
};

#[test]
fn test_integrate_with_dpc() {
    use snarkos_testing::storage::*;
    type L = Ledger<Tx, CommitmentMerkleParameters>;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    // Specify network_id
    let network_id: u8 = 0;

    // Generate parameters for the ledger, commitment schemes, CRH, and the
    // "always-accept" program.
    let ledger_parameters = CommitmentMerkleParameters::setup(&mut rng);
    let system_parameters = InstantiatedDPC::generate_system_parameters(&mut rng).unwrap();
    let noop_program_snark_pp =
        InstantiatedDPC::generate_noop_program_snark_parameters(&system_parameters, &mut rng).unwrap();
    let dummy_program_snark_pp =
        InstantiatedDPC::generate_dummy_program_snark_parameters(&system_parameters, &mut rng).unwrap();

    let noop_program_id = to_bytes![
        ProgramVerificationKeyHash::hash(
            &system_parameters.program_verification_key_hash,
            &to_bytes![noop_program_snark_pp.verification_key].unwrap()
        )
        .unwrap()
    ]
    .unwrap();

    let dummy_program_id = to_bytes![
        ProgramVerificationKeyHash::hash(
            &system_parameters.program_verification_key_hash,
            &to_bytes![dummy_program_snark_pp.verification_key].unwrap()
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

    let genesis_block = Block {
        header: BlockHeader {
            previous_block_hash: BlockHeaderHash([0u8; 32]),
            merkle_root_hash: MerkleRootHash([0u8; 32]),
            time: 0,
            difficulty_target: 0x07FF_FFFF_FFFF_FFFF_u64,
            nonce: 0,
            pedersen_merkle_root_hash: PedersenMerkleRootHash([0u8; 32]),
            proof: ProofOfSuccinctWork::default(),
        },
        transactions: DPCTransactions::new(),
    };

    // Use genesis record, serial number, and memo to initialize the ledger.
    let ledger = initialize_test_blockchain::<Tx, CommitmentMerkleParameters>(ledger_parameters, genesis_block);

    let sn_nonce = SerialNumberNonce::hash(&system_parameters.serial_number_nonce, &[0u8; 1]).unwrap();
    // let old_record = DPC::generate_record(
    //     &system_parameters,
    //     &sn_nonce,
    //     &dummy_account.address,
    //     true,
    //     0,
    //     &RecordPayload::default(),
    //     &dummy_program_id,
    //     &dummy_program_id,
    //     &mut rng,
    // )
    //     .unwrap();

    let value = rng.gen();
    let payload: [u8; 32] = rng.gen();
    let old_record = DPC::generate_record(
        &system_parameters,
        &sn_nonce,
        &dummy_account.address,
        false,
        value,
        &RecordPayload::from_bytes(&payload),
        &noop_program_id,
        &noop_program_id,
        &mut rng,
    )
    .unwrap();

    // Set the input records for our transaction to be the initial dummy records.
    let old_records = vec![old_record.clone(); NUM_INPUT_RECORDS];
    let old_account_private_keys = vec![dummy_account.private_key.clone(); NUM_INPUT_RECORDS];

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

    let new_record_owners = vec![new_account.address.clone(); NUM_OUTPUT_RECORDS];
    let new_is_dummy_flags = vec![false; NUM_OUTPUT_RECORDS];
    let new_values = vec![10; NUM_OUTPUT_RECORDS];
    let new_payloads = vec![RecordPayload::default(); NUM_OUTPUT_RECORDS];
    let new_birth_program_ids = vec![noop_program_id.clone(); NUM_OUTPUT_RECORDS];
    let new_death_program_ids = vec![noop_program_id.clone(); NUM_OUTPUT_RECORDS];
    let memo = [0u8; 32];

    let context = <InstantiatedDPC as DPCScheme<L>>::execute_offline(
        &system_parameters,
        &old_records,
        &old_account_private_keys,
        &new_record_owners,
        &new_is_dummy_flags,
        &new_values,
        &new_payloads,
        &new_birth_program_ids,
        &new_death_program_ids,
        &memo,
        network_id,
        &mut rng,
    )
    .unwrap();

    let local_data = context.into_local_data();

    for (i, record) in local_data.old_records.iter().enumerate() {
        println!("{} : {}", i, record.is_dummy());
    }

    //////////////////////////////////////////////////////////////
    // Todo: parse state from file instead of DPC::generate_record
    // compare commitments
    /*
       let commitment = Commit(
           record.owner,
           record.value,
           record.payload,
           record.is_dummy,
           record.birth_program_id,
           record.death_program_id,
           record.serial_number_nonce,
           record.commitment_randomness,
       );

       record.commitment == commitment
    */

    let record_commitment_input = to_bytes![
        old_record.owner(),
        old_record.is_dummy(),
        old_record.value(),
        old_record.payload(),
        old_record.birth_program_id(),
        old_record.death_program_id(),
        old_record.serial_number_nonce()
    ]
    .unwrap();

    let record_commitment = RecordCommitment::commit(
        &system_parameters.record_commitment,
        &record_commitment_input,
        &old_record.commitment_randomness(),
    )
    .unwrap();

    assert_eq!(record_commitment, old_record.commitment());

    //////////////////////////////////////////////////////////////

    // Verify local data commitment

    // let leaf_index = 0;
    // let root = local_data.local_data_merkle_tree.root();
    //
    // let path = ledger.prove_cm(&old_record.commitment()).unwrap();
    // let memo = local_data.memorandum;
    // let network_id = local_data.network_id;
    // let leaf_randomness = local_data.local_data_commitment_randomizers[0].clone();

    // Verify that the local data commitment leaf is valid for the root

    // let path = ledger.prove_cm(&record.commitment()).unwrap();
    // let digest = ledger.digest().unwrap();
    // let verified = path.verify(&digest, &record.commitment()).unwrap();

    /////////////////////////////////////////////////

    // Generate the program proofs

    // let noop_program = NoopProgram::<_, <Components as BaseDPCComponents>::NoopProgramSNARK>::new(noop_program_id);
    // let dummy_program = DummyProgram::<_, <Components as BaseDPCComponents>::DummyProgramSNARK>::new(dummy_program_id);
    //
    // let mut old_proof_and_vk = vec![];
    // for i in 0..NUM_INPUT_RECORDS {
    //     let private_input = dummy_program
    //         .execute(
    //             &dummy_program_snark_pp.proving_key,
    //             &dummy_program_snark_pp.verification_key,
    //             &local_data,
    //             i as u8,
    //             &mut rng,
    //         )
    //         .unwrap();
    //
    //     old_proof_and_vk.push(private_input);
    // }
    //
    // let mut new_proof_and_vk = vec![];
    // for j in 0..NUM_OUTPUT_RECORDS {
    //     let private_input = noop_program
    //         .execute(
    //             &noop_program_snark_pp.proving_key,
    //             &noop_program_snark_pp.verification_key,
    //             &local_data,
    //             (NUM_INPUT_RECORDS + j) as u8,
    //             &mut rng,
    //         )
    //         .unwrap();
    //
    //     new_proof_and_vk.push(private_input);
    // }
    //
    // let ExecuteContext {
    //     system_parameters: _,
    //
    //     old_records,
    //     old_account_private_keys,
    //     old_serial_numbers,
    //     old_randomizers: _,
    //
    //     new_records,
    //     new_sn_nonce_randomness,
    //     new_commitments,
    //
    //     new_records_encryption_randomness,
    //     new_encrypted_records: _,
    //     new_encrypted_record_hashes,
    //
    //     program_commitment,
    //     program_randomness,
    //     local_data_merkle_tree,
    //     local_data_commitment_randomizers,
    //     value_balance,
    //     memorandum,
    //     network_id,
    // } = context;
    //
    // let local_data_root = local_data_merkle_tree.root();

    // Verify that the local data commitment leaf is valid for the root

    // let local_data_commitment = LocalDataCommitment::commit(
    //     &system_parameters.local_data_commitment,
    //
    // )

    // let merkle = CommitmentMerklePath::verify(
    //     system_parameters.local_data_commitment
    //      state.root
    //      state.path
    // )

    // system_parameters.local_data_commitment
}
