// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use leo_ast::{Error as FormattedError, Span};

use std::path::Path;

/// Errors encountered when tracking variable names in a program.
#[derive(Debug, Error)]
pub enum VariableTableError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl VariableTableError {
    ///
    /// Set the filepath for the error stacktrace
    ///
    pub fn set_path(&mut self, path: &Path) {
        match self {
            VariableTableError::Error(error) => error.set_path(path),
        }
    }

    ///
    /// Return a new formatted error with a given message and span information
    ///
    fn new_from_span(message: String, span: Span) -> Self {
        VariableTableError::Error(FormattedError::new_from_span(message, span))
    }

    ///
    /// Attempted to define two function inputs with the same name.
    ///
    pub fn duplicate_function_input(name: &str, span: &Span) -> Self {
        let message = format!("Duplicate function input `{}`found in function signature.", name);

        Self::new_from_span(message, span.clone())
    }
}
