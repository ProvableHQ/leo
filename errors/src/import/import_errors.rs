// Copyright (C) 2019-2021 Aleo Systems Inc.
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
    ImportError,
    exit_code_mask: 3000i32,
    error_code_prefix: "IMP",

    // An imported package has the same name as an imported core_package.
    @formatted
    conflicting_imports {
        args: (name: impl Display),
        msg: format!("conflicting imports found for `{}`.", name),
        help: None,
    }

    @formatted
    recursive_imports {
        args: (package: impl Display),
        msg: format!("recursive imports for `{}`.", package),
        help: None,
    }

    // Failed to convert a file path into an os string.
    @formatted
    convert_os_string {
        args: (),
        msg: "Failed to convert file string name, maybe an illegal character?",
        help: None,
    }

    // Failed to find the directory of the current file.
    @formatted
    current_directory_error {
        args: (error: impl ErrorArg),
        msg: format!("Compilation failed trying to find current directory - {:?}.", error),
        help: None,
    }

    // Failed to open or get the name of a directory.
    @formatted
    directory_error {
        args: (error: impl ErrorArg, path:impl Debug),
        msg: format!(
            "Compilation failed due to directory error @ '{:?}' - {:?}.",
            path,
            error
        ),
        help: None,
    }

    // Failed to find a main file for the current package.
    @formatted
    expected_main_file {
        args: (entry: impl Debug),
        msg: format!("Expected main file at `{:?}`.", entry),
        help: None,
    }

    // Failed to import a package name.
    @formatted
    unknown_package {
        args: (name: impl Display),
        msg: format!(
            "Cannot find imported package `{}` in source files or import directory.",
            name
        ),
        help: None,
    }

    @formatted
    io_error {
        args: (path: impl Display, error: impl ErrorArg),
        msg: format!("cannot read imported file '{}': {:?}", path, error),
        help: None,
    }
);
