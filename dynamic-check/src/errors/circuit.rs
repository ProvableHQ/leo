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

use crate::FunctionError;
use leo_typed::{Error as FormattedError, Span};

use std::path::PathBuf;

/// Errors encountered when resolving a program
#[derive(Debug, Error)]
pub enum CircuitError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FunctionError(#[from] FunctionError),
}

impl CircuitError {
    /// Set the filepath for the error stacktrace
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            CircuitError::Error(error) => error.set_path(path),
            CircuitError::FunctionError(error) => error.set_path(path),
        }
    }

    /// Return a new formatted error with a given message and span information
    fn new_from_span(message: String, span: Span) -> Self {
        CircuitError::Error(FormattedError::new_from_span(message, span))
    }
}
