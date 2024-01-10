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
    /// InputError enum that represents all the errors for the `utils` crate.
    UtilError,
    code_mask: 10000i32,
    code_prefix: "UTL",

    @formatted
    util_file_io_error {
        args: (msg: impl Display, err: impl ErrorArg),
        msg: format!("File system io error: {msg}. Error: {err}"),
        help: None,
    }

    @formatted
    toml_serizalization_error {
        args: (error: impl ErrorArg),
        msg: format!("TOML serialization error: {error}"),
        help: None,
    }

    @formatted
    json_serialization_error {
        args: (error: impl ErrorArg),
        msg: format!("JSON serialization error: {error}"),
        help: None,
    }

    @formatted
    snarkvm_parsing_error {
        args: (),
        msg: format!("SnarkVM failure to parse `.aleo` program"),
        help: None,
    }

    @formatted
    circular_dependency_error {
        args: (),
        msg: format!("Circular dependency detected"),
        help: None,
    }

    @formatted
    network_error {
        args: (url: impl Display, status: impl Display),
        msg: format!("Failed network request to {url}. Status: {status}"),
        help: None,
    }

    @formatted
    duplicate_dependency_name_error {
        args: (dependency: impl Display),
        msg: format!("Duplicate dependency found: {dependency}"),
        help: None,
    }

    @backtraced
    reqwest_error {
        args: (error: impl Display),
        msg: format!("{}", error),
        help: None,
    }

    @backtraced
    failed_to_open_file {
        args: (error: impl Display),
        msg: format!("Failed to open file {error}"),
        help: None,
    }

    @backtraced
    failed_to_read_file {
        args: (error: impl Display),
        msg: format!("Failed to read file {error}"),
        help: None,
    }

    @backtraced
    failed_to_deserialize_file {
        args: (error: impl Display),
        msg: format!("Failed to deserialize file {error}"),
        help: None,
    }

    @formatted
    failed_to_retrieve_dependencies {
        args: (error: impl Display),
        msg: format!("Failed to retrieve dependencies. {error}"),
        help: None,
    }

    @formatted
    missing_network_error {
        args: (dependency: impl Display),
        msg: format!("Dependency {dependency} is missing a network specification"),
        help: Some("Add a network specification to the dependency in the `program.json` file. Example: `network: \"testnet3\"`".to_string()),
    }

    @formatted
    missing_path_error {
        args: (dependency: impl Display),
        msg: format!("Local dependency {dependency} is missing a path specification"),
        help: Some("Add a path in the `program.json` file to the dependency project root . Example: `path: \"../../board\"`".to_string()),
    }

    @formatted
    program_name_mismatch_error {
        args: (program_json_name: impl Display, dep_name: impl Display, path: impl Display),
        msg: format!("Name mismatch: Local program at path `{path}` is named `{program_json_name}` in `program.json` but `{dep_name}` in the program that imports it"),
        help: Some("Change one of the names to match the other".to_string()),
    }

    @formatted
    snarkvm_error_building_program_id {
        args: (),
        msg: format!("Snarkvm error building program id"),
        help: None,
    }

    @formatted
    failed_to_retrieve_from_endpoint {
        args: (endpoint: impl Display, error: impl ErrorArg),
        msg: format!("Failed to retrieve from endpoint `{endpoint}`. Error: {error}"),
        help: None,
    }
);
