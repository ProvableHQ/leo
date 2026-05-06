// Copyright (C) 2019-2026 Provable Inc.
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

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

use leo_errors::Backtraced;

const CODE_PREFIX: &str = "AST";
const CODE_MASK: i32 = 2000;

pub(crate) fn failed_to_convert_ast_to_json_string(error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK, format!("failed to convert ast to a json string {error}"))
}

pub(crate) fn failed_to_create_ast_json_file(path: &dyn Debug, error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 1, format!("failed to create ast json file `{path:?}` {error}"))
}

pub(crate) fn failed_to_write_ast_to_json_file(path: &dyn Debug, error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 2, format!("failed to write ast to a json file `{path:?}` {error}"))
}

pub(crate) fn failed_to_read_json_string_to_ast(error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 3, format!("failed to convert json string to an ast {error}"))
}

pub(crate) fn failed_to_read_json_file(path: &dyn Debug, error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 4, format!("failed to convert json file `{path:?}` to an ast {error}"))
}

pub(crate) fn failed_to_convert_ast_to_json_value(error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 5, format!("failed to convert ast to a json value {error}"))
}

pub(crate) fn invalid_network_name(network: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 18, format!("Invalid network name: {network}"))
        .with_help("Valid network names are `testnet`, `mainnet`, and `canary`.")
}
