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
use std::fmt::{Debug, Display};

create_errors!(
    /// InputError enum that represents all the errors for the inputs part of `leo-ast` crate.
    InputError,
    exit_code_mask: 8000i32,
    error_code_prefix: "INP",

    /// For when declared variable type mismatches actual type.
    @formatted
    unexpected_type {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "unexpected type, expected: '{}', received: '{}'",
            expected,
            received,
        ),
        help: None,
    }

    /// For when string value is assigned to an array of non Char type.
    @formatted
    string_is_array_of_chars {
        args: (expected: impl Display),
        msg: format!(
            "strings transforms into array of 'char', expected: {}",
            expected,
        ),
        help: None,
    }

    /// For when [`ArrayDimensions`] are not specified.
    @formatted
    array_dimensions_must_be_specified {
        args: (),
        msg: "array dimensions must be specified",
        help: None,
    }

    /// For when array init is using spread.
    @formatted
    array_spread_is_not_allowed {
        args: (),
        msg: "array spread is not allowed in inputs",
        help: None,
    }

    /// For when any of the array dimensions is zero.
    @formatted
    invalid_array_dimension_size {
        args: (),
        msg: "received dimension size of 0, expected it to be 1 or larger.",
        help: None,
    }

    /// For when the expression is not allowed in an input file.
    @formatted
    illegal_expression {
        args: (expr: impl Display),
        msg: format!("expression '{}' is not allowed in inputs", expr),
        help: None,
    }

    /// For when section name is not an allowed one.
    @formatted
    unexpected_section {
        args: (expected: &[impl Display], received: impl Display),
        msg: format!(
            "unexpected section: expected {} -- got '{}'",
            expected
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<_>>()
                .join(", "),
            received
        ),
        help: None,
    }

    /// For when declared tuple length is not equal to the value's.
    @formatted
    tuple_length_mismatch {
        args: (expected: impl Display, received: impl Display),
        msg: format!("tuple length mismatch, defined {} types, got {} values", expected, received),
        help: None,
    }
);
