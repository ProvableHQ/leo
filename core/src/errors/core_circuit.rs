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

use crate::Value;
use leo_typed::{Error as FormattedError, Span};

use snarkos_errors::gadgets::SynthesisError;

use std::path::Path;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CoreCircuitError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl CoreCircuitError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            CoreCircuitError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        CoreCircuitError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn arguments_length(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!("Core circuit expected {} arguments, found {}", expected, actual);

        CoreCircuitError::new_from_span(message, span)
    }

    pub fn array_length(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Core circuit expected an array of length {}, found an array of length {}",
            expected, actual
        );

        CoreCircuitError::new_from_span(message, span)
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "The gadget operation `{}` failed due to synthesis error `{:?}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_array(actual: Value, span: Span) -> Self {
        let message = format!("Core circuit expected an array argument, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_array_bytes(actual: Value, span: Span) -> Self {
        let message = format!(
            "Core circuit expected an array of UInt8 gadgets, found an array of `{}`",
            actual
        );

        Self::new_from_span(message, span)
    }
}
