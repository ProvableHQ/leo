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
    /// AstError enum that represents all the errors for the `leo-ast` crate.
    AstError,
    exit_code_mask: 2000i32,
    error_code_prefix: "AST",

    /// For when the AST fails to be represented as a JSON string.
    @backtraced
    failed_to_convert_ast_to_json_string {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert ast to a json string {}", error),
        help: None,
    }

    /// For when the AST fails to create the AST JSON file.
    @backtraced
    failed_to_create_ast_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to creat ast json file `{:?}` {}", path, error),
        help: None,
    }

    /// For when the AST fails to write the AST JSON file.
    @backtraced
    failed_to_write_ast_to_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to write ast to a json file `{:?}` {}", path, error),
        help: None,
    }

    /// For when the a JSON string fails to be represented as an AST.
    @backtraced
    failed_to_read_json_string_to_ast {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert json string to an ast {}", error),
        help: None,
    }

    /// For when the a JSON files fails to be represented as an AST.
    @backtraced
    failed_to_read_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to convert json file `{:?}` to an ast {}", path, error),
        help: None,
    }

    /// For when a user tries to use the `Self` keyword outside of a cricuit.
    @formatted
    big_self_outside_of_circuit {
        args: (),
        msg: "cannot call keyword `Self` outside of a circuit function",
        help: None,
    }

    /// For when a user tries to define a array dimension of 0.
    @formatted
    invalid_array_dimension_size {
        args: (),
        msg: "received dimension size of 0, expected it to be 1 or larger.",
        help: None,
    }

    /// For when a user tries to give certain statements a block rather than another statement.
    @formatted
    ast_statement_not_block {
        args: (),
        msg: "AstStatement should be be a block",
        help: None,
    }

    /// For when a user tries to construct an empty string, which is a zero size array.
    @formatted
    empty_string {
        args: (),
        msg: "Cannot constrcut an empty string: it has the type of [char; 0] which is not possible.",
        help: None,
    }

    /// This error should never be reached, but represents trying to expand a console assert.
    @formatted
    impossible_console_assert_call {
        args: (),
        msg: "Console::Assert cannot be matched here, its handled in another case.",
        help: None,
    }

    /// This error is for when a user tries to use the library and programatically inject an import
    /// on the rust side.
    @backtraced
    injected_programs {
        args: (injected_import_count: impl Display),
        msg: format!("It seems the AST has {} injected imports. This is unexpected please import the library naturally", injected_import_count),
        help: None,
    }

    /// For when a import of the specified name is unresolved.
    @formatted
    unresolved_import {
        args: (name: impl Display),
        msg: format!("failed to resolve import: '{}'", name),
        help: None,
    }

    /// For when the AST fails to be represented as a JSON value.
    @backtraced
    failed_to_convert_ast_to_json_value {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert ast to a json value {}", error),
        help: None,
    }

    /// For when const function modifier is added to the main function.
    @formatted
    main_cannot_be_const {
        args: (),
        msg: "main function cannot be const",
        help: None,
    }

    /// For when const function has non-const inputs or self.
    @formatted
    const_function_cannot_have_inputs {
        args: (),
        msg: "const function cannot have non-const input",
        help: None,
    }

    /// For when `main` is annotated.
    @formatted
    main_cannot_have_annotations {
        args: (),
        msg: "main function cannot have annotations",
        help: None,
    }

    /// For when unsupported annotation is added.
    @formatted
    unsupported_annotation {
        args: (name: impl Display),
        msg: format!("annotation `{}` does not exist", name),
        help: None,
    }
);
