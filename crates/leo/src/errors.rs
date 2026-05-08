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

use std::{error::Error as ErrorArg, fmt::Display};

use leo_errors::Backtraced;

const CODE_PREFIX: &str = "CLI";
const CODE_MASK: i32 = 7000;

pub(crate) fn cli_io_error(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK, format!("cli io error {error}"))
}

pub(crate) fn cli_invalid_input(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 1, format!("cli input error: {error}"))
}

pub(crate) fn cli_runtime_error(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 2, format!("cli error: {error}"))
}

pub(crate) fn could_not_fetch_versions(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 3, format!("Could not fetch versions: {error}"))
}

pub(crate) fn self_update_error(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 5, format!("self update crate Error: {error}"))
}

pub(crate) fn self_update_build_error(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 6, format!("self update crate failed to build Error: {error}"))
}

pub(crate) fn old_release_version(current: impl Display, latest: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 7, format!("Old release version {current} {latest}"))
}

pub(crate) fn failed_to_load_instructions(error: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 8,
        format!("Failed to load compiled Aleo instructions into an Aleo file.\nError: {error}"),
    )
    .with_help("Generated Aleo instructions have been left in `main.aleo`")
}

pub(crate) fn failed_to_serialize_abi(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 9, format!("Failed to serialize ABI to JSON.\nError: {error}"))
}

pub(crate) fn failed_to_write_abi(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 10, format!("Failed to write ABI file.\nIO Error: {error}"))
}

pub(crate) fn failed_to_parse_seed(error: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 23,
        format!("Failed to parse the seed string for account.\nError: {error}"),
    )
}

pub(crate) fn failed_to_parse_private_key(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 25, format!("Failed to parse private key.\nError: {error}"))
}

pub(crate) fn failed_to_execute_account(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 26, format!("Failed to execute the `account` command.\nError: {error}"))
}

pub(crate) fn string_parse_error(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 33, format!("{error}"))
}

pub(crate) fn broadcast_error(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 34, format!("Failed to broadcast transaction:\n{error}"))
}

pub(crate) fn constraint_limit_exceeded(
    program: impl Display,
    actual: u64,
    limit: u64,
    network: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 38,
        format!("Program `{program}` has {actual} constraints, which exceeds the limit of {limit} for deployment on network {network}."),
    )
    .with_help("Reduce the number of constraints in the program by reducing the number of instructions in entry point functions.")
}

pub(crate) fn variable_limit_exceeded(
    program: impl Display,
    actual: u64,
    limit: u64,
    network: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 39,
        format!("Program `{program}` has {actual} variables, which exceeds the limit of {limit} for deployment on network {network}."),
    )
    .with_help("Reduce the number of variables in the program by reducing the number of instructions in entry point functions.")
}

pub(crate) fn invalid_balance(account: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 41, format!("Invalid public balance for account: {account}"))
        .with_help("Make sure the account has enough balance to pay for the deployment.")
}

pub(crate) fn invalid_package_name(kind: impl Display, name: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 43, format!("Invalid {kind} name `{name}`"))
}

pub(crate) fn failed_to_parse_aleo_file(name: impl Display, error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 44, format!("Failed to parse Aleo program '{name}'.\nError: {error}"))
        .with_help("Ensure the file contains valid Aleo bytecode.")
}

pub(crate) fn custom(msg: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 45, format!("{msg}"))
}

pub(crate) fn tests_failed(failed: impl Display, total: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 46, format!("{failed} out of {total} tests failed"))
}

pub(crate) fn generated_invalid_bytecode(
    name: impl Display,
    path: impl Display,
    checksum: impl Display,
    error: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 47,
        format!(
            "Leo generated invalid Aleo bytecode for program '{name}'. This is a compiler bug.\n\n  \
             Please report this issue at https://github.com/ProvableHQ/leo/issues\n\n  \
             Leo version:  {}\n  \
             Bytecode:     {path}\n  \
             Checksum:     [{checksum}]\n\n  \
             snarkVM diagnostic:\n    {error}",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .with_help("Include the bytecode file and this full error message in your bug report.")
}

// --- Duplicated from PAK (used by leo CLI) ---

pub(crate) fn failed_to_set_cwd(dir: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 48,
        format!("Failed to set current working directory to `{dir}`. Error: {error}."),
    )
}

pub(crate) fn io_error_env_file(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 49, format!("IO error env file from the provided file path - {error}"))
}

pub(crate) fn dependency_not_found(name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 50,
        format!("The dependency program `{name}` was not found among the manifest's dependencies."),
    )
}

pub(crate) fn insufficient_balance(address: impl Display, balance: impl Display, fee: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 51,
        format!("❌ Your public balance of {balance} for {address} is insufficient to pay the base fee of {fee}"),
    )
}

// --- Duplicated from UTL ---

pub(crate) fn failed_to_read_file(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 52, format!("Failed to read file {error}"))
}

pub(crate) fn util_file_io_error(msg: impl Display, err: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 53, format!("File system io error: {msg}. Error: {err}"))
}

pub(crate) fn failed_to_open_file(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 54, format!("Failed to open file {error}"))
}

pub(crate) fn program_size_limit_exceeded(name: impl Display, size: usize, limit: usize) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 55,
        format!("Program `{name}.aleo` exceeds the maximum size limit. Program size: {size} bytes; maximum allowed: {limit} bytes."),
    )
    .with_help("Reduce the program size by removing unnecessary code, optimizing functions, or splitting the program into smaller programs.")
}

pub(crate) fn invalid_input_id_len(input: impl Display, expected_type: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 56, format!("Invalid input: {input}."))
        .with_help(format!("Type `{expected_type}` must contain exactly 61 lowercase characters or numbers."))
}

pub(crate) fn invalid_input_id(
    input: impl Display,
    expected_type: impl Display,
    expected_preface: impl Display,
) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 57, format!("Invalid input: {input}."))
        .with_help(format!("Type `{expected_type}` must start with \"{expected_preface}\"."))
}

pub(crate) fn invalid_numerical_input(input: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 58, format!("Invalid numerical input: {input}."))
        .with_help("Input must be a valid u32.")
}

pub(crate) fn invalid_height_or_hash(input: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 59, format!("Invalid input: {input}."))
        .with_help("Input must be a valid height or hash. Valid hashes are 61 characters long, composed of only numbers and lower case letters, and be prefaced with \"ab1\".")
}

pub(crate) fn invalid_field(field: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 60, format!("Invalid field: {field}."))
        .with_help("Field element must be numerical string with optional \"field\" suffix.")
}

pub(crate) fn invalid_bound(bound: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 61, format!("Invalid bound: {bound}."))
        .with_help("Bound must be a valid u32.")
}

pub(crate) fn invalid_range() -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 62, "The range must be less than or equal to 50 blocks.")
}

/// For when --package names a member not listed in workspace.json.
pub(crate) fn workspace_package_not_found(name: impl Display, workspace_root: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 63,
        format!("No workspace member named '{name}' found in workspace at '{workspace_root}'."),
    )
    .with_help("Check the `members` list in `workspace.json`.".to_string())
}

/// For when --package is used outside a workspace.
pub(crate) fn workspace_no_workspace() -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 64,
        "The `--package` flag requires a workspace, but no `workspace.json` was found.",
    )
    .with_help("Create a `workspace.json` in the project root, or run the command from within a Leo package.")
}

#[cfg(target_family = "windows")]
pub(crate) fn failed_to_enable_ansi_support() -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 65, "failed_to enable ansi support")
}
