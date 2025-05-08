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

use crate::cli::query::{LeoBlock, LeoProgram};
use indexmap::IndexMap;
use snarkvm::prelude::Program;

use super::*;

use ureq::Response;

/// Utilities for querying the network.
// Note: Minimize the number of prints in these utilities to avoid formatting inconsistencies.
// TODO (@d0cd) Remove prints.
pub fn check_balance<N: Network>(
    private_key: &PrivateKey<N>,
    endpoint: &str,
    network: &str,
    context: &Context,
    total_cost: u64,
    skip_confirmation: bool,
) -> Result<()> {
    // Derive the account address.
    let address = Address::<N>::try_from(ViewKey::try_from(private_key)?)?;
    // Query the public balance of the address on the `account` mapping from `credits.aleo`.
    let mut public_balance = LeoQuery {
        endpoint: Some(endpoint.to_string()),
        network: Some(network.to_string()),
        command: QueryCommands::Program {
            command: LeoProgram {
                name: "credits".to_string(),
                mappings: false,
                mapping_value: Some(vec!["account".to_string(), address.to_string()]),
            },
        },
    }
    .execute(Context::new(context.path.clone(), context.home.clone(), true)?)?;
    // Remove the last 3 characters since they represent the `u64` suffix.
    public_balance.truncate(public_balance.len() - 3);
    // Make sure the balance is valid.
    let balance = if let Ok(credits) = public_balance.parse::<u64>() {
        credits
    } else {
        return Err(CliError::invalid_balance(address).into());
    };
    // Compare balance.
    if balance < total_cost {
        Err(PackageError::insufficient_balance(address, public_balance, total_cost).into())
    } else {
        println!("      Your current public balance is {} credits.\n", balance as f64 / 1_000_000.0);
        confirm("       This transaction will cost {} credits, do you want to continue?", skip_confirmation)?;
        Ok(())
    }
}

// A helper function to query for the latest block height.
pub fn get_latest_block_height(endpoint: &str, network: &str, context: &Context) -> Result<u32> {
    // Query the latest block height.
    let height = LeoQuery {
        endpoint: Some(endpoint.to_string()),
        network: Some(network.to_string()),
        command: QueryCommands::Block {
            command: LeoBlock {
                id: None,
                latest: false,
                latest_hash: false,
                latest_height: true,
                range: None,
                transactions: false,
                to_height: false,
            },
        },
    }
    .execute(Context::new(context.path.clone(), context.home.clone(), true)?)?;
    // Parse the height.
    let height = height.parse::<u32>().map_err(CliError::string_parse_error)?;
    Ok(height)
}

/// Determine if the transaction should be broadcast or displayed to user.
pub fn handle_broadcast<N: Network>(endpoint: &str, transaction: &Transaction<N>, operation: &str) -> Result<Response> {
    // Send the deployment request to the endpoint.
    let response = ureq::AgentBuilder::new()
        .redirects(0)
        .build()
        .post(endpoint)
        .set("X-Leo-Version", env!("CARGO_PKG_VERSION"))
        .send_json(transaction)
        .map_err(|err| CliError::broadcast_error(err.to_string()))?;
    match response.status() {
        200 => Ok(response),
        301 => {
            let msg = format!(
                "⚠️ The endpoint `{endpoint}` has been permanently moved. Try using `https://api.explorer.provable.com/v1` in your `.env` file or via the `--endpoint` flag."
            );
            Err(CliError::broadcast_error(msg).into())
        }
        _ => {
            let code = response.status();
            let error_message = match response.into_string() {
                Ok(response) => format!("(status code {code}: {:?})", response),
                Err(err) => format!("({err})"),
            };

            let msg = match transaction {
                Transaction::Deploy(..) => {
                    format!("❌ Failed to deploy '{}' to {}: {}", operation.bold(), &endpoint, error_message)
                }
                Transaction::Execute(..) => {
                    format!(
                        "❌ Failed to broadcast execution '{}' to {}: {}",
                        operation.bold(),
                        &endpoint,
                        error_message
                    )
                }
                Transaction::Fee(..) => {
                    format!("❌ Failed to broadcast fee '{}' to {}: {}", operation.bold(), &endpoint, error_message)
                }
            };

            Err(CliError::broadcast_error(msg).into())
        }
    }
}

/// Loads a program and all its imports from the network, using an iterative DFS.
pub fn load_programs_from_network<N: Network>(
    context: &Context,
    program_id: &str,
    network: &str,
    endpoint: &str,
) -> Result<Vec<(String, Program<N>)>> {
    use snarkvm::prelude::Program;
    use std::collections::HashSet;

    // Set of already loaded program IDs to prevent redundant fetches.
    let mut visited = HashSet::new();
    // Maintains the insertion order of loaded programs.
    let mut programs = IndexMap::new();
    // Stack of program IDs to process (DFS traversal).
    let mut stack = vec![program_id.to_string()];

    // Loop until all programs and their dependencies are visited.
    while let Some(current_id) = stack.pop() {
        // Skip if we've already loaded this program.
        if !visited.insert(current_id.clone()) {
            continue;
        }

        // Fetch the program source from the network.
        let program_src = LeoQuery {
            endpoint: Some(endpoint.to_string()),
            network: Some(network.to_string()),
            command: QueryCommands::Program {
                command: query::LeoProgram { name: current_id.clone(), mappings: false, mapping_value: None },
            },
        }
        .execute(Context::new(context.path.clone(), context.home.clone(), true)?)?;

        // Parse the program source into a Program object.
        let program = Program::<N>::from_str(&program_src)
            .map_err(|_| CliError::custom(format!("Failed to parse program source for ID: {current_id}")))?;

        // Queue all imported programs for future processing.
        for import_id in program.imports().keys() {
            stack.push(import_id.to_string());
        }

        // Add the program to our ordered set.
        programs.insert(current_id.clone(), program);
    }

    // Return all loaded programs in insertion order.
    Ok(programs.into_iter().collect())
}
