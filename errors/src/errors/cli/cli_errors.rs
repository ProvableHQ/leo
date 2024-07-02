// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::create_messages;
use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_messages!(
    /// CliError enum that represents all the errors for the `leo-lang` crate.
    CliError,
    code_mask: 7000i32,
    code_prefix: "CLI",

    /// For when the CLI experiences an IO error.
    @backtraced
    cli_io_error {
        args: (error: impl ErrorArg),
        msg: format!("cli io error {error}"),
        help: None,
    }

    /// For when the CLI is given invalid user input.
    @backtraced
    cli_invalid_input {
        args: (error: impl Display),
        msg: format!("cli input error: {error}"),
        help: None,
    }

    /// For when the CLI fails to run something
    @backtraced
    cli_runtime_error {
        args: (error: impl Display),
        msg: format!("cli error: {error}"),
        help: None,
    }

    /// For when the CLI could not fetch the versions.
    @backtraced
    could_not_fetch_versions {
        args: (error: impl ErrorArg),
        msg: format!("Could not fetch versions: {error}"),
        help: None,
    }

    /// For when the CLI fails to enable ansi support.
    @backtraced
    failed_to_enable_ansi_support {
        args: (),
        msg: "failed to enable ansi_support",
        help: None,
    }

    /// For when the CLI fails to self update.
    @backtraced
    self_update_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate Error: {error}"),
        help: None,
    }

    /// For when the CLI fails to self update.
    @backtraced
    self_update_build_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate failed to build Error: {error}"),
        help: None,
    }

    /// For when the CLI has an old release version.
    @backtraced
    old_release_version {
        args: (current: impl Display, latest: impl Display),
        msg: format!("Old release version {current} {latest}"),
        help: None,
    }

    @backtraced
    failed_to_load_instructions {
        args: (error: impl Display),
        msg: format!("Failed to load compiled Aleo instructions into an Aleo file.\nError: {error}"),
        help: Some("Generated Aleo instructions have been left in `main.aleo`".to_string()),
    }

    @backtraced
    needs_leo_build {
        args: (),
        msg: "You must run leo build before deploying a program.".to_string(),
        help: None,
    }

    @backtraced
    failed_to_execute_build {
        args: (error: impl Display),
        msg: format!("Failed to execute the `build` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_new {
        args: (error: impl Display),
        msg: format!("Failed to execute the `new` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_run {
        args: (error: impl Display),
        msg: format!("Failed to execute the `run` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_node {
        args: (error: impl Display),
        msg: format!("Failed to execute the `node` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_deploy {
        args: (error: impl Display),
        msg: format!("Failed to execute the `deploy` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_new {
        args: (error: impl Display),
        msg: format!("Failed to parse the `new` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_run {
        args: (error: impl Display),
        msg: format!("Failed to parse the `run` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_node {
        args: (error: impl Display),
        msg: format!("Failed to parse the `node` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_deploy {
        args: (error: impl Display),
        msg: format!("Failed to parse the `deploy` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_execute {
        args: (error: impl Display),
        msg: format!("Failed to parse the `execute` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_execute {
        args: (error: impl Display),
        msg: format!("Failed to execute the `execute` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_seed {
        args: (error: impl Display),
        msg: format!("Failed to parse the seed string for account.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_write_file {
        args: (error: impl Display),
        msg: format!("Failed to write file.\nIO Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_private_key {
        args: (error: impl Display),
        msg: format!("Failed to parse private key.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_account {
        args: (error: impl Display),
        msg: format!("Failed to execute the `account` command.\nError: {error}"),
        help: None,
    }

    @backtraced
    failed_to_read_environment_private_key {
        args: (error: impl Display),
        msg: format!("Failed to read private key from environment.\nIO Error: {error}"),
        help: Some("Pass in private key using `--private-key <PRIVATE-KEY>` or create a .env file with your private key information. See examples for formatting information.".to_string()),
    }

    @backtraced
    recursive_deploy_with_record {
        args: (),
        msg: "Cannot combine recursive deploy with private fee.".to_string(),
        help: None,
    }

    @backtraced
    invalid_network_name {
        args: (network: impl Display),
        msg: format!("Invalid network name: {network}"),
        help: Some("Valid network names are `testnet` and `mainnet`.".to_string()),
    }

    @backtraced
    invalid_example {
        args: (example: impl Display),
        msg: format!("Invalid Leo example: {example}"),
        help: Some("Valid Leo examples are `lottery`, `tictactoe`, and `token`.".to_string()),
    }

    @backtraced
    build_error {
        args: (error: impl Display),
        msg: format!("Failed to build program: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_record {
        args: (error: impl Display),
        msg: format!("Failed to parse the record string.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    string_parse_error {
        args: (error: impl Display),
        msg: format!("{error}"),
        help: None,
    }

    @backtraced
    broadcast_error {
        args: (error: impl Display),
        msg: format!("{error}"),
        help: None,
    }

    @backtraced
    failed_to_get_endpoint_from_env {
        args: (),
        msg: "Failed to get an endpoint.".to_string(),
        help: Some("Either make sure you have a `.env` file in current project directory with an `ENDPOINT` variable set, or set the `--endpoint` flag when invoking the CLI command.\n Example: `ENDPOINT=https://api.explorer.aleo.org/v1` or `leo build --endpoint \"https://api.explorer.aleo.org/v1\"`.".to_string()),
    }

    @backtraced
    failed_to_get_private_key_from_env {
        args: (),
        msg: "Failed to get a private key.".to_string(),
        help: Some("Either make sure you have a `.env` file in current project directory with a `PRIVATE_KEY` variable set, or set the `--private-key` flag when invoking the CLI command.\n Example: `PRIVATE_KEY=0x1234...` or `leo deploy --private-key \"APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH\"`.".to_string()),
    }

    @backtraced
    failed_to_get_network_from_env {
        args: (),
        msg: "Failed to get a network.".to_string(),
        help: Some("Either make sure you have a `.env` file in current project directory with a `NETWORK` variable set, or set the `--network` flag when invoking the CLI command.\n Example: `NETWORK=testnet` or `leo build --network testnet`.".to_string()),
    }

    @backtraced
    constraint_limit_exceeded {
        args: (program: impl Display, limit: u64, network: impl Display),
        msg: format!("Program `{program}` exceeds the constraint limit {limit} for deployment on network {network}."),
        help: Some("Reduce the number of constraints in the program by reducing the number of instructions in transition functions.".to_string()),
    }

    @backtraced
    variable_limit_exceeded {
        args: (program: impl Display, limit: u64, network: impl Display),
        msg: format!("Program `{program}` exceeds the variable limit {limit} for deployment on network {network}."),
        help: Some("Reduce the number of variables in the program by reducing the number of instructions in transition functions.".to_string()),
    }

    @backtraced
    confirmation_failed {
        args: (),
        msg: "Failed to confirm transaction".to_string(),
        help: None,
    }

    @backtraced
    invalid_balance {
        args: (account: impl Display),
        msg: format!("Invalid public balance for account: {account}"),
        help: Some("Make sure the account has enough balance to pay for the deployment.".to_string()),
    }

    @backtraced
    table_render_failed {
        args: (error: impl Display),
        msg: format!("Failed to render table.\nError: {error}"),
        help: None,
    }
);
