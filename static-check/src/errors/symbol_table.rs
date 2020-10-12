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

use crate::TypeError;
use leo_typed::{Error as FormattedError, Identifier, Span};

use std::path::PathBuf;

/// Errors encountered when tracking variable, function, and circuit names in a program
#[derive(Debug, Error)]
pub enum SymbolTableError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    TypeError(#[from] TypeError),
}

impl SymbolTableError {
    ///
    /// Set the filepath for the error stacktrace
    ///
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            SymbolTableError::Error(error) => error.set_path(path),
            SymbolTableError::TypeError(error) => error.set_path(path),
        }
    }

    ///
    /// Return a new formatted error with a given message and span information
    ///
    fn new_from_span(message: String, span: Span) -> Self {
        SymbolTableError::Error(FormattedError::new_from_span(message, span))
    }

    ///
    /// Two circuits have been defined with the same name
    ///
    pub fn duplicate_circuit(identifier: Identifier, span: Span) -> Self {
        let message = format!("Duplicate circuit definition found for `{}`", identifier);

        Self::new_from_span(message, span)
    }

    ///
    /// Two functions have been defined with the same name
    ///
    pub fn duplicate_function(identifier: Identifier, span: Span) -> Self {
        let message = format!("Duplicate function definition found for `{}`", identifier);

        Self::new_from_span(message, span)
    }
}
