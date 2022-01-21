// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::create_errors;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_errors!(
    /// PackageError enum that represents all the errors for the `leo-package` crate.
    PackageError,
    exit_code_mask: 5000i32,
    error_code_prefix: "PAK",

    /// For when the specified import does not exist.
    @backtraced
    import_does_not_exist {
        args: (package: impl Display),
        msg: format!("package {} does not exist as an import", package),
        help: None,
    }

    /// For when removing the imports directory failed.
    @backtraced
    failed_to_remove_imports_directory {
        args: (error: impl ErrorArg),
        msg: format!("failed removing imports directory {}", error),
        help: None,
    }

    /// For when getting a input file entry failed.
    @backtraced
    failed_to_get_input_file_entry {
        args: (error: impl ErrorArg),
        msg: format!("failed to get input file entry: {}", error),
        help: None,
    }

    /// For when getting the input file name failed.
    @backtraced
    failed_to_get_input_file_name {
        args: (file: impl Debug),
        msg: format!("failed to get input file name: {:?}", file),
        help: None,
    }

    /// For when getting the input file type failed.
    @backtraced
    failed_to_get_input_file_type {
        args: (file: impl Debug, error: impl ErrorArg),
        msg: format!("failed to get input file `{:?}` type: {}", file, error),
        help: None,
    }

    /// For when getting the input file has an invalid file type.
    @backtraced
    invalid_input_file_type {
        args: (file: impl Debug, type_: std::fs::FileType),
        msg: format!("input file `{:?}` has invalid type: {:?}", file, type_),
        help: None,
    }

    /// For when reading the input directory failed.
    @backtraced
    failed_to_read_inputs_directory {
        args: (error: impl ErrorArg),
        msg: format!("failed reading inputs directory {}", error),
        help: None,
    }

    /// For when reading the input file failed.
    @backtraced
    failed_to_read_input_file {
        args: (path: impl Debug),
        msg: format!("Cannot read input file from the provided file path - {:?}", path),
        help: None,
    }

    /// For when reading the proving key failed.
    @backtraced
    failed_to_read_proving_key_file {
        args: (path: impl Debug),
        msg: format!("Cannot read proving key file from the provided file path - {:?}", path),
        help: None,
    }

    /// For when reading the verification key file failed.
    @backtraced
    failed_to_read_verification_key_file {
        args: (path: impl Debug),
        msg: format!("Cannot read verification key file from the provided file path - {:?}", path),
        help: None,
    }

    /// For when parsing the manifest file failed.
    @backtraced
    failed_to_parse_manifest_file {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed parsing manifest file `{}` {}", filename, error),
        help: None,
    }

    /// For when reading the manifest file failed.
    @backtraced
    failed_to_read_manifest_file {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed reading manifest file `{}` {}", filename, error),
        help: None,
    }

    /// For when creating the zip file failed.
    @backtraced
    failed_to_create_zip_file {
        args: (error: impl ErrorArg),
        msg: format!("failed creating zip file {}", error),
        help: None,
    }

    /// For when opening the zip file failed.
    @backtraced
    failed_to_open_zip_file {
        args: (error: impl ErrorArg),
        msg: format!("failed opening zip file {}", error),
        help: None,
    }

    /// For when reading the zip file failed.
    @backtraced
    failed_to_read_zip_file {
        args: (error: impl ErrorArg),
        msg: format!("failed reading zip file {}", error),
        help: None,
    }

    /// For when writing the zip file failed.
    @backtraced
    failed_to_write_zip_file {
        args: (error: impl ErrorArg),
        msg: format!("failed writing zip file {}", error),
        help: None,
    }

    /// For when the zip file has an IO error.
    @backtraced
    io_error_zip_file {
        args: (error: impl ErrorArg),
        msg: format!("IO error zip file from the provided file path - {}", error),
        help: None,
    }

    /// For when removing the zip file failed.
    @backtraced
    failed_to_remove_zip_file {
        args: (path: impl Debug),
        msg: format!("failed removing zip file from the provided file path - {:?}", path),
        help: None,
    }

    /// For when getting a source file entry failed.
    @backtraced
    failed_to_get_source_file_entry {
        args: (error: impl ErrorArg),
        msg: format!("failed to get input file entry: {}", error),
        help: None,
    }

    /// For when getting the source file extension failed.
    @backtraced
    failed_to_get_source_file_extension {
        args: (extension: impl Debug),
        msg: format!("failed to get source file extension: {:?}", extension),
        help: None,
    }

    /// For when getting the source file type failed.
    @backtraced
    failed_to_get_source_file_type {
        args: (file: impl Debug, error: impl ErrorArg),
        msg: format!("failed to get source file `{:?}` type: {}", file, error),
        help: None,
    }

    /// For when getting the source file has an invalid extension.
    @backtraced
    invalid_source_file_extension {
        args: (file: impl Debug, extension: impl Debug),
        msg: format!("source file `{:?}` has invalid extension: {:?}", file, extension),
        help: None,
    }

    /// For when getting the source file has an invalid file type.
    @backtraced
    invalid_source_file_type {
        args: (file: impl Debug, type_: std::fs::FileType),
        msg: format!("source file `{:?}` has invalid type: {:?}", file, type_),
        help: None,
    }

    /// For when the package failed to initalize.
    @backtraced
    failed_to_initialize_package {
        args: (package: impl Display, path: impl Debug),
        msg: format!("failed to initialize package {} {:?}", package, path),
        help: None,
    }

    /// For when the package has an invalid name.
    @backtraced
    invalid_package_name {
        args: (package: impl Display),
        msg: format!("invalid project name {}", package),
        help: None,
    }

    @backtraced
    failed_to_create_lock_file {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed creating manifest file `{}` {}", filename, error),
        help: None,
    }

    /// For when getting the lock file metadata failed.
    @backtraced
    failed_to_get_lock_file_metadata {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed getting lock file metadata `{}` {}", filename, error),
        help: None,
    }

    /// For when opening the lock file failed.
    @backtraced
    failed_to_open_lock_file {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed openining lock file `{}` {}", filename, error),
        help: None,
    }

    /// For when parsing the lock file failed.
    @backtraced
    failed_to_parse_lock_file {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed parsing lock file `{}` {}", filename, error),
        help: None,
    }

    /// For when reading the lock file failed.
    @backtraced
    failed_to_read_lock_file {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed reading lock file `{}` {}", filename, error),
        help: None,
    }

    /// For when writing the lock file failed.
    @backtraced
    failed_to_write_lock_file {
        args: (filename: impl Display, error: impl ErrorArg),
        msg: format!("failed writing lock file `{}` {}", filename, error),
        help: None,
    }

    @backtraced
    failed_to_serialize_lock_file {
        args: (error: impl ErrorArg),
        msg: format!("serialization failed: {}", error),
        help: None,
    }

    /// For when creating a directory failed.
    @backtraced
    failed_to_create_directory {
        args: (dirname: impl Display, error: impl ErrorArg),
        msg: format!("failed to create directory: {}, error: {}", dirname, error),
        help: None,
    }

    /// For when removing a directory failed.
    @backtraced
    failed_to_remove_directory {
        args: (dirname: impl Display, error: impl ErrorArg),
        msg: format!("failed to remove directory: {}, error: {}", dirname, error),
        help: None,
    }

    /// For when file could not be read.
    @backtraced
    failed_to_read_file {
        args: (path: impl Display, error: impl ErrorArg),
        msg: format!("failed to read file: {}, error: {}", path, error),
        help: None,
    }

    /// For when file failed to remove.
    @backtraced
    failed_to_remove_file {
        args: (path: impl Display, error: impl ErrorArg),
        msg: format!("failed to remove file: {}, error: {}", path, error),
        help: None,
    }

    /// For when I/O operation failed.
    @backtraced
    io_error {
        args: (file: impl Display, error: impl ErrorArg),
        msg: format!("i/o operation failed, file: {}, error: {}", file, error),
        help: None,
    }
);
