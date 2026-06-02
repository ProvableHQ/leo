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

//! Errors emitted by `handle_build`. The CLI's own errors module owns
//! everything else; this is just the minimum surface the build core needs.

use leo_errors::Backtraced;
use std::{error::Error as ErrorArg, fmt::Display};

const CODE_PREFIX: &str = "CLI";
const CODE_MASK: i32 = 7000;

pub fn util_file_io_error(msg: impl Display, err: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 1, format!("filesystem I/O error: {msg}: {err}"))
        .with_help("Check the target path and the current process's permissions.")
}

pub fn failed_to_load_instructions(err: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 2, format!("failed to write Aleo instructions: {err}"))
        .with_help("Verify the build directory is writable.")
}

pub fn failed_to_serialize_abi(err: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 3, format!("failed to serialize program ABI: {err}"))
}

pub fn program_size_limit_exceeded(name: impl Display, size: usize, limit: usize) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("program `{name}.aleo` is {size} bytes, exceeding the maximum allowed size of {limit} bytes"),
    )
    .with_help("Reduce the program size by removing unused code or splitting the program.")
}

pub fn failed_to_open_file(err: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 5, format!("failed to open file: {err}"))
}

pub fn custom(message: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 99, message.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn failed_to_parse_aleo_file(name: impl Display, err: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 6, format!("failed to parse `{name}.aleo`: {err}"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generated_invalid_bytecode(
    name: impl Display,
    path: impl Display,
    checksum: impl Display,
    err: impl Display,
) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 7,
        format!("compiler emitted invalid bytecode for `{name}` at `{path}` (checksum [{checksum}]): {err}"),
    )
}
