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

use super::*;
use crate::cli::query::{LeoBlock, LeoProgram};

use leo_ast::NetworkName;
use leo_package::ProgramData;
use leo_span::Symbol;
use snarkvm::prelude::{Program, ProgramID};

use indexmap::IndexSet;
use std::collections::HashMap;

/// A helper function to query the public balance of an address.
pub fn get_public_balance<N: Network>(
    private_key: &PrivateKey<N>,
    endpoint: &str,
    network: NetworkName,
    context: &Context,
) -> Result<u64> {
    // Derive the account address.
    let address = Address::<N>::try_from(ViewKey::try_from(private_key)?)?;
    // Query the public balance of the address on the `account` mapping from `credits.aleo`.
    let mut public_balance = LeoQuery {
        env_override: EnvOptions { endpoint: Some(endpoint.to_string()), network: Some(network), ..Default::default() },
        command: QueryCommands::Program {
            command: LeoProgram {
                name: "credits".to_string(),
                edition: None,
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
        env_override: EnvOptions { endpoint: Some(endpoint.to_string()), network: Some(network), ..Default::default() },
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
        .query("check_transaction", "true")
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
                // Print the fee as a transaction.
                println!("  - fee transaction ID: '{}'", Transaction::from_fee(fee)?.id().to_string().bold().yellow());
                println!("    (use this to check for rejected transactions)\n");
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
                Ok(response) => format!("(status code {code}: {response:?})"),
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

/// Loads the latest edition of a program and all its imports from the network, using an iterative DFS.
pub fn load_latest_programs_from_network<N: Network>(
    context: &Context,
    program_id: ProgramID<N>,
    network: NetworkName,
    endpoint: &str,
) -> Result<Vec<(Program<N>, Option<u16>)>> {
    use snarkvm::prelude::Program;
    use std::collections::HashSet;

    // A cache for loaded programs, mapping a program ID to its bytecode and edition.
    let mut programs = HashMap::new();
    // The ordered set of programs.
    let mut ordered_programs = IndexSet::new();
    // Stack of program IDs to process (DFS traversal).
    let mut stack = vec![(program_id, false)];

    // Loop until all programs and their dependencies are visited.
    while let Some((current_id, seen)) = stack.pop() {
        // If the program has already been seen, then all its imports have been processed.
        // Add it to the ordered set and continue.
        if seen {
            ordered_programs.insert(current_id);
        }
        // Otherwise, fetch it and schedule its imports for processing.
        else {
            // If the program is already in the cache, skip it.
            if programs.contains_key(&current_id) {
                continue;
            }
            // Fetch the program source from the network.
            let program = leo_package::Program::fetch(
                Symbol::intern(&current_id.name().to_string()),
                None,
                &context.home()?,
                network,
                endpoint,
                true,
            )
            .map_err(|_| CliError::custom(format!("Failed to fetch program source for ID: {current_id}")))?;
            let ProgramData::Bytecode(program_src) = program.data else {
                panic!("Expected bytecode when fetching a remote program");
            };

            // Parse the program source into a Program object.
            let bytecode = Program::<N>::from_str(&program_src)
                .map_err(|_| CliError::custom(format!("Failed to parse program source for ID: {current_id}")))?;

            // Get the imports of the program.
            let imports = bytecode.imports().keys().cloned().collect::<HashSet<_>>();

            // Add the program to the cache.
            programs.insert(current_id, (bytecode, program.edition));

            // Mark the program as seen.
            stack.push((current_id, true));

            // Queue all imported programs for processing.
            for import_id in imports {
                stack.push((import_id, false));
            }
        }
    }

    // Return all loaded programs in insertion order.
    Ok(ordered_programs
        .iter()
        .map(|program_id| programs.remove(program_id).expect("Program not found in cache"))
        .collect())
}
