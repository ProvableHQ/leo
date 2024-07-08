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
        args: (name: impl Display),
        msg: format!("Failed to parse the source file for `{name}.aleo` into a valid Aleo program."),
        help: None,
    }

    @formatted
    circular_dependency_error {
        args: (),
        msg: "Circular dependency detected".to_string(),
        help: None,
    }

    @formatted
    network_error {
        args: (url: impl Display, status: impl Display),
        msg: format!("Failed network request to {url}. Status: {status}"),
        help: Some("Make sure that you are using the correct `--network` and `--endpoint` options.".to_string()),
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
        help: Some("Add a network specification to the dependency in the `program.json` file. Example: `network: \"testnet\"`".to_string()),
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
        msg: "Snarkvm error building program id".to_string(),
        help: None,
    }

    @formatted
    failed_to_retrieve_from_endpoint {
        args: (error: impl ErrorArg),
        msg: format!("{error}"),
        help: None,
    }

    @formatted
    build_file_does_not_exist {
        args: (path: impl Display),
        msg: format!("Compiled file at `{path}` does not exist, cannot compile parent."),
        help: Some("If you were using the `--non-recursive` flag, remove it and try again.".to_string()),
    }

    @backtraced
    invalid_input_id_len {
        args: (input: impl Display, expected_type: impl Display),
        msg: format!("Invalid input: {input}."),
        help: Some(format!("Type `{expected_type}` must contain exactly 61 lowercase characters or numbers.")),
    }

    @backtraced
    invalid_input_id {
        args: (input: impl Display, expected_type: impl Display, expected_preface: impl Display),
        msg: format!("Invalid input: {input}."),
        help: Some(format!("Type `{expected_type}` must start with \"{expected_preface}\".")),
    }

    @backtraced
    invalid_numerical_input {
        args: (input: impl Display),
        msg: format!("Invalid numerical input: {input}."),
        help: Some("Input must be a valid u32.".to_string()),
    }

    @backtraced
    invalid_range {
        args: (),
        msg: "The range must be less than or equal to 50 blocks.".to_string(),
        help: None,
    }

    @backtraced
    invalid_height_or_hash {
        args: (input: impl Display),
        msg: format!("Invalid input: {input}."),
        help: Some("Input must be a valid height or hash. Valid hashes are 61 characters long, composed of only numbers and lower case letters, and be prefaced with \"ab1\".".to_string()),
    }

    @backtraced
    invalid_field {
        args: (field: impl Display),
        msg: format!("Invalid field: {field}."),
        help: Some("Field element must be numerical string with optional \"field\" suffix.".to_string()),
    }

    @backtraced
    invalid_bound {
        args: (bound: impl Display),
        msg: format!("Invalid bound: {bound}."),
        help: Some("Bound must be a valid u32.".to_string()),
    }
);
