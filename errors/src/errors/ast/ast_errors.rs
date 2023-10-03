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
    /// AstError enum that represents all the errors for the `leo-ast` crate.
    AstError,
    code_mask: 2000i32,
    code_prefix: "AST",

    /// For when the AST fails to be represented as a JSON string.
    @backtraced
    failed_to_convert_ast_to_json_string {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert ast to a json string {error}"),
        help: None,
    }

    /// For when the AST fails to create the AST JSON file.
    @backtraced
    failed_to_create_ast_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to create ast json file `{path:?}` {error}"),
        help: None,
    }

    /// For when the AST fails to write the AST JSON file.
    @backtraced
    failed_to_write_ast_to_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to write ast to a json file `{path:?}` {error}"),
        help: None,
    }

    /// For when the a JSON string fails to be represented as an AST.
    @backtraced
    failed_to_read_json_string_to_ast {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert json string to an ast {error}"),
        help: None,
    }

    /// For when the a JSON files fails to be represented as an AST.
    @backtraced
    failed_to_read_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to convert json file `{path:?}` to an ast {error}"),
        help: None,
    }

    /// For when the AST fails to be represented as a JSON value.
    @backtraced
    failed_to_convert_ast_to_json_value {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert ast to a json value {error}"),
        help: None,
    }

    /// For when a user shadows a function.
    @formatted
    shadowed_function {
        args: (func: impl Display),
        msg: format!("function `{func}` shadowed by"),
        help: None,
    }

    /// For when a user shadows a struct.
    @formatted
    shadowed_struct {
        args: (struct_: impl Display),
        msg: format!("struct `{struct_}` shadowed by"),
        help: None,
    }

    /// For when a user shadows a record.
    @formatted
    shadowed_record {
        args: (record: impl Display),
        msg: format!("record `{record}` shadowed by"),
        help: None,
    }

    /// For when a user shadows a variable.
    @formatted
    shadowed_variable {
        args: (var: impl Display),
        msg: format!("variable `{var}` shadowed by"),
        help: None,
    }

    /// For when the symbol table fails to be represented as a JSON string.
    @backtraced
    failed_to_convert_symbol_table_to_json_string {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert symbol_table to a json string {error}"),
        help: None,
    }

    /// For when the symbol table fails to create the symbol table JSON file.
    @backtraced
    failed_to_create_symbol_table_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to create symbol_table json file `{path:?}` {error}"),
        help: None,
    }

    /// For when the symbol table fails to write the symbol table JSON file.
    @backtraced
    failed_to_write_symbol_table_to_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to write symbol_table to a json file `{path:?}` {error}"),
        help: None,
    }

    /// For when the a JSON string fails to be represented as an symbol table.
    @backtraced
    failed_to_read_json_string_to_symbol_table {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert json string to an symbol_table {error}"),
        help: None,
    }

    /// For when the symbol table fails to be represented as a JSON value.
    @backtraced
    failed_to_convert_symbol_table_to_json_value {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert symbol_table to a json value {error}"),
        help: None,
    }
);
