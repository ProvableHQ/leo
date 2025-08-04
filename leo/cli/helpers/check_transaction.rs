// Copyright (C) 2019-2025 Provable Inc.
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

use leo_ast::NetworkName;
use leo_errors::Result;

use anyhow::anyhow;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Deserialize)]
struct Transaction {
    id: String,
}

#[derive(Debug, Deserialize)]
struct TransactionResult {
    status: TransactionStatus,

    transaction: Transaction,
}

#[derive(Debug, Deserialize)]
struct Block {
    transactions: Vec<TransactionResult>,
    aborted_transaction_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Transition {
    id: String,
}

#[derive(Debug, Deserialize)]
struct Fee {
    transition: Transition,
}

#[derive(Debug, Deserialize)]
struct RejectedTransaction {
    fee: Option<Fee>,
}

pub fn current_height(endpoint: &str, network: NetworkName) -> Result<usize> {
    let height_url = format!("{endpoint}/{network}/block/height/latest");
    let height_str = leo_package::fetch_from_network_plain(&height_url)?;
    let height: usize = height_str.parse().map_err(|e| anyhow!("error parsing height: {e}"))?;
    Ok(height)
}

fn status_at_height(
    id: &str,
    maybe_fee_id: Option<&str>,
    endpoint: &str,
    network: NetworkName,
    height: usize,
    max_wait: usize,
) -> Result<Option<TransactionStatus>> {
    // Wait until the block at `height` exists.
    for i in 0usize.. {
        if current_height(endpoint, network)? >= height {
            break;
        } else if i >= max_wait {
            // We've waited too long; give up.
            return Ok(None);
        } else {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    let block_url = format!("{endpoint}/{network}/block/{height}");
    let block_str = leo_package::fetch_from_network_plain(&block_url)?;
    let block: Block = serde_json::from_str(&block_str).map_err(|e| anyhow!("Deserialization failure X: {e}."))?;
    let maybe_this_transaction =
        block.transactions.iter().find(|transaction_result| transaction_result.transaction.id == id);

    if let Some(transaction_result) = maybe_this_transaction {
        // We found it.
        return Ok(Some(transaction_result.status));
    }

    if block.aborted_transaction_ids.iter().any(|aborted_id| aborted_id == id) {
        // It was aborted.
        return Ok(Some(TransactionStatus::Aborted));
    }

    for rejected in &block.transactions {
        if rejected.status != TransactionStatus::Rejected {
            continue;
        }

        let url = format!("{endpoint}/{network}/transaction/unconfirmed/{}", rejected.transaction.id);
        let transaction_str = leo_package::fetch_from_network_plain(&url)?;
        let transaction: RejectedTransaction =
            serde_json::from_str(&transaction_str).map_err(|e| anyhow!("Deserialization failure: {e}"))?;
        // It's actually the fee that will show up as rejected.
        if transaction.fee.map(|fee| fee.transition.id).as_deref() == maybe_fee_id {
            // It was rejected.
            return Ok(Some(TransactionStatus::Rejected));
        }
    }

    Ok(None)
}

struct CheckedTransaction {
    blocks_checked: usize,
    status: Option<TransactionStatus>,
}

fn check_transaction(
    id: &str,
    maybe_fee_id: Option<&str>,
    endpoint: &str,
    network: NetworkName,
    start_height: usize,
    max_wait: usize,
    blocks_to_check: usize,
) -> Result<CheckedTransaction> {
    // It appears that the default rate limit for snarkOS is 10 requests per second per IP,
    // and this value seems to avoid rate limits in practice, so let's go with this.
    const DELAY_MILLIS: u64 = 201;

    for use_height in start_height..start_height + blocks_to_check {
        let status = status_at_height(id, maybe_fee_id, endpoint, network, use_height, max_wait)?;
        if status.is_some() {
            return Ok(CheckedTransaction { blocks_checked: use_height - start_height + 1, status });
        }

        // Avoid rate limits.
        std::thread::sleep(std::time::Duration::from_millis(DELAY_MILLIS));
    }

    Ok(CheckedTransaction { blocks_checked: blocks_to_check, status: None })
}

/// Check to find the transaction id among new blocks, printing its status (if found)
/// to the user. Returns `Some(..)` if and only if the transaction was found.
pub fn check_transaction_with_message(
    id: &str,
    maybe_fee_id: Option<&str>,
    endpoint: &str,
    network: NetworkName,
    start_height: usize,
    max_wait: usize,
    blocks_to_check: usize,
) -> Result<Option<TransactionStatus>> {
    println!("🔄 Searching up to {blocks_to_check} blocks to confirm transaction (this may take several seconds)...");
    let checked = crate::cli::check_transaction::check_transaction(
        id,
        maybe_fee_id,
        endpoint,
        network,
        start_height,
        max_wait,
        blocks_to_check,
    )?;
    println!("Explored {} blocks.", checked.blocks_checked);
    match checked.status {
        Some(TransactionStatus::Accepted) => println!("Transaction accepted."),
        Some(TransactionStatus::Rejected) => println!("Transaction rejected."),
        Some(TransactionStatus::Aborted) => println!("Transaction aborted."),
        None => println!("Could not find the transaction."),
    }
    Ok(checked.status)
}
