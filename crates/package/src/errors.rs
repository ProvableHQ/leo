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
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 16,
        format!("IO error gitignore file from the provided file path - {error}"),
    )
}

pub(crate) fn failed_to_create_source_directory(path: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 17, format!("Failed creating source directory at `{path}`: {error}."))
}

pub(crate) fn failed_to_initialize_package(package: impl Display, path: impl Debug, error: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 21,
        format!("Failed to initialize package {package} at {path:?}. Error: {error}"),
    )
}

pub(crate) fn failed_to_write_manifest(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 31, format!("Failed to write manifest file: {error}"))
        .with_help("Create a package by running `leo new`.")
}

pub(crate) fn failed_to_deserialize_manifest_file(path: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 40,
        format!("Failed to deserialize `program.json` from the provided file path {path} - {error}"),
    )
}

pub(crate) fn failed_to_serialize_manifest_file(path: impl Display, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 41,
        format!("Failed to update `program.json` from the provided file path {path} - {error}"),
    )
}

pub(crate) fn failed_to_load_package(path: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 53, format!("Failed to load leo project at path {path}"))
        .with_help("Make sure that the path is correct and that the project exists.")
}

pub(crate) fn conflicting_dependency(existing: impl Display, new: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 54,
        format!("Conflicting dependency. The existing dependency is '{existing}', while the new one is '{new}'."),
    )
    .with_help("If your project has multiple dependencies on the same program, make sure that they are all network or all local dependencies, with the same editions.")
}

pub(crate) fn conflicting_manifest(expected_name: impl Display, manifest_name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 55,
        format!("Expected to find program {expected_name}, but manifest found for program {manifest_name}."),
    )
}

pub(crate) fn invalid_network_name(name: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 56, format!("Invalid network name {name} in manifest `program.json`."))
}

pub(crate) fn failed_path(path: impl Display, err: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 57, format!("Cannot find path `{path}`: {err}."))
}

pub(crate) fn invalid_entry_file(
    path: impl Display,
    main_filename: impl Display,
    lib_filename: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 58,
        format!("No valid entry file found in '{}'. Expected either '{}' or '{}'.", path, main_filename, lib_filename),
    )
    .with_help("Ensure that your source directory contains either a main program file or a library file.")
}

pub(crate) fn ambiguous_entry_file(
    path: impl Display,
    main_filename: impl Display,
    lib_filename: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 59,
        format!(
            "Source directory '{}' contains both '{}' and '{}'. A package must be either a program or a library, not both.",
            path, main_filename, lib_filename
        ),
    )
    .with_help("Remove either the program entry file or the library entry file.")
}

pub(crate) fn cli_invalid_package_name(kind: impl Display, name: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 60, format!("Invalid {kind} name `{name}`"))
}

pub(crate) fn snarkvm_parsing_error(name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 61,
        format!("Failed to parse the source file for `{name}.aleo` into a valid Aleo program."),
    )
}

pub(crate) fn util_file_io_error(msg: impl Display, err: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 62, format!("File system io error: {msg}. Error: {err}"))
}

pub(crate) fn failed_to_open_file(error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 63, format!("Failed to open file {error}"))
}

pub(crate) fn program_size_limit_exceeded(name: impl Display, size: usize, limit: usize) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 64,
        format!("Program `{name}.aleo` exceeds the maximum size limit. Program size: {size} bytes; maximum allowed: {limit} bytes."),
    )
    .with_help("Reduce the program size by removing unnecessary code, optimizing functions, or splitting the program into smaller programs.")
}

pub(crate) fn failed_to_retrieve_from_endpoint(url: impl Display, error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 65, format!("Failed to retrieve from endpoint `{url}`: {error}"))
}

pub(crate) fn endpoint_moved_error(endpoint: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 66, format!("The endpoint `{endpoint}` has been permanently moved."))
        .with_help("Try using `https://api.explorer.provable.com/v1` in your `.env` file or via the `--endpoint` flag.")
}

pub(crate) fn network_error(url: impl Display, status: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 67, format!("Failed network request to {url}. Status: {status}"))
        .with_help("Make sure that you are using the correct `--network` and `--endpoint` options.")
}

pub(crate) fn circular_dependency_error() -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 68, "Circular dependency detected")
}

/// For when a workspace member directory is missing or lacks a manifest.
pub(crate) fn workspace_member_not_found(member: impl Display, workspace_root: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 69,
        format!("Workspace member '{member}' not found. Expected a Leo package at '{workspace_root}/{member}'."),
    )
    .with_help("Ensure the member directory exists and contains a `program.json` manifest.")
}

/// For when workspace.json cannot be read or parsed.
pub(crate) fn workspace_manifest_error(path: impl Display, error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 72, format!("Failed to read workspace manifest at '{path}': {error}"))
}

/// A dependency uses `"location": "workspace"` but no enclosing workspace exists.
pub(crate) fn workspace_dep_outside_workspace(dep_name: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 73,
        format!("Dependency '{dep_name}' has location 'workspace' but no enclosing workspace was found."),
    )
    .with_help("Create a `workspace.json` in a parent directory, or change the dependency location to 'local' with an explicit path.")
}

/// A workspace dependency names a member that does not exist in the workspace.
pub(crate) fn workspace_dep_member_not_found(dep_name: impl Display, workspace_root: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 74,
        format!("Workspace dependency '{dep_name}' not found in workspace at '{workspace_root}'."),
    )
    .with_help("Check the `members` list in `workspace.json` and ensure the member's `program.json` has a matching program name.")
}
