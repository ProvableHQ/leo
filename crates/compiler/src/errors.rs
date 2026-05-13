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

use leo_errors::{Backtraced, Formatted};
use leo_span::Span;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

const CODE_PREFIX: &str = "CMP";
const CODE_MASK: i32 = 6000;

pub(crate) fn file_read_error(path: impl Debug, error: impl ErrorArg) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK, format!("Cannot read from the provided file path '{path:?}': {error}"))
}

pub(crate) fn program_name_should_match_file_name(
    program_name: impl Display,
    file_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("The program name `{program_name}` must match {file_name}"),
        span,
    )
}

pub(crate) fn imported_program_not_found(
    main_program_name: impl Display,
    dependency_name: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 6,
        format!(
            "`{main_program_name}` imports `{dependency_name}`, but `{dependency_name}` is not found in program manifest. Use `leo add --help` for more information on how to add a dependency."
        ),
        span,
    )
}

pub(crate) fn failed_ast_file(filename: impl Display, error: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 11, format!("Failed to write AST to file {filename}: {error}."))
}

// Duplicated from package — same message, different code.
pub(crate) fn failed_path(path: impl Display, err: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 16, format!("Cannot find path `{path}`: {err}."))
}
