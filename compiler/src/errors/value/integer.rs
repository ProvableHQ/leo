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

use leo_gadgets::errors::SignedIntegerError;
use leo_typed::{error::Error as FormattedError, Span};

use snarkos_errors::gadgets::SynthesisError;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl IntegerError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            IntegerError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        IntegerError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the integer operation `{}` failed due to the synthesis error `{:?}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn signed(error: SignedIntegerError, span: Span) -> Self {
        let message = format!("integer operation failed due to the signed integer error `{:?}`", error);

        Self::new_from_span(message, span)
    }

    pub fn synthesis(error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "integer operation failed due to the synthesis error or overflow `{:?}`",
            error
        );

        Self::new_from_span(message, span)
    }

    pub fn signed_error(operation: String, error: SignedIntegerError, span: Span) -> Self {
        let message = format!(
            "the integer operation `{}` failed due to the signed integer error `{:?}`",
            operation, error
        );

        Self::new_from_span(message, span)
    }

    pub fn negate_operation(span: Span) -> Self {
        let message = format!("integer negation can only be enforced on signed integers");

        Self::new_from_span(message, span)
    }

    pub fn binary_operation(operation: String, span: Span) -> Self {
        let message = format!(
            "the integer binary operation `{}` can only be enforced on integers of the same type",
            operation
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_index(span: Span) -> Self {
        let message = format!(
            "index must be a constant value unsigned integer. allocated indices produce a circuit of unknown size"
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_integer(actual: String, span: Span) -> Self {
        let message = format!("failed to parse `{}` as expected integer type", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_integer(expected: String, span: Span) -> Self {
        let message = format!("expected integer input `{}` not found", expected);

        Self::new_from_span(message, span)
    }
}
