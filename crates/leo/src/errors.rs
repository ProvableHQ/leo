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
    Backtraced::error(CODE_PREFIX, CODE_MASK, format!("CLI I/O error: {error}"))
        .with_help("Verify file permissions and that the working directory is accessible.")
}

pub(crate) fn cli_invalid_input(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 1, format!("invalid CLI input: {error}"))
        .with_help("Run `leo <command> --help` to review the expected arguments.")
}

pub(crate) fn cli_runtime_error(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 2, format!("CLI runtime error: {error}"))
}

pub(crate) fn could_not_fetch_versions(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 3, format!("could not fetch Leo release versions: {error}"))
        .with_help("Check your internet connection and retry. If the problem persists, download a release manually from https://github.com/ProvableHQ/leo/releases.")
}

pub(crate) fn self_update_error(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 5, format!("self-update failed: {error}")).with_help(
        "Retry the update, or download the latest release manually from https://github.com/ProvableHQ/leo/releases.",
    )
}

pub(crate) fn self_update_build_error(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 6, format!("self-update failed to build the updater: {error}"))
        .with_help("Retry the update, or download the latest release manually from https://github.com/ProvableHQ/leo/releases.")
}

pub(crate) fn old_release_version(current: impl Display, latest: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 7,
        format!("Leo `{current}` is older than the latest release `{latest}`"),
    )
    .with_help("Run `leo update` to upgrade to the latest version.")
}

pub(crate) fn failed_to_load_instructions(error: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 8,
        format!("failed to load compiled Aleo instructions into an Aleo file: {error}"),
    )
    .with_help(
        "The generated Aleo instructions have been left in `main.aleo`. Inspect that file for the offending bytecode.",
    )
}

pub(crate) fn failed_to_serialize_abi(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 9, format!("failed to serialize ABI to JSON: {error}"))
        .with_help("This is an internal serialization failure. Re-run the build; if it persists, please file an issue.")
}

pub(crate) fn failed_to_write_abi(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 10, format!("failed to write ABI file: {error}"))
        .with_help("Verify the build output directory exists, is writable, and has enough free space.")
}

pub(crate) fn failed_to_parse_seed(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 23, format!("failed to parse the account seed: {error}"))
        .with_help("Provide a hex-encoded seed of the expected length, or omit the flag to use a fresh seed.")
}

pub(crate) fn failed_to_parse_private_key(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 25, format!("failed to parse private key: {error}"))
        .with_help("Private keys must be bech32-encoded and start with `APrivateKey1`.")
}

pub(crate) fn failed_to_execute_account(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 26, format!("failed to execute the `account` command: {error}"))
        .with_help("Run `leo account --help` to review the expected arguments.")
}

pub(crate) fn string_parse_error(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 33, format!("{error}"))
}

pub(crate) fn broadcast_error(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 34, format!("failed to broadcast transaction:\n{error}"))
        .with_help("Verify `--network` and `--endpoint` point to a reachable node, and that the transaction is valid for that network.")
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
        format!("program `{program}` has {actual} constraints, which exceeds the limit of {limit} for deployment on `{network}`"),
    )
    .with_help("Reduce the number of constraints by simplifying entry point functions. Fewer instructions, less branching, smaller loops.")
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
        format!("program `{program}` has {actual} variables, which exceeds the limit of {limit} for deployment on `{network}`"),
    )
    .with_help("Reduce the number of variables by simplifying entry point functions. Fewer intermediate bindings, less branching, smaller loops.")
}

pub(crate) fn invalid_balance(account: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 41, format!("invalid public balance for account `{account}`"))
        .with_help("Make sure the account has enough public balance to cover the deployment fee.")
}

pub(crate) fn invalid_package_name(kind: impl Display, name: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 43, format!("invalid {kind} name `{name}`"))
        .with_help(format!(
            "A {kind} name must be a valid Leo identifier: start with a letter, and use only letters, digits, and single underscores."
        ))
}

pub(crate) fn failed_to_parse_aleo_file(name: impl Display, error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 44, format!("failed to parse Aleo program `{name}`: {error}"))
        .with_help("Verify the file contains valid Aleo bytecode. If it was produced by Leo, rebuild from source.")
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
        format!("Leo generated invalid Aleo bytecode for program `{name}` — this is a compiler bug"),
    )
    .with_help(
        "Please report this issue at https://github.com/ProvableHQ/leo/issues, including the bytecode file and this full error message.",
    )
    .with_note(format!(
        "Leo version:  {}\nBytecode:     {path}\nChecksum:     [{checksum}]\n\nsnarkVM diagnostic:\n    {error}",
        env!("CARGO_PKG_VERSION")
    ))
}

// --- Duplicated from PAK (used by leo CLI) ---

pub(crate) fn failed_to_set_cwd(dir: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 48,
        format!("failed to set current working directory to `{dir}`: {error}"),
    )
    .with_help("Verify the directory exists and the current process has permission to enter it.")
}

pub(crate) fn io_error_env_file(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 49, format!("failed to read `.env` file: {error}"))
        .with_help("Verify `.env` exists in the package root and is readable.")
}

pub(crate) fn dependency_not_found(name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 50,
        format!("dependency program `{name}` was not found among the manifest's dependencies"),
    )
    .with_help(format!("Add `{name}` to `program.json` (e.g. with `leo add {name}`), then retry."))
}

pub(crate) fn insufficient_balance(address: impl Display, balance: impl Display, fee: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 51,
        format!("public balance of {balance} for `{address}` is insufficient to pay the base fee of {fee}"),
    )
    .with_help(format!("Fund `{address}` with at least {fee} of public credits before retrying."))
}

// --- Duplicated from UTL ---

pub(crate) fn failed_to_read_file(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 52, format!("failed to read file: {error}"))
        .with_help("Verify the file exists and that the current process has permission to read it.")
}

pub(crate) fn util_file_io_error(msg: impl Display, err: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 53, format!("filesystem I/O error: {msg}: {err}"))
        .with_help("Check the target path and the current process's permissions.")
}

pub(crate) fn failed_to_open_file(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 54, format!("failed to open file: {error}"))
        .with_help("Verify the file exists and that the current process has permission to read it.")
}

pub(crate) fn program_size_limit_exceeded(name: impl Display, size: usize, limit: usize) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 55,
        format!("program `{name}.aleo` is {size} bytes, exceeding the maximum allowed size of {limit} bytes"),
    )
    .with_help("Reduce the program size by removing unused code, simplifying functions, or splitting the program into smaller programs.")
}

pub(crate) fn invalid_input_id_len(input: impl Display, expected_type: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 56, format!("invalid input `{input}`"))
        .with_help(format!("`{expected_type}` values must contain exactly 61 lowercase letters or digits."))
}

pub(crate) fn invalid_input_id(
    input: impl Display,
    expected_type: impl Display,
    expected_preface: impl Display,
) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 57, format!("invalid input `{input}`"))
        .with_help(format!("`{expected_type}` values must start with `{expected_preface}`."))
}

pub(crate) fn invalid_numerical_input(input: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 58, format!("invalid numerical input `{input}`"))
        .with_help("Provide a valid `u32` value.")
}

pub(crate) fn invalid_height_or_hash(input: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 59, format!("invalid input `{input}`"))
        .with_help("Provide a valid block height (a `u32`) or hash. Valid hashes are 61 characters long, contain only lowercase letters and digits, and start with `ab1`.")
}

pub(crate) fn invalid_field(field: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 60, format!("invalid field `{field}`"))
        .with_help("Field elements must be numeric strings with an optional `field` suffix, e.g. `42field`.")
}

pub(crate) fn invalid_bound(bound: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 61, format!("invalid bound `{bound}`"))
        .with_help("Provide a valid `u32` value.")
}

pub(crate) fn invalid_range() -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 62, "block range must span at most 50 blocks")
        .with_help("Reduce the difference between `--start` and `--end` to 50 or fewer.")
}

/// For when --package names a member not listed in workspace.json.
pub(crate) fn workspace_package_not_found(name: impl Display, workspace_root: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 63,
        format!("no workspace member named `{name}` found in workspace at `{workspace_root}`"),
    )
    .with_help("Check the `members` list in `workspace.json` and verify the spelling of the package name.")
}

/// For when --package is used outside a workspace.
pub(crate) fn workspace_no_workspace() -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 64,
        "`--package` requires a workspace, but no `workspace.json` was found",
    )
    .with_help("Create a `workspace.json` in the project root, or run the command from within a Leo package.")
}

#[cfg(target_family = "windows")]
pub(crate) fn failed_to_enable_ansi_support() -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 65, "failed to enable ANSI support").with_help(
        "Update to a recent version of Windows 10/11, or run Leo from a terminal that supports ANSI escapes.",
    )
}

pub(crate) fn failed_to_load_process(reason: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 66, format!("failed to load snarkVM process: {reason}")).with_help(
        "This is an internal snarkVM error; the process should always load on a supported network. Please file an issue if this persists.",
    )
}

pub(crate) fn failed_to_read_import(path: impl Display, reason: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 67, format!("could not read import `{path}`: {reason}")).with_help(
        "Place the missing file at the path above, or pass `--imports-dir <DIR>` to point at the directory containing it.",
    )
}

pub(crate) fn circular_import(cycle: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 68, format!("circular import: {cycle}"))
        .with_help("Break the cycle by removing one of the import declarations.")
}

pub(crate) fn not_compatible(program: impl Display, abi: impl Display, unsatisfied: usize) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 69,
        format!("program `{program}` is not compatible with ABI `{abi}`: {unsatisfied} interface item(s) unsatisfied"),
    )
    .with_help("Review the unsatisfied items listed above. The program must declare every function, view, mapping, storage variable, record, and struct the ABI requires, with matching signatures.")
}

pub(crate) fn missing_constructor(program: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 70, format!("program `{program}` must declare a constructor"))
        .with_help("Add a constructor such as `@noupgrade constructor() {}` before deploying the program.")
}
