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
    Backtraced::error(CODE_PREFIX, CODE_MASK, format!("failed to convert the AST to a JSON string: {error}"))
        .with_help("This is an internal serialization failure. Re-run the build; if it persists, please file an issue.")
}

pub(crate) fn failed_to_create_ast_json_file(path: &dyn Debug, error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 1, format!("failed to create AST JSON file `{path:?}`: {error}"))
        .with_help("Check that the build output directory exists and is writable.")
}

pub(crate) fn failed_to_write_ast_to_json_file(path: &dyn Debug, error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 2, format!("failed to write the AST to JSON file `{path:?}`: {error}"))
        .with_help("Check that the build output directory is writable and has enough free space.")
}

pub(crate) fn failed_to_read_json_string_to_ast(error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 3, format!("failed to deserialize a JSON string into an AST: {error}"))
        .with_help("The cached AST JSON is corrupt or was produced by an incompatible Leo version. Run `leo clean` and rebuild.")
}

pub(crate) fn failed_to_read_json_file(path: &dyn Debug, error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("failed to deserialize JSON file `{path:?}` into an AST: {error}"),
    )
    .with_help(
        "The cached AST JSON is corrupt or was produced by an incompatible Leo version. Run `leo clean` and rebuild.",
    )
}

pub(crate) fn failed_to_convert_ast_to_json_value(error: &dyn ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 5, format!("failed to convert the AST to a JSON value: {error}"))
        .with_help("This is an internal serialization failure. Re-run the build; if it persists, please file an issue.")
}

pub(crate) fn invalid_network_name(network: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 18, format!("invalid network name: `{network}`"))
        .with_help("Valid network names are `testnet`, `mainnet`, and `canary`.")
}
