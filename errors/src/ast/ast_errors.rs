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
use std::{error::Error as ErrorArg, fmt::Debug};

create_errors!(
    AstError,
    exit_code_mask: 1000i32,
    error_code_prefix: "AST",

    @backtraced
    failed_to_convert_ast_to_json_string {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert ast to a json string {}", error),
        help: None,
    }

    @backtraced
    failed_to_create_ast_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to creat ast json file `{:?}` {}", path, error),
        help: None,
    }

    @backtraced
    failed_to_write_ast_to_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to write ast to a json file `{:?}` {}", path, error),
        help: None,
    }

    @backtraced
    failed_to_read_json_string_to_ast {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert json stirng to an ast {}", error),
        help: None,
    }

    @backtraced
    failed_to_read_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to convert json file `{:?}` to an ast {}", path, error),
        help: None,
    }

    @formatted
    big_self_outside_of_circuit {
        args: (),
        msg: "cannot call keyword `Self` outside of a circuit function",
        help: None,
    }

    @formatted
    invalid_array_dimension_size {
        args: (),
        msg: "received dimension size of 0, expected it to be 1 or larger.",
        help: None,
    }

    @formatted
    asg_statement_not_block {
        args: (),
        msg: "AstStatement should be be a block",
        help: None,
    }

    @formatted
    empty_string {
        args: (),
        msg: "Cannot constrcut an empty string: it has the type of [char; 0] which is not possible.",
        help: None,
    }

    @formatted
    impossible_console_assert_call {
        args: (),
        msg: "Console::Assert cannot be matched here, its handled in another case.",
        help: None,
    }
);
