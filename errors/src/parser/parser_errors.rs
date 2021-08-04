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

use std::fmt::Display;

create_errors!(
    ParserError,
    exit_code_mask: 5000u32,
    error_code_prefix: "PAR",

    @formatted
    unexpected_token {
        args: (message: impl Display, help: String),
        msg: message,
        help: Some(help),
    }

    @formatted
    invalid_address_lit {
        args: (token: impl Display),
        msg: format!("invalid address literal: '{}'", token),
        help: None,
    }

    @formatted
    invalid_import_list {
        args: (),
        msg: "Cannot import empty list",
        help: None,
    }

    @formatted
    unexpected_eof {
        args: (),
        msg: "unexpected EOF",
        help: None,
    }

    @formatted
    unexpected_whitespace {
        args: (left: impl Display, right: impl Display),
        msg: format!("Unexpected white space between terms {} and {}", left, right),
        help: None,
    }

    @formatted
    unexpected {
        args: (got: impl Display, expected: impl Display),
        msg: format!("expected {} -- got '{}'", expected, got),
        help: None,
    }

    @formatted
    mixed_commas_and_semicolons {
        args: (),
        msg: "Cannot mix use of commas and semi-colons for circuit member variable declarations.",
        help: None,
    }

    @formatted
    unexpected_ident {
        args: (got: impl Display, expected: &[impl Display]),
        msg: format!(
            "unexpected identifier: expected {} -- got '{}'",
            expected
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<_>>()
                .join(", "),
            got
        ),
        help: None,
    }

    @formatted
    unexpected_statement {
        args: (got: impl Display, expected: impl Display),
        msg: format!("unexpected statement: expected '{}', got '{}'", expected, got),
        help: None,
    }

    @formatted
    unexpected_str {
        args: (got: impl Display, expected: impl Display),
        msg: format!("unexpected string: expected '{}', got '{}'", expected, got),
        help: None,
    }

    @formatted
    spread_in_array_init {
        args: (),
        msg: "illegal spread in array initializer",
        help: None,
    }

    @formatted
    invalid_assignment_target {
        args: (),
        msg: "invalid assignment target",
        help: None,
    }

    @formatted
    invalid_package_name {
        args: (),
        msg: "package names must be lowercase alphanumeric ascii with underscores and singular dashes",
        help: None,
    }

    @formatted
    illegal_self_const {
        args: (),
        msg: "cannot have const self",
        help: None,
    }

    @formatted
    mut_function_input {
        args: (),
        msg: "function func(mut a: u32) { ... } is deprecated. Passed variables are mutable by default.",
        help: None,
    }

    @formatted
    let_mut_statement {
        args: (),
        msg: "let mut = ... is deprecated. `let` keyword implies mutabality by default.",
        help: None,
    }

    @formatted
    test_function {
        args: (),
        msg: "\"test function...\" is deprecated. Did you mean @test annotation?",
        help: None,
    }

    @formatted
    context_annotation {
        args: (),
        msg: "\"@context(...)\" is deprecated. Did you mean @test annotation?",
        help: None,
    }
);
