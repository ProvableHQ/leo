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
use anyhow::{bail, ensure};
use itertools::Itertools;
use leo_ast::NetworkName;
use leo_package::fetch_from_network;
use snarkvm::prelude::{
    CANARY_V0_CONSENSUS_VERSION_HEIGHTS,
    ConsensusVersion,
    MAINNET_V0_CONSENSUS_VERSION_HEIGHTS,
    TEST_CONSENSUS_VERSION_HEIGHTS,
    TESTNET_V0_CONSENSUS_VERSION_HEIGHTS,
};

/// Compiler Options wrapper for Build command. Also used by other commands which
/// require Build command output as their input.
#[derive(Parser, Clone, Debug)]
pub struct BuildOptions {
    #[clap(long, help = "Enables offline mode.")]
    pub offline: bool,
    #[clap(long, help = "Enable spans in AST snapshots.")]
    pub enable_ast_spans: bool,
    #[clap(long, help = "Enables dead code elimination in the compiler.", default_value = "true")]
    pub enable_dce: bool,
    #[clap(long, help = "Max depth to type check nested conditionals.", default_value = "10")]
    pub conditional_block_max_depth: usize,
    #[clap(long, help = "Disable type checking of nested conditional branches in finalize scope.")]
    pub disable_conditional_branch_type_checking: bool,
    #[clap(long, help = "Write an AST snapshot immediately after parsing.")]
    pub enable_initial_ast_snapshot: bool,
    #[clap(long, help = "Writes all AST snapshots for the different compiler phases.")]
    pub enable_all_ast_snapshots: bool,
    #[clap(long, help = "Comma separated list of passes whose AST snapshots to capture.", value_delimiter = ',', num_args = 1..)]
    pub ast_snapshots: Vec<String>,
    #[clap(long, help = "Build tests along with the main program and dependencies.")]
    pub build_tests: bool,
    #[clap(long, help = "Don't use the dependency cache.")]
    pub no_cache: bool,
    #[clap(long, help = "Don't use the local source code.")]
    pub no_local: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            offline: false,
            enable_ast_spans: false,
            enable_dce: true,
            conditional_block_max_depth: 10,
            disable_conditional_branch_type_checking: false,
            enable_initial_ast_snapshot: false,
            enable_all_ast_snapshots: false,
            ast_snapshots: Vec::new(),
            build_tests: false,
            no_cache: false,
            no_local: false,
        }
    }
}

/// Overrides for the `.env` file.
#[derive(Parser, Clone, Debug, Default)]
pub struct EnvOptions {
    #[clap(
        long,
        help = "The private key to use for the deployment. Overrides the `PRIVATE_KEY` environment variable."
    )]
    pub(crate) private_key: Option<String>,
    #[clap(long, help = "The network to deploy to. Overrides the `NETWORK` environment variable.")]
    pub(crate) network: Option<String>,
    #[clap(long, help = "The endpoint to deploy to. Overrides the `ENDPOINT` environment variable.")]
    pub(crate) endpoint: Option<String>,
    #[clap(long, help = "Whether the network is a devnet. If not set, defaults to the `DEVNET` environment variable.")]
    pub(crate) devnet: bool,
    #[clap(
        long,
        help = "Optional consensus heights to use. This should only be set if you are using a custom devnet.",
        value_delimiter = ','
    )]
    pub(crate) consensus_heights: Option<Vec<u32>>,
}

/// The fee options for the transactions.
#[derive(Parser, Clone, Debug, Default)]
pub struct FeeOptions {
    #[clap(
        long,
        help = "[UNUSED] Base fees in microcredits, delimited by `|`, and used in order. The fees must either be valid `u64` or `default`. Defaults to automatic calculation.",
        hide = true,
        value_delimiter = '|',
        value_parser = parse_amount
    )]
    pub(crate) base_fees: Vec<Option<u64>>,
    #[clap(
        long,
        help = "Priority fee in microcredits, delimited by `|`, and used in order. The fees must either be valid `u64` or `default`. Defaults to 0.",
        value_delimiter = '|',
        value_parser = parse_amount
    )]
    pub(crate) priority_fees: Vec<Option<u64>>,
    #[clap(
        short,
        help = "Records to pay for fees privately, delimited by '|', and used in order. The fees must either be valid plaintext, ciphertext, or `default`. Defaults to public fees.",
        long,
        value_delimiter = '|',
        value_parser = parse_record_string,
    )]
    fee_records: Vec<Option<String>>,
}

// A helper function to parse amounts, which can either be a `u64` or `default`.
fn parse_amount(s: &str) -> Result<Option<u64>, String> {
    let trimmed = s.trim();
    if trimmed == "default" { Ok(None) } else { trimmed.parse::<u64>().map_err(|e| e.to_string()).map(Some) }
}

// A helper function to parse record strings, which can either be a string or `default`.
fn parse_record_string(s: &str) -> Result<Option<String>, String> {
    let trimmed = s.trim();
    if trimmed == "default" { Ok(None) } else { Ok(Some(trimmed.to_string())) }
}

/// Parses the record string. If the string is a ciphertext, then attempt to decrypt it. Lifted from snarkOS.
fn parse_record<N: Network>(private_key: &PrivateKey<N>, record: &str) -> Result<Record<N, Plaintext<N>>> {
    match record.starts_with("record1") {
        true => {
            // Parse the ciphertext.
            let ciphertext = Record::<N, Ciphertext<N>>::from_str(record)?;
            // Derive the view key.
            let view_key = ViewKey::try_from(private_key)?;
            // Decrypt the ciphertext.
            Ok(ciphertext.decrypt(&view_key)?)
        }
        false => Ok(Record::<N, Plaintext<N>>::from_str(record)?),
    }
}

// A helper function to construct fee options for `k` transactions.
#[allow(clippy::type_complexity)]
pub fn parse_fee_options<N: Network>(
    private_key: &PrivateKey<N>,
    fee_options: &FeeOptions,
    k: usize,
) -> Result<Vec<(Option<u64>, Option<u64>, Option<Record<N, Plaintext<N>>>)>> {
    // Parse the base fees.
    let base_fees = fee_options.base_fees.clone();
    // Parse the priority fees.
    let priority_fees = fee_options.priority_fees.clone();
    // Parse the fee records.
    let parse_record = |record: &Option<String>| record.as_ref().map(|r| parse_record::<N>(private_key, r)).transpose();
    let fee_records = fee_options.fee_records.iter().map(parse_record).collect::<Result<Vec<_>>>()?;

    // Pad the vectors to length `k`.
    let base_fees = base_fees.into_iter().chain(iter::repeat(None)).take(k);
    let priority_fees = priority_fees.into_iter().chain(iter::repeat(None)).take(k);
    let fee_records = fee_records.into_iter().chain(iter::repeat(None)).take(k);

    Ok(base_fees.zip(priority_fees).zip(fee_records).map(|((x, y), z)| (x, y, z)).collect())
}

/// Additional options that are common across a number of commands.
#[derive(Parser, Clone, Debug, Default)]
pub struct ExtraOptions {
    #[clap(
        short,
        long,
        help = "Don't ask for confirmation. DO NOT SET THIS FLAG UNLESS YOU KNOW WHAT YOU ARE DOING",
        default_value = "false"
    )]
    pub(crate) yes: bool,
    #[clap(
        long,
        help = "Consensus version to use. If one is not provided, the CLI will attempt to determine it from the latest block."
    )]
    pub(crate) consensus_version: Option<u8>,
    #[clap(
        long,
        help = "Seconds to wait for a block to appear when searching for a transaction.",
        default_value = "8"
    )]
    pub(crate) max_wait: usize,
    #[clap(long, help = "Number of blocks to look at when searching for a transaction.", default_value = "12")]
    pub(crate) blocks_to_check: usize,
}

// A helper function to get the consensus version from the fee options.
// If a consensus version is not provided, then attempt to query the current block height and use it to determine the version.
pub fn get_consensus_version(
    consensus_version: &Option<u8>,
    endpoint: &str,
    network: NetworkName,
    heights: &[u32],
    context: &Context,
) -> Result<ConsensusVersion> {
    // Get the consensus version.
    let result = match consensus_version {
        Some(1) => Ok(ConsensusVersion::V1),
        Some(2) => Ok(ConsensusVersion::V2),
        Some(3) => Ok(ConsensusVersion::V3),
        Some(4) => Ok(ConsensusVersion::V4),
        Some(5) => Ok(ConsensusVersion::V5),
        Some(6) => Ok(ConsensusVersion::V6),
        Some(7) => Ok(ConsensusVersion::V7),
        Some(8) => Ok(ConsensusVersion::V8),
        Some(9) => Ok(ConsensusVersion::V9),
        Some(10) => Ok(ConsensusVersion::V10),
        // If none is provided, then attempt to query the current block height and use it to determine the version.
        None => {
            println!("Attempting to determine the consensus version from the latest block height at {endpoint}...");
            // Get the consensus heights for the current network.
            get_latest_block_height(endpoint, network, context)
                .and_then(|current_block_height| get_consensus_version_from_height(current_block_height, heights))
                .map_err(|_| {
                    CliError::custom(
                        "Failed to get consensus version. Ensure that your endpoint is valid or provide an explicit version to use via `--consensus-version`",
                    )
                        .into()
                })
        }
        Some(version) => Err(CliError::custom(format!("Invalid consensus version: {version}")).into()),
    };

    // Check `{endpoint}/{network}/consensus_version` endpoint for the consensus version.
    // If it returns a result and does not match the given version, print a warning.
    if let Ok(consensus_version) = result {
        if let Err(e) = check_consensus_version_mismatch(consensus_version, endpoint, network) {
            println!("⚠️ Warning: {e}");
        }
    }

    result
}

/// A helper function to check for a consensus version mismatch against the network.
pub fn check_consensus_version_mismatch(
    consensus_version: ConsensusVersion,
    endpoint: &str,
    network: NetworkName,
) -> anyhow::Result<()> {
    // Check the `{endpoint}/{network}/consensus_version` endpoint for the consensus version.
    if let Ok(response) = fetch_from_network(&format!("{endpoint}/{network}/consensus_version")) {
        if let Ok(response) = response.parse::<u8>() {
            let consensus_version = consensus_version as u8;
            if response != consensus_version {
                bail!("Expected consensus version {consensus_version} but found {response} at {endpoint}",);
            }
        }
    }
    Ok(())
}

// A helper function to get the consensus version based on the block height.
// Note. This custom implementation is necessary because we use `snarkVM` with the `test_heights` feature enabled, which does not reflect the actual consensus version heights.
pub fn get_consensus_version_from_height(seek_height: u32, heights: &[u32]) -> Result<ConsensusVersion> {
    // Find the consensus version based on the block height.
    let index = match heights.binary_search_by(|height| height.cmp(&seek_height)) {
        // If a consensus version was found at this height, return it.
        Ok(index) => index,
        // If the specified height was not found, determine whether to return an appropriate version.
        Err(index) => {
            if index == 0 {
                return Err(CliError::custom("Expected consensus version 1 to exist at height 0.").into());
            } else {
                // Return the appropriate version belonging to the height *lower* than the sought height.
                index - 1
            }
        }
    };
    // Convert the index to a consensus version.
    Ok(number_to_consensus_version(index + 1))
}

// A helper to convert an index to a consensus version.
pub fn number_to_consensus_version(index: usize) -> ConsensusVersion {
    match index {
        1 => ConsensusVersion::V1,
        2 => ConsensusVersion::V2,
        3 => ConsensusVersion::V3,
        4 => ConsensusVersion::V4,
        5 => ConsensusVersion::V5,
        6 => ConsensusVersion::V6,
        7 => ConsensusVersion::V7,
        8 => ConsensusVersion::V8,
        9 => ConsensusVersion::V9,
        10 => ConsensusVersion::V10,
        _ => panic!("Invalid consensus version: {index}"),
    }
}

/// Get the consensus heights for the current network.
/// If `is_devnet` is true, first , then return the test consensus heights.
pub fn get_consensus_heights(network_name: NetworkName, is_devnet: bool) -> Vec<u32> {
    if is_devnet {
        TEST_CONSENSUS_VERSION_HEIGHTS.into_iter().map(|(_, v)| v).collect_vec()
    } else {
        match network_name {
            NetworkName::CanaryV0 => CANARY_V0_CONSENSUS_VERSION_HEIGHTS,
            NetworkName::MainnetV0 => MAINNET_V0_CONSENSUS_VERSION_HEIGHTS,
            NetworkName::TestnetV0 => TESTNET_V0_CONSENSUS_VERSION_HEIGHTS,
        }
        .into_iter()
        .map(|(_, v)| v)
        .collect_vec()
    }
}

/// Validates a vector of heights as consensus heights.
pub fn validate_consensus_heights(heights: &[u32]) -> anyhow::Result<()> {
    // Assert that the genesis height is 0.
    ensure!(heights[0] == 0, "Genesis height must be 0.");
    // Assert that the consensus heights are strictly increasing.
    for window in heights.windows(2) {
        if window[0] >= window[1] {
            bail!("Heights must be strictly increasing, but found: {window:?}");
        }
    }
    Ok(())
}

/// What to do with a transaction produced by the CLI.
#[derive(Args, Clone, Debug)]
pub struct TransactionAction {
    #[arg(long, help = "Print the transaction to stdout.")]
    pub print: bool,
    #[arg(long, help = "Broadcast the transaction to the network.")]
    pub broadcast: bool,
    #[arg(long, help = "Save the transaction to the provided directory.")]
    pub save: Option<String>,
}

#[cfg(test)]
mod test {
    use snarkvm::prelude::ConsensusVersion;

    #[test]
    fn test_latest_consensus_version() {
        assert_eq!(ConsensusVersion::latest(), ConsensusVersion::V10); // If this fails, update the test and any code that matches on `ConsensusVersion`.
    }
}
