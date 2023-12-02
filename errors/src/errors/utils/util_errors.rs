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
        args: (error: impl ErrorArg),
        msg: format!("File system io error: {error}"),
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
);
