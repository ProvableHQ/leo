// Copyright (C) 2019-2026 Provable Inc.
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

use snarkvm::prelude::{Identifier, LimitedWriter, Plaintext, PrivateKey, Program, ToBytes, Transaction, VM};

use axum::{Json, extract::rejection::JsonRejection};

use anyhow::{Context, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{str::FromStr, sync::atomic::Ordering};

use rayon::prelude::*;

/// Deserialize a CSV string into a vector of strings.
fn de_csv<'de, D>(de: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(if s.trim().is_empty() { Vec::new() } else { s.split(',').map(|x| x.trim().to_string()).collect() })
}

/// The `get_blocks` query object.
#[derive(Deserialize, Serialize)]
pub(crate) struct BlockRange {
    /// The starting block height (inclusive).
    start: u32,
    /// The ending block height (exclusive).
    end: u32,
}

/// The query object for `get_mapping_value` and `get_mapping_values`.
#[derive(Copy, Clone, Deserialize, Serialize)]
pub(crate) struct Metadata {
    metadata: Option<bool>,
    all: Option<bool>,
}

/// The query object for `transaction_broadcast`.
#[derive(Copy, Clone, Deserialize, Serialize)]
pub(crate) struct CheckTransaction {
    check_transaction: Option<bool>,
}

/// The query object for `get_state_paths_for_commitments`.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Commitments {
    #[serde(deserialize_with = "de_csv")]
    commitments: Vec<String>,
}

/// The request object for creating a new block.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct CreateBlockRequest {
    /// number of blocks to create.
    pub num_blocks: Option<u32>,
}

impl<N: Network, C: ConsensusStorage<N>> Rest<N, C> {
    /// Get /<network>/consensus_version
    pub(crate) async fn get_consensus_version(State(rest): State<Self>) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(N::CONSENSUS_VERSION(rest.ledger.latest_height())? as u16))
    }

    /// GET /<network>/block/height/latest
    pub(crate) async fn get_block_height_latest(State(rest): State<Self>) -> ErasedJson {
        ErasedJson::new(rest.ledger.latest_height())
    }

    /// GET /<network>/block/hash/latest
    pub(crate) async fn get_block_hash_latest(State(rest): State<Self>) -> ErasedJson {
        ErasedJson::new(rest.ledger.latest_hash())
    }

    /// GET /<network>/block/latest
    pub(crate) async fn get_block_latest(State(rest): State<Self>) -> ErasedJson {
        ErasedJson::new(rest.ledger.latest_block())
    }

    /// GET /<network>/block/{height}
    /// GET /<network>/block/{blockHash}
    pub(crate) async fn get_block(
        State(rest): State<Self>,
        Path(height_or_hash): Path<String>,
    ) -> Result<ErasedJson, RestError> {
        // Manually parse the height or the height of the hash, axum doesn't support different types
        // for the same path param.
        let block = if let Ok(height) = height_or_hash.parse::<u32>() {
            rest.ledger.get_block(height).with_context(|| "Failed to get block by height")?
        } else if let Ok(hash) = height_or_hash.parse::<N::BlockHash>() {
            rest.ledger.get_block_by_hash(&hash).with_context(|| "Failed to get block by hash")?
        } else {
            return Err(RestError::bad_request(anyhow!(
                "invalid input, it is neither a block height nor a block hash"
            )));
        };

        Ok(ErasedJson::new(block))
    }

    /// GET /<network>/blocks?start={start_height}&end={end_height}
    pub(crate) async fn get_blocks(
        State(rest): State<Self>,
        Query(block_range): Query<BlockRange>,
    ) -> Result<ErasedJson, RestError> {
        let start_height = block_range.start;
        let end_height = block_range.end;

        const MAX_BLOCK_RANGE: u32 = 50;

        // Ensure the end height is greater than the start height.
        if start_height > end_height {
            return Err(RestError::bad_request(anyhow!("Invalid block range")));
        }

        // Ensure the block range is bounded.
        if end_height - start_height > MAX_BLOCK_RANGE {
            return Err(RestError::bad_request(anyhow!(
                "Cannot request more than {MAX_BLOCK_RANGE} blocks per call (requested {})",
                end_height - start_height
            )));
        }

        // Prepare a closure for the blocking work.
        let get_json_blocks = move || -> Result<ErasedJson, RestError> {
            let blocks = (start_height..end_height)
                .into_par_iter()
                .map(|height| rest.ledger.get_block(height))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(ErasedJson::new(blocks))
        };

        // Fetch the blocks from ledger and serialize to json.
        match tokio::task::spawn_blocking(get_json_blocks).await {
            Ok(json) => json,
            Err(err) => {
                let err: anyhow::Error = err.into();

                Err(RestError::internal_server_error(
                    err.context(format!("Failed to get blocks '{start_height}..{end_height}'")),
                ))
            }
        }
    }

    /// GET /<network>/height/{blockHash}
    pub(crate) async fn get_height(
        State(rest): State<Self>,
        Path(hash): Path<N::BlockHash>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.get_height(&hash)?))
    }

    /// GET /<network>/block/{height}/header
    pub(crate) async fn get_block_header(
        State(rest): State<Self>,
        Path(height): Path<u32>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.get_header(height)?))
    }

    /// GET /<network>/block/{height}/transactions
    pub(crate) async fn get_block_transactions(
        State(rest): State<Self>,
        Path(height): Path<u32>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.get_transactions(height)?))
    }

    /// GET /<network>/transaction/{transactionID}
    pub(crate) async fn get_transaction(
        State(rest): State<Self>,
        Path(tx_id): Path<N::TransactionID>,
    ) -> Result<ErasedJson, RestError> {
        // Ledger returns a generic anyhow::Error, so checking the message is the only way to parse it.
        Ok(ErasedJson::new(rest.ledger.get_transaction(tx_id).map_err(|err| {
            if err.to_string().contains("Missing") { RestError::not_found(err) } else { RestError::from(err) }
        })?))
    }

    /// GET /<network>/transaction/confirmed/{transactionID}
    pub(crate) async fn get_confirmed_transaction(
        State(rest): State<Self>,
        Path(tx_id): Path<N::TransactionID>,
    ) -> Result<ErasedJson, RestError> {
        // Ledger returns a generic anyhow::Error, so checking the message is the only way to parse it.
        Ok(ErasedJson::new(rest.ledger.get_confirmed_transaction(tx_id).map_err(|err| {
            if err.to_string().contains("Missing") { RestError::not_found(err) } else { RestError::from(err) }
        })?))
    }

    /// GET /<network>/transaction/unconfirmed/{transactionID}
    pub(crate) async fn get_unconfirmed_transaction(
        State(rest): State<Self>,
        Path(tx_id): Path<N::TransactionID>,
    ) -> Result<ErasedJson, RestError> {
        // Ledger returns a generic anyhow::Error, so checking the message is the only way to parse it.
        Ok(ErasedJson::new(rest.ledger.get_unconfirmed_transaction(&tx_id).map_err(|err| {
            if err.to_string().contains("Missing") { RestError::not_found(err) } else { RestError::from(err) }
        })?))
    }

    /// GET /<network>/program/{programID}
    /// GET /<network>/program/{programID}?metadata={true}
    pub(crate) async fn get_program(
        State(rest): State<Self>,
        Path(id): Path<ProgramID<N>>,
        metadata: Query<Metadata>,
    ) -> Result<ErasedJson, RestError> {
        // Get the program from the ledger.
        let program = rest.ledger.get_program(id).with_context(|| format!("Failed to find program `{id}`"))?;
        // Check if metadata is requested and return the program with metadata if so.
        if metadata.metadata.unwrap_or(false) {
            // Get the edition of the program.
            let edition = rest.ledger.get_latest_edition_for_program(&id)?;
            return rest.return_program_with_metadata(program, edition);
        }
        // Return the program without metadata.
        Ok(ErasedJson::new(program))
    }

    /// GET /<network>/program/{programID}/{edition}
    /// GET /<network>/program/{programID}/{edition}?metadata={true}
    pub(crate) async fn get_program_for_edition(
        State(rest): State<Self>,
        Path((id, edition)): Path<(ProgramID<N>, u16)>,
        metadata: Query<Metadata>,
    ) -> Result<ErasedJson, RestError> {
        // Get the program from the ledger.
        match rest
            .ledger
            .try_get_program_for_edition(&id, edition)
            .with_context(|| format!("Failed get program `{id}` for edition {edition}"))?
        {
            Some(program) => {
                // Check if metadata is requested and return the program with metadata if so.
                if metadata.metadata.unwrap_or(false) {
                    rest.return_program_with_metadata(program, edition)
                } else {
                    Ok(ErasedJson::new(program))
                }
            }
            None => Err(RestError::not_found(anyhow!("No program `{id}` exists for edition {edition}"))),
        }
    }

    /// A helper function to return the program and its metadata.
    /// This function is used in the `get_program` and `get_program_for_edition` functions.
    fn return_program_with_metadata(&self, program: Program<N>, edition: u16) -> Result<ErasedJson, RestError> {
        let id = program.id();
        // Get the transaction ID associated with the program and edition.
        let tx_id = self.ledger.find_transaction_id_from_program_id_and_edition(id, edition)?;
        // Get the optional program owner associated with the program.
        // Note: The owner is only available after `ConsensusVersion::V9`.
        let program_owner = match &tx_id {
            Some(tid) => self
                .ledger
                .vm()
                .block_store()
                .transaction_store()
                .deployment_store()
                .get_deployment(tid)?
                .and_then(|deployment| deployment.program_owner()),
            None => None,
        };
        Ok(ErasedJson::new(json!({
            "program": program,
            "edition": edition,
            "transaction_id": tx_id,
            "program_owner": program_owner,
        })))
    }

    /// GET /<network>/program/{programID}/latest_edition
    pub(crate) async fn get_latest_program_edition(
        State(rest): State<Self>,
        Path(id): Path<ProgramID<N>>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.get_latest_edition_for_program(&id)?))
    }

    /// GET /<network>/program/{programID}/mappings
    pub(crate) async fn get_mapping_names(
        State(rest): State<Self>,
        Path(id): Path<ProgramID<N>>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.vm().finalize_store().get_mapping_names_confirmed(&id)?))
    }

    /// GET /<network>/program/{programID}/mapping/{mappingName}/{mappingKey}
    /// GET /<network>/program/{programID}/mapping/{mappingName}/{mappingKey}?metadata={true}
    pub(crate) async fn get_mapping_value(
        State(rest): State<Self>,
        Path((id, name, key)): Path<(ProgramID<N>, Identifier<N>, Plaintext<N>)>,
        metadata: Query<Metadata>,
    ) -> Result<ErasedJson, RestError> {
        // Retrieve the mapping value.
        let mapping_value = rest.ledger.vm().finalize_store().get_value_confirmed(id, name, &key)?;

        // Check if metadata is requested and return the value with metadata if so.
        if metadata.metadata.unwrap_or(false) {
            return Ok(ErasedJson::new(json!({
                "data": mapping_value,
                "height": rest.ledger.latest_height(),
            })));
        }

        // Return the value without metadata.
        Ok(ErasedJson::new(mapping_value))
    }

    /// GET /<network>/program/{programID}/mapping/{mappingName}?all={true}&metadata={true}
    pub(crate) async fn get_mapping_values(
        State(rest): State<Self>,
        Path((id, name)): Path<(ProgramID<N>, Identifier<N>)>,
        metadata: Query<Metadata>,
    ) -> Result<ErasedJson, RestError> {
        // Return an error if the `all` query parameter is not set to `true`.
        if metadata.all != Some(true) {
            return Err(RestError::bad_request(anyhow!(
                "Invalid query parameter. At this time, 'all=true' must be included"
            )));
        }

        // Retrieve the latest height.
        let height = rest.ledger.latest_height();

        // Retrieve all the mapping values from the mapping.
        match tokio::task::spawn_blocking(move || rest.ledger.vm().finalize_store().get_mapping_confirmed(id, name))
            .await
        {
            Ok(Ok(mapping_values)) => {
                // Check if metadata is requested and return the mapping with metadata if so.
                if metadata.metadata.unwrap_or(false) {
                    return Ok(ErasedJson::new(json!({
                        "data": mapping_values,
                        "height": height,
                    })));
                }

                // Return the full mapping without metadata.
                Ok(ErasedJson::new(mapping_values))
            }
            Ok(Err(err)) => Err(RestError::internal_server_error(err.context("Unable to read mapping"))),
            Err(err) => Err(RestError::internal_server_error(anyhow!("Tokio error: {err}"))),
        }
    }

    /// GET /<network>/statePath/{commitment}
    pub(crate) async fn get_state_path_for_commitment(
        State(rest): State<Self>,
        Path(commitment): Path<Field<N>>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.get_state_path_for_commitment(&commitment)?))
    }

    /// GET /<network>/statePaths?commitments=cm1,cm2,...
    pub(crate) async fn get_state_paths_for_commitments(
        State(rest): State<Self>,
        Query(commitments): Query<Commitments>,
    ) -> Result<ErasedJson, RestError> {
        // Retrieve the number of commitments.
        let num_commitments = commitments.commitments.len();
        // Return an error if no commitments are provided.
        if num_commitments == 0 {
            return Err(RestError::unprocessable_entity(anyhow!("No commitments provided")));
        }
        // Return an error if the number of commitments exceeds the maximum allowed.
        if num_commitments > N::MAX_INPUTS {
            return Err(RestError::unprocessable_entity(anyhow!(
                "Too many commitments provided (max: {}, got: {})",
                N::MAX_INPUTS,
                num_commitments
            )));
        }

        // Deserialize the commitments from the query.
        let commitments = commitments
            .commitments
            .iter()
            .map(|s| {
                s.parse::<Field<N>>()
                    .map_err(|err| RestError::unprocessable_entity(err.context(format!("Invalid commitment: {s}"))))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ErasedJson::new(rest.ledger.get_state_paths_for_commitments(&commitments)?))
    }

    /// GET /<network>/stateRoot/latest
    pub(crate) async fn get_state_root_latest(State(rest): State<Self>) -> ErasedJson {
        ErasedJson::new(rest.ledger.latest_state_root())
    }

    /// GET /<network>/stateRoot/{height}
    pub(crate) async fn get_state_root(
        State(rest): State<Self>,
        Path(height): Path<u32>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.get_state_root(height)?))
    }

    /// GET /<network>/find/blockHash/{transactionID}
    pub(crate) async fn find_block_hash(
        State(rest): State<Self>,
        Path(tx_id): Path<N::TransactionID>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.find_block_hash(&tx_id)?))
    }

    /// GET /<network>/find/blockHeight/{stateRoot}
    pub(crate) async fn find_block_height_from_state_root(
        State(rest): State<Self>,
        Path(state_root): Path<N::StateRoot>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.find_block_height_from_state_root(state_root)?))
    }

    /// GET /<network>/find/transactionID/deployment/{programID}
    pub(crate) async fn find_latest_transaction_id_from_program_id(
        State(rest): State<Self>,
        Path(program_id): Path<ProgramID<N>>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.find_latest_transaction_id_from_program_id(&program_id)?))
    }

    /// GET /<network>/find/transactionID/deployment/{programID}/{edition}
    pub(crate) async fn find_transaction_id_from_program_id_and_edition(
        State(rest): State<Self>,
        Path((program_id, edition)): Path<(ProgramID<N>, u16)>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.find_transaction_id_from_program_id_and_edition(&program_id, edition)?))
    }

    /// GET /<network>/find/transactionID/{transitionID}
    pub(crate) async fn find_transaction_id_from_transition_id(
        State(rest): State<Self>,
        Path(transition_id): Path<N::TransitionID>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.find_transaction_id_from_transition_id(&transition_id)?))
    }

    /// GET /<network>/find/transitionID/{inputOrOutputID}
    pub(crate) async fn find_transition_id(
        State(rest): State<Self>,
        Path(input_or_output_id): Path<Field<N>>,
    ) -> Result<ErasedJson, RestError> {
        Ok(ErasedJson::new(rest.ledger.find_transition_id(&input_or_output_id)?))
    }

    // /// POST /<network>/transaction/broadcast
    // /// POST /<network>/transaction/broadcast?check_transaction={true}
    pub(crate) async fn transaction_broadcast(
        State(rest): State<Self>,
        check_transaction: Query<CheckTransaction>,
        json_result: Result<Json<Transaction<N>>, JsonRejection>,
    ) -> Result<impl axum::response::IntoResponse, RestError> {
        let Json(tx) = match json_result {
            Ok(json) => json,
            Err(JsonRejection::JsonDataError(err)) => {
                // For JsonDataError, return 422 to let transaction validation handle it.
                return Err(RestError::unprocessable_entity(anyhow!("Invalid transaction data: {err}")));
            }
            Err(other_rejection) => return Err(other_rejection.into()),
        };
        let tx_id = tx.id();

        // If the transaction exceeds the transaction size limit, return an error.
        // The buffer is initially roughly sized to hold a `transfer_public`.
        // Most transactions will be smaller and this reduces unnecessary allocations.
        let buffer = Vec::with_capacity(3000);
        if tx.write_le(LimitedWriter::new(buffer, N::MAX_TRANSACTION_SIZE)).is_err() {
            return Err(RestError::bad_request(anyhow!("Transaction size exceeds the byte limit")));
        }

        // Determine if we need to check the transaction.
        let check_transaction = check_transaction.check_transaction.unwrap_or(true);

        if check_transaction {
            // Select counter and limit based on transaction type.
            let (counter, limit, err_msg) = if tx.is_execute() {
                (
                    &rest.num_verifying_executions,
                    VM::<N, C>::MAX_PARALLEL_EXECUTE_VERIFICATIONS,
                    "Too many execution verifications in progress",
                )
            } else {
                (
                    &rest.num_verifying_deploys,
                    VM::<N, C>::MAX_PARALLEL_DEPLOY_VERIFICATIONS,
                    "Too many deploy verifications in progress",
                )
            };

            // Try to acquire a slot.
            if counter
                .fetch_update(
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                    |val| {
                        if val < limit { Some(val + 1) } else { None }
                    },
                )
                .is_err()
            {
                return Err(RestError::too_many_requests(anyhow!("{err_msg}")));
            }
            // Perform the check.
            let res = rest
                .ledger
                .check_transaction_basic(&tx, None, &mut rand::thread_rng())
                .map_err(|err| RestError::unprocessable_entity(err.context("Invalid transaction")));

            // Release the slot.
            counter.fetch_sub(1, Ordering::Relaxed);
            // Propagate error if any.
            res?;
        }
        // Create a block with the transaction if the manual block creation feature is not enabled.
        if !rest.manual_block_creation {
            // Parse the private key.
            let private_key = PrivateKey::<N>::from_str(&rest.private_key)?;

            // Clone the ledger for the blocking task
            let ledger = rest.ledger.clone();
            // Wrap blocking operations in spawn_blocking
            let new_block = tokio::task::spawn_blocking(move || {
                ledger.prepare_advance_to_next_beacon_block(
                    &private_key,
                    vec![],
                    vec![],
                    vec![tx],
                    &mut rand::thread_rng(),
                )
            })
            .await
            .map_err(|e| RestError::internal_server_error(anyhow!("Task panicked: {}", e)))??;

            // Advance to the next block.
            tokio::task::spawn_blocking(move || rest.ledger.advance_to_next_block(&new_block))
                .await
                .map_err(|e| RestError::internal_server_error(anyhow!("Task panicked: {}", e)))??;
            return Ok((StatusCode::OK, ErasedJson::new(tx_id)));
        }

        // Add the transaction to the Rest buffer.
        {
            let mut buffer = rest.buffer.lock();
            buffer.push(tx);
        }

        Ok((StatusCode::OK, ErasedJson::new(tx_id)))
    }

    /// POST /{network}/create_block
    pub(crate) async fn create_block(
        State(rest): State<Self>,
        Json(req): Json<CreateBlockRequest>,
    ) -> Result<ErasedJson, RestError> {
        // Determine the number of blocks to create.
        let num_blocks = req.num_blocks.unwrap_or(1);

        // Iterate and create the specified number of blocks.
        // Return the last created block.
        let last_block = tokio::task::spawn_blocking(move || -> Result<ErasedJson, RestError> {
            let private_key = PrivateKey::<N>::from_str(&rest.private_key)
                .map_err(|e| RestError::bad_request(anyhow!("Invalid private key: {}", e)))?;

            let mut last_block = None;

            // Take all unconfirmed transactions from the buffer.
            let mut unconfirmed_txs = Some({
                let mut buffer = rest.buffer.lock();
                buffer.drain(..).collect()
            });

            for _ in 0..num_blocks {
                let txs = unconfirmed_txs.take().unwrap_or_default();

                // Prepare the new block.  Note that transactions in the buffer are added to the first block.
                // If there are no transactions left in the buffer, create an empty block.
                let new_block = rest
                    .ledger
                    .prepare_advance_to_next_beacon_block(&private_key, vec![], vec![], txs, &mut rand::thread_rng())
                    .map_err(|e| RestError::internal_server_error(anyhow!("Failed to prepare block: {}", e)))?;

                // Update the ledger to the new block.
                rest.ledger
                    .advance_to_next_block(&new_block)
                    .map_err(|e| RestError::internal_server_error(anyhow!("Failed to advance block: {}", e)))?;

                last_block = Some(new_block);
            }

            Ok(ErasedJson::new(last_block.unwrap()))
        })
        .await
        .map_err(|e| RestError::internal_server_error(anyhow!("Task panicked: {}", e)))??;

        Ok(last_block)
    }
}
