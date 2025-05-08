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

/// Compiler Options wrapper for Build command. Also used by other commands which
/// require Build command output as their input.
#[derive(Parser, Clone, Debug)]
pub struct BuildOptions {
    #[clap(long, help = "Endpoint to retrieve network state from. Overrides setting in `.env`.")]
    pub endpoint: Option<String>,
    #[clap(long, help = "Network to broadcast to. Overrides setting in `.env`.")]
    pub(crate) network: Option<String>,
    #[clap(long, help = "Does not recursively compile dependencies.")]
    pub non_recursive: bool,
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
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            endpoint: None,
            network: None,
            non_recursive: false,
            offline: false,
            enable_ast_spans: false,
            enable_dce: true,
            conditional_block_max_depth: 10,
            disable_conditional_branch_type_checking: false,
            enable_initial_ast_snapshot: false,
            enable_all_ast_snapshots: false,
            ast_snapshots: Vec::new(),
        }
    }
}

/// Overrides for the `.env` file.
#[derive(Parser, Clone, Debug)]
pub struct EnvOptions {
    #[clap(long, help = "The private key to use for the deployment. Overrides the `PRIVATE_KEY` in the `.env` file.")]
    pub(crate) private_key: Option<String>,
    #[clap(long, help = "The network to deploy to. Overrides the `NETWORK` in the .env file.")]
    pub(crate) network: Option<String>,
    #[clap(long, help = "The endpoint to deploy to. Overrides the `ENDPOINT` in the .env file.")]
    pub(crate) endpoint: Option<String>,
}

/// The fee options for the transactions.
#[derive(Parser, Clone, Debug, Default)]
pub struct FeeOptions {
    #[clap(
        short,
        long,
        help = "Don't ask for confirmation. DO NOT SET THIS FLAG UNLESS YOU KNOW WHAT YOU ARE DOING",
        default_value = "false"
    )]
    pub(crate) yes: bool,
    #[clap(
        long,
        help = "[UNUSED] Base fees in microcredits, delimited by `|`, and used in order. The fees must either be valid `u64` or `default`. Defaults to automatic calculation."
    )]
    pub(crate) base_fees: Option<String>,
    #[clap(
        long,
        help = "Priority fee in microcredits, delimited by `|`, and used in order. The fees must either be valid `u64` or `default`. Defaults to 0."
    )]
    pub(crate) priority_fees: Option<String>,
    #[clap(
        short,
        help = "Records to pay for fees privately, delimited by '|', and used in order. The fees must either be valid plaintext, ciphertext, or `default`. Defaults to public fees.",
        long
    )]
    fee_records: Option<String>,
    #[clap(long, help = "Consensus version to use for the transaction.")]
    pub(crate) consensus_version: Option<u8>,
}

/// Parses a list delimited by `|` into a vector of `Option<T>`.
pub fn parse_delimited_list_with_default<T>(
    list: Option<String>,
    parser: impl Fn(&str) -> Result<T>,
) -> Result<Vec<Option<T>>> {
    let mut parsed = Vec::new();
    for element in list.unwrap_or_default().split('|') {
        if element == "default" || element.is_empty() {
            parsed.push(None);
        } else {
            let element = parser(element)?;
            parsed.push(Some(element));
        }
    }
    Ok(parsed)
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

/// Parses a list of records delimited by `|` into a vector of `Option<Record<N, Plaintext<N>>>`.
#[allow(clippy::type_complexity)]
fn parse_fee_records<N: Network>(
    private_key: &PrivateKey<N>,
    list: Option<String>,
) -> Result<Vec<Option<Record<N, Plaintext<N>>>>> {
    let parser = |record: &str| -> Result<Record<N, Plaintext<N>>> { parse_record(private_key, record) };
    parse_delimited_list_with_default(list, parser)
}

/// Parses a list of amounts delimited by `|` into a vector of `Option<u64>`.
fn parse_amounts(list: Option<String>) -> Result<Vec<Option<u64>>> {
    let parser = |amount: &str| -> Result<u64> {
        amount.parse::<u64>().map_err(|_| CliError::custom("Failed to parse fee amount '{amount}' as u64").into())
    };
    parse_delimited_list_with_default(list, parser)
}

// A helper function to construct fee options for `k` transactions.
#[allow(clippy::type_complexity)]
pub fn parse_fee_options<N: Network>(
    private_key: &PrivateKey<N>,
    fee_options: &FeeOptions,
    k: usize,
) -> Result<Vec<(Option<u64>, Option<u64>, Option<Record<N, Plaintext<N>>>)>> {
    // Parse the base fees.
    let base_fees = parse_amounts(fee_options.base_fees.clone())?;
    // Parse the priority fees.
    let priority_fees = parse_amounts(fee_options.priority_fees.clone())?;
    // Parse the fee records.
    let fee_records = parse_fee_records(private_key, fee_options.fee_records.clone())?;

    // Pad the vectors to length `k`.
    let base_fees = base_fees.into_iter().chain(iter::repeat(None)).take(k);
    let priority_fees = priority_fees.into_iter().chain(iter::repeat(None)).take(k);
    let fee_records = fee_records.into_iter().chain(iter::repeat(None)).take(k);

    Ok(base_fees.zip(priority_fees).zip(fee_records).map(|((x, y), z)| (x, y, z)).collect())
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
