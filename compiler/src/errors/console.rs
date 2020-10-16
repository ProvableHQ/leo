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

use crate::errors::ExpressionError;
use leo_typed::{Error as FormattedError, Span};

use std::path::Path;

#[derive(Debug, Error)]
pub enum ConsoleError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    Expression(#[from] ExpressionError),
}

impl ConsoleError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            ConsoleError::Expression(error) => error.set_path(path),
            ConsoleError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        ConsoleError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn length(containers: usize, parameters: usize, span: Span) -> Self {
        let message = format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        );

        Self::new_from_span(message, span)
    }

    pub fn assertion_depends_on_input(span: Span) -> Self {
        let message =
            "console.assert() failed to evaluate. This error is caused by empty input file values".to_string();

        Self::new_from_span(message, span)
    }

    pub fn assertion_failed(expression: String, span: Span) -> Self {
        let message = format!("Assertion `{}` failed", expression);

        Self::new_from_span(message, span)
    }

    pub fn assertion_must_be_boolean(expression: String, span: Span) -> Self {
        let message = format!("Assertion expression `{}` must evaluate to a boolean value", expression);

        Self::new_from_span(message, span)
    }
}
