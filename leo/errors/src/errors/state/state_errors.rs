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
    /// StateError enum that represents all the errors for the `leo-state` crate.
    StateError,
    exit_code_mask: 1000i32,
    error_code_prefix: "STA",

    /// For when it cannot parse the state boolean value.
    @backtraced
    parse_bool_error {
        args: (error: impl ErrorArg),
        msg: format!("failed to parse state file bool: {}", error),
        help: None,
    }

    /// For when it cannot parse the state int value.
    @backtraced
    parse_int_error {
        args: (error: impl ErrorArg),
        msg: format!("failed to parse state file int: {}", error),
        help: None,
    }

    /// For when it expected an array of u8 bytes but found something else.
    @backtraced
    expected_bytes {
        args: (found: impl Display),
        msg: format!("expected parameter array of u8 bytes, found `{}`", found),
        help: None,
    }

    /// For when an int was expected but received something else.
    @backtraced
    expected_int {
        args: (found: impl Display),
        msg: format!("expected integer parameter, found `{}`", found),
        help: None,
    }

    /// For when it cannot parse the state int value.
    @backtraced
    missing_parameter {
        args: (parameter: impl Display),
        msg: format!("input parameter `{}` not found in state file", parameter),
        help: None,
    }

    /// For when the crate encounters an IO error.
    @backtraced
    state_io_error {
        args: (error: impl ErrorArg),
        msg: format!("io error found {}", error),
        help: None,
    }
);
