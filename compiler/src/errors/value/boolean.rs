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

use leo_typed::{Error as FormattedError, Span};

use snarkos_errors::gadgets::SynthesisError;
use std::path::Path;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl BooleanError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            BooleanError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        BooleanError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the boolean operation `{}` failed due to the synthesis error `{:?}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn cannot_evaluate(operation: String, span: Span) -> Self {
        let message = format!("no implementation found for `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn invalid_boolean(actual: String, span: Span) -> Self {
        let message = format!("expected boolean input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_boolean(expected: String, span: Span) -> Self {
        let message = format!("expected boolean input `{}` not found", expected);

        Self::new_from_span(message, span)
    }
}
