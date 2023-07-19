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
    /// PackageError enum that represents all the errors for the `leo-package` crate.
    PackageError,
    code_mask: 5000i32,
    code_prefix: "PAK",

    /// For when getting a input file entry failed.
    @backtraced
    failed_to_get_input_file_entry {
        args: (error: impl ErrorArg),
        msg: format!("failed to get input file entry: {error}"),
        help: None,
    }

    /// For when getting the input file type failed.
    @backtraced
    failed_to_get_input_file_type {
        args: (file: impl Debug, error: impl ErrorArg),
        msg: format!("failed to get input file `{file:?}` type: {error}"),
        help: None,
    }

    /// For when getting the input file has an invalid file type.
    @backtraced
    invalid_input_file_type {
        args: (file: impl Debug, type_: std::fs::FileType),
        msg: format!("input file `{file:?}` has invalid type: {type_:?}"),
        help: None,
    }

    /// For when creating the inputs directory failed.
    @backtraced
    failed_to_create_inputs_directory {
        args: (error: impl ErrorArg),
        msg: format!("failed creating inputs directory {error}"),
        help: None,
    }

    /// For when reading the struct file failed.
    @backtraced
    failed_to_read_circuit_file {
        args: (path: impl Debug),
        msg: format!("Cannot read struct file from the provided file path - {path:?}"),
        help: None,
    }

    /// For when reading the input directory failed.
    @backtraced
    failed_to_read_inputs_directory {
        args: (error: impl ErrorArg),
        msg: format!("failed reading inputs directory {error}"),
        help: None,
    }

    /// For when reading the input file failed.
    @backtraced
    failed_to_read_input_file {
        args: (path: impl Debug),
        msg: format!("Cannot read input file from the provided file path - {path:?}"),
        help: None,
    }

    /// For when reading the snapshot file failed.
    @backtraced
    failed_to_read_snapshot_file {
        args: (path: impl Debug),
        msg: format!("Cannot read snapshot file from the provided file path - {path:?}"),
        help: None,
    }

    /// For when reading the checksum file failed.
    @backtraced
    failed_to_read_checksum_file {
        args: (path: impl Debug),
        msg: format!("Cannot read checksum file from the provided file path - {path:?}"),
        help: None,
    }

    /// For when the struct file has an IO error.
    @backtraced
    io_error_circuit_file {
        args: (error: impl ErrorArg),
        msg: format!("IO error struct file from the provided file path - {error}"),
        help: None,
    }

    /// For when the checksum file has an IO error.
    @backtraced
    io_error_checksum_file {
        args: (error: impl ErrorArg),
        msg: format!("IO error checksum file from the provided file path - {error}"),
        help: None,
    }

    /// For when the main file has an IO error.
    @backtraced
    io_error_main_file {
        args: (error: impl ErrorArg),
        msg: format!("IO error main file from the provided file path - {error}"),
        help: None,
    }

    /// For when removing the struct file failed.
    @backtraced
    failed_to_remove_circuit_file {
        args: (path: impl Debug),
        msg: format!("failed removing struct file from the provided file path - {path:?}"),
        help: None,
    }

    /// For when removing the checksum file failed.
    @backtraced
    failed_to_remove_checksum_file {
        args: (path: impl Debug),
        msg: format!("failed removing checksum file from the provided file path - {path:?}"),
        help: None,
    }

    /// For when removing the snapshot file failed.
    @backtraced
    failed_to_remove_snapshot_file {
        args: (path: impl Debug),
        msg: format!("failed removing snapshot file from the provided file path - {path:?}"),
        help: None,
    }

    /// For when the input file has an IO error.
    @backtraced
    io_error_input_file {
        args: (error: impl ErrorArg),
        msg: format!("IO error input file from the provided file path - {error}"),
        help: None,
    }

    /// For when the gitignore file has an IO error.
    @backtraced
    io_error_gitignore_file {
        args: (error: impl ErrorArg),
        msg: format!("IO error gitignore file from the provided file path - {error}"),
        help: None,
    }

    /// For when creating the source directory failed.
    @backtraced
    failed_to_create_source_directory {
        args: (error: impl ErrorArg),
        msg: format!("Failed creating source directory {error}."),
        help: None,
    }

    /// For when getting a Leo file entry failed.
    @backtraced
    failed_to_get_leo_file_entry {
        args: (error: impl ErrorArg),
        msg: format!("Failed to get Leo file entry: {error}."),
        help: None,
    }

    /// For when getting the source file extension failed.
    @backtraced
    failed_to_get_leo_file_extension {
        args: (extension: impl Debug),
        msg: format!("Failed to get Leo file extension: {extension:?}."),
        help: None,
    }

    /// For when the Leo file has an invalid extension.
    @backtraced
    invalid_leo_file_extension {
        args: (file: impl Debug, extension: impl Debug),
        msg: format!("Source file `{file:?}` has invalid extension: {extension:?}."),
        help: None,
    }

    /// For when the package failed to initalize.
    @backtraced
    failed_to_initialize_package {
        args: (package: impl Display, path: impl Debug),
        msg: format!("failed to initialize package {package} {path:?}"),
        help: None,
    }

    /// For when the package has an invalid name.
    @backtraced
    invalid_package_name {
        args: (package: impl Display),
        msg: format!("invalid project name {package}"),
        help: None,
    }

    /// For when opening a directory failed.
    @backtraced
    directory_not_found {
        args: (dirname: impl Display, path: impl Display),
        msg: format!("The `{dirname}` does not exist at `{path}`."),
        help: None,
    }

    /// For when creating a directory failed.
    @backtraced
    failed_to_create_directory {
        args: (dirname: impl Display, error: impl ErrorArg),
        msg: format!("failed to create directory `{dirname}`, error: {error}."),
        help: None,
    }

    /// For when removing a directory failed.
    @backtraced
    failed_to_remove_directory {
        args: (dirname: impl Display, error: impl ErrorArg),
        msg: format!("failed to remove directory: {dirname}, error: {error}"),
        help: None,
    }

    /// For when file could not be read.
    @backtraced
    failed_to_read_file {
        args: (path: impl Display, error: impl ErrorArg),
        msg: format!("failed to read file: {path}, error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_get_file_name {
        args: (),
        msg: "Failed to get names of Leo files in the `src/` directory.".to_string(),
        help: Some("Check your `src/` directory for invalid file names.".to_string()),
    }

    @backtraced
    failed_to_set_cwd {
        args: (dir: impl Display, error: impl ErrorArg),
        msg: format!("Failed to set current working directory to `{dir}`. Error: {error}."),
        help: None,
    }

    @backtraced
    failed_to_open_manifest {
        args: (error: impl Display),
        msg: format!("Failed to open manifest file: {error}"),
        help: Some("Create a package by running `leo new`.".to_string()),
    }

    @backtraced
    failed_to_open_aleo_file {
        args: (error: impl Display),
        msg: format!("Failed to open Aleo file: {error}"),
        help: Some("Create a package by running `leo new`.".to_string()),
    }

    @backtraced
    failed_to_create_aleo_file {
        args: (error: impl Display),
        msg: format!("Failed to create Aleo file: {error}."),
        help: None,
    }

    @backtraced
    failed_to_write_aleo_file {
        args: (error: impl Display),
        msg: format!("Failed to write aleo file: {error}."),
        help: None,
    }

    @backtraced
    failed_to_remove_aleo_file {
        args: (error: impl Display),
        msg: format!("Failed to remove aleo file: {error}."),
        help: None,
    }

    @backtraced
    empty_source_directory {
        args: (),
        msg: "The `src/` directory is empty.".to_string(),
        help: Some("Add a `main.leo` file to the `src/` directory.".to_string()),
    }

    @backtraced
    source_directory_can_contain_only_one_file {
        args: (),
        msg: "The `src/` directory can contain only one file and must be named `main.leo`.".to_string(),
        help: None,
    }

    /// For when the environment file has an IO error.
    @backtraced
    io_error_env_file {
        args: (error: impl ErrorArg),
        msg: format!("IO error env file from the provided file path - {error}"),
        help: None,
    }
);
