// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use super::*;

use snarkvm::{
    ledger::store::helpers::memory::ConsensusMemory,
    prelude::{
        Block,
        CryptoRng,
        Field,
        Header,
        Metadata,
        Program,
        Result,
        VM,
        Zero,
        bail,
        ensure,
        store::{ConsensusStorage, ConsensusStore},
    },
    synthesizer::program::FinalizeGlobalState,
};

const GENESIS_PRIVATE_KEY: &str = "APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH";

pub(super) fn initialize_vm<N: Network>(
    programs: Vec<Program<N>>,
) -> Result<(VM<N, ConsensusMemory<N>>, PrivateKey<N>)> {
    // Initialize an RNG with a fixed seed.
    let rng = &mut ChaChaRng::seed_from_u64(123456789);
    // Initialize the genesis private key.
    let genesis_private_key = PrivateKey::<N>::from_str(GENESIS_PRIVATE_KEY).unwrap();
    // Initialize the VM.
    let vm = VM::<N, ConsensusMemory<N>>::from(ConsensusStore::<N, ConsensusMemory<N>>::open(None)?)?;
    // Initialize the genesis block.
    let genesis = vm.genesis_beacon(&genesis_private_key, rng)?;
    // Add the genesis block to the VM.
    vm.add_next_block(&genesis)?;

    // Deploy the programs to the VM.
    for program in &programs {
        // Create the deployment.
        let deployment = vm.deploy(&genesis_private_key, program, None, 0, None, rng)?;
        // Create a new block and check that the transaction was accepted.
        let (block, is_accepted) = construct_next_block(&vm, &genesis_private_key, deployment, rng)?;
        ensure!(is_accepted, "Failed to deploy the program");
        // Add the block to the VM.
        vm.add_next_block(&block)?;
    }

    Ok((vm, genesis_private_key))
}

// A helper function that takes a single transaction and creates a new block.
// The function also tells whether the transaction was accepted or rejected.
#[allow(clippy::too_many_arguments)]
pub(super) fn construct_next_block<N: Network, C: ConsensusStorage<N>, R: Rng + CryptoRng>(
    vm: &VM<N, C>,
    private_key: &PrivateKey<N>,
    transaction: Transaction<N>,
    rng: &mut R,
) -> Result<(Block<N>, bool)> {
    // Speculate on the transaction.
    let time_since_last_block = N::BLOCK_TIME as i64;
    let (ratifications, transactions, aborted_transaction_ids, ratified_finalize_operations) = vm.speculate(
        construct_finalize_global_state(&vm)?,
        time_since_last_block,
        Some(0u64),
        vec![],
        &None.into(),
        [transaction].iter(),
        rng,
    )?;
    let is_accepted = match transactions.iter().next() {
        Some(confirmed_transaction) => confirmed_transaction.is_accepted(),
        None => false,
    };

    // Get the most recent block.
    let block_hash = vm.block_store().get_block_hash(vm.block_store().max_height().unwrap()).unwrap().unwrap();
    let previous_block = vm.block_store().get_block(&block_hash).unwrap().unwrap();

    // Construct the metadata associated with the block.
    let metadata = Metadata::new(
        N::ID,
        previous_block.round() + 1,
        previous_block.height() + 1,
        0,
        0,
        N::GENESIS_COINBASE_TARGET,
        N::GENESIS_PROOF_TARGET,
        previous_block.last_coinbase_target(),
        previous_block.last_coinbase_timestamp(),
        previous_block.timestamp().saturating_add(time_since_last_block),
    )?;
    // Construct the block header.
    let header = Header::from(
        vm.block_store().current_state_root(),
        transactions.to_transactions_root().unwrap(),
        transactions.to_finalize_root(ratified_finalize_operations).unwrap(),
        ratifications.to_ratifications_root().unwrap(),
        Field::zero(),
        Field::zero(),
        metadata,
    )?;

    // Construct the new block.
    Ok((
        Block::new_beacon(
            private_key,
            previous_block.hash(),
            header,
            ratifications,
            None.into(),
            vec![],
            transactions,
            aborted_transaction_ids,
            rng,
        )?,
        is_accepted,
    ))
}

// A helper function to construct the `FinalizeGlobalState` from the current `VM` state.
fn construct_finalize_global_state<N: Network, C: ConsensusStorage<N>>(vm: &VM<N, C>) -> Result<FinalizeGlobalState> {
    // Retrieve the latest block.
    let block_height = match vm.block_store().max_height() {
        Some(height) => height,
        None => bail!("Failed to retrieve the latest block height"),
    };
    let latest_block_hash = match vm.block_store().get_block_hash(block_height)? {
        Some(hash) => hash,
        None => bail!("Failed to retrieve the latest block hash"),
    };
    let latest_block = match vm.block_store().get_block(&latest_block_hash)? {
        Some(block) => block,
        None => bail!("Failed to retrieve the latest block"),
    };
    // Retrieve the latest round.
    let latest_round = latest_block.round();
    // Retrieve the latest height.
    let latest_height = latest_block.height();
    // Retrieve the latest cumulative weight.
    let latest_cumulative_weight = latest_block.cumulative_weight();

    // Compute the next round number./
    let next_round = latest_round.saturating_add(1);
    // Compute the next height.
    let next_height = latest_height.saturating_add(1);

    // Construct the finalize state.
    FinalizeGlobalState::new::<N>(next_round, next_height, latest_cumulative_weight, 0u128, latest_block.hash())
}
