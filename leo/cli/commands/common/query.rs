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
use snarkvm::prelude::{Program, ProgramID};

use super::*;

use leo_package::{NetworkName, ProgramData};
use leo_span::Symbol;

/// A helper function to query the public balance of an address.
pub fn get_public_balance<N: Network>(
    private_key: &PrivateKey<N>,
    endpoint: &str,
    network: &str,
    context: &Context,
) -> Result<u64> {
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
    public_balance.parse::<u64>().map_err(|_| CliError::invalid_balance(address).into())
}

// A helper function to query for the latest block height.
#[allow(dead_code)]
pub fn get_latest_block_height(endpoint: &str, network: NetworkName, context: &Context) -> Result<u32> {
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
///
/// Returns (body, status code).
pub fn handle_broadcast<N: Network>(
    endpoint: &str,
    transaction: &Transaction<N>,
    operation: &str,
) -> Result<(String, u16)> {
    // Send the deployment request to the endpoint.
    let mut response = ureq::Agent::config_builder()
        .max_redirects(0)
        .build()
        .new_agent()
        .post(endpoint)
        .header("X-Leo-Version", env!("CARGO_PKG_VERSION"))
        .send_json(transaction)
        .map_err(|err| CliError::broadcast_error(err.to_string()))?;
    match response.status().as_u16() {
        200..=299 => {
            println!(
                "✉️ Broadcasted transaction with:\n  - transaction ID: '{}'",
                transaction.id().to_string().bold().yellow(),
            );
            if let Some(fee) = transaction.fee_transition() {
                // Most transactions will have fees, but some, like credits.aleo/upgrade executions, may not.
                println!("  - fee ID: '{}'", fee.id().to_string().bold().yellow());
            }
            Ok((response.body_mut().read_to_string().unwrap(), response.status().as_u16()))
        }
        301 => {
            let msg = format!(
                "⚠️ The endpoint `{endpoint}` has been permanently moved. Try using `https://api.explorer.provable.com/v1` in your `.env` file or via the `--endpoint` flag."
            );
            Err(CliError::broadcast_error(msg).into())
        }
        _ => {
            let code = response.status();
            let error_message = match response.body_mut().read_to_string() {
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
    program_id: ProgramID<N>,
    network: NetworkName,
    endpoint: &str,
) -> Result<Vec<(ProgramID<N>, Program<N>)>> {
    use snarkvm::prelude::Program;
    use std::collections::HashSet;

    // Set of already loaded program IDs to prevent redundant fetches.
    let mut visited = HashSet::new();
    // Maintains the insertion order of loaded programs.
    let mut programs = IndexMap::new();
    // Stack of program IDs to process (DFS traversal).
    let mut stack = vec![program_id];

    // Loop until all programs and their dependencies are visited.
    while let Some(current_id) = stack.pop() {
        // Skip if we've already loaded this program.
        if !visited.insert(current_id) {
            continue;
        }

        // Fetch the program source from the network.
        let ProgramData::Bytecode(program_src) = leo_package::Program::fetch(
            Symbol::intern(&current_id.name().to_string()),
            &context.home()?,
            network,
            endpoint,
            true,
        )
        .map_err(|_| CliError::custom(format!("Failed to fetch program source for ID: {current_id}")))?
        .data
        else {
            panic!("Expected bytecode when fetching a remote program");
        };

        // Parse the program source into a Program object.
        let program = Program::<N>::from_str(&program_src)
            .map_err(|_| CliError::custom(format!("Failed to parse program source for ID: {current_id}")))?;

        // Queue all imported programs for future processing.
        for import_id in program.imports().keys() {
            stack.push(*import_id);
        }

        // Add the program to our ordered set.
        programs.insert(current_id, program);
    }

    // Return all loaded programs in insertion order.
    Ok(programs.into_iter().rev().collect())
}
