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

use leo_errors::Backtraced;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

const CODE_PREFIX: &str = "PAK";
const CODE_MASK: i32 = 5000;

pub(crate) fn io_error_gitignore_file(error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 16, format!("failed to write `.gitignore`: {error}"))
        .with_help("Verify the package directory is writable.")
}

pub(crate) fn failed_to_create_source_directory(path: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 17, format!("failed to create source directory at `{path}`: {error}"))
        .with_help("Verify the parent directory exists and is writable.")
}

pub(crate) fn failed_to_initialize_package(package: impl Display, path: impl Debug, error: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 21,
        format!("failed to initialize package `{package}` at `{path:?}`: {error}"),
    )
    .with_help("Verify the target directory is empty (or does not exist) and is writable.")
}

pub(crate) fn failed_to_write_manifest(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 31, format!("failed to write manifest file: {error}"))
        .with_help("Run `leo new` to scaffold a new package, or verify the package directory is writable.")
}

pub(crate) fn failed_to_deserialize_manifest_file(path: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 40, format!("failed to deserialize `program.json` at `{path}`: {error}"))
        .with_help(
            "Open `program.json` and fix the JSON syntax. Run `leo new` for a working example of the expected schema.",
        )
}

pub(crate) fn failed_to_serialize_manifest_file(path: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 41, format!("failed to update `program.json` at `{path}`: {error}"))
        .with_help("Verify the file is writable and that no other process holds it open.")
}

pub(crate) fn failed_to_load_package(path: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 53, format!("failed to load Leo project at `{path}`"))
        .with_help("Verify the path points to a directory containing a `program.json` manifest.")
}

pub(crate) fn conflicting_dependency(existing: impl Display, new: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 54,
        format!("conflicting dependency: existing is `{existing}`, new is `{new}`"),
    )
    .with_help("Multiple dependencies on the same program must all be network or all local, with the same edition. Align the entries in `program.json`.")
}

pub(crate) fn conflicting_manifest(expected_name: impl Display, manifest_name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 55,
        format!("expected program `{expected_name}`, but the manifest declares program `{manifest_name}`"),
    )
    .with_help(format!(
        "Rename the program in `program.json` to `{expected_name}`, or update the importer to use `{manifest_name}`."
    ))
}

pub(crate) fn invalid_network_name(name: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 56, format!("invalid network name `{name}` in `program.json`"))
        .with_help("Valid network names are `testnet`, `mainnet`, and `canary`.")
}

pub(crate) fn failed_path(path: impl Display, err: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 57, format!("cannot find path `{path}`: {err}"))
        .with_help("Verify the path exists and is accessible from the current working directory.")
}

pub(crate) fn invalid_entry_file(
    path: impl Display,
    main_filename: impl Display,
    lib_filename: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 58,
        format!("no entry file found in `{path}` (expected `{main_filename}` or `{lib_filename}`)"),
    )
    .with_help(format!(
        "Add either `{main_filename}` (program entry) or `{lib_filename}` (library entry) to the source directory."
    ))
}

pub(crate) fn ambiguous_entry_file(
    path: impl Display,
    main_filename: impl Display,
    lib_filename: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 59,
        format!("source directory `{path}` contains both `{main_filename}` and `{lib_filename}`"),
    )
    .with_help("A package must be either a program or a library, not both. Remove one of the entry files.")
}

pub(crate) fn cli_invalid_package_name(kind: impl Display, name: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 60, format!("invalid {kind} name `{name}`"))
        .with_help(format!(
            "A {kind} name must be a valid Leo identifier: start with a letter, and use only letters, digits, and single underscores."
        ))
}

pub(crate) fn snarkvm_parsing_error(name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 61,
        format!("failed to parse the source file for `{name}.aleo` into a valid Aleo program"),
    )
    .with_help(format!("Verify that `{name}.aleo` is valid Aleo bytecode. If it was produced by Leo, rebuild the dependency from source."))
}

pub(crate) fn util_file_io_error(msg: impl Display, err: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 62, format!("filesystem I/O error: {msg}: {err}"))
        .with_help("Check the target path and the current process's permissions.")
}

pub(crate) fn failed_to_open_file(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 63, format!("failed to open file: {error}"))
        .with_help("Verify the file exists and that the current process has permission to read it.")
}

pub(crate) fn program_size_limit_exceeded(name: impl Display, size: usize, limit: usize) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 64,
        format!("program `{name}.aleo` is {size} bytes, exceeding the maximum allowed size of {limit} bytes"),
    )
    .with_help("Reduce the program size by removing unused code, simplifying functions, or splitting the program into smaller programs.")
}

pub(crate) fn failed_to_retrieve_from_endpoint(url: impl Display, error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 65, format!("failed to retrieve from endpoint `{url}`: {error}"))
        .with_help("Verify the endpoint is reachable, check `--network`/`--endpoint`, and ensure the resource exists on that network.")
}

pub(crate) fn endpoint_moved_error(endpoint: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 66, format!("the endpoint `{endpoint}` has been permanently moved"))
        .with_help("Use `https://api.explorer.provable.com/v1` in your `.env` file or via the `--endpoint` flag.")
}

pub(crate) fn network_error(url: impl Display, status: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 67, format!("network request to `{url}` failed with status `{status}`"))
        .with_help("Verify that `--network` and `--endpoint` point to a running node and that you have connectivity.")
}

pub(crate) fn circular_dependency_error() -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 68, "circular dependency detected")
        .with_help("Break the cycle by removing one of the dependency edges in `program.json`. Programs cannot depend on themselves transitively.")
}

/// For when a workspace member directory is missing or lacks a manifest.
pub(crate) fn workspace_member_not_found(member: impl Display, workspace_root: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 69,
        format!("workspace member `{member}` not found (expected a Leo package at `{workspace_root}/{member}`)"),
    )
    .with_help("Ensure the member directory exists and contains a `program.json` manifest.")
}

/// For when workspace.json cannot be read or parsed.
pub(crate) fn workspace_manifest_error(path: impl Display, error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 72, format!("failed to read workspace manifest at `{path}`: {error}"))
        .with_help("Verify `workspace.json` exists and contains valid JSON.")
}

/// A dependency uses `"location": "workspace"` but no enclosing workspace exists.
pub(crate) fn workspace_dep_outside_workspace(dep_name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 73,
        format!("dependency `{dep_name}` has location `workspace` but no enclosing workspace was found"),
    )
    .with_help("Create a `workspace.json` in a parent directory, or change the dependency location to `local` with an explicit path.")
}

/// A workspace dependency names a member that does not exist in the workspace.
pub(crate) fn workspace_dep_member_not_found(dep_name: impl Display, workspace_root: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 74,
        format!("workspace dependency `{dep_name}` not found in workspace at `{workspace_root}`"),
    )
    .with_help("Check the `members` list in `workspace.json` and verify the member's `program.json` declares a matching program name.")
}
