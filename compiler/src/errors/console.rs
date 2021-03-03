// Copyright (C) 2019-2021 Aleo Systems Inc.
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
use leo_ast::{FormattedError, LeoError, Span};

#[derive(Debug, Error)]
pub enum ConsoleError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    Expression(#[from] ExpressionError),
}

impl LeoError for ConsoleError {
    fn get_path(&self) -> Option<&str> {
        match self {
            ConsoleError::Error(error) => error.get_path(),
            ConsoleError::Expression(error) => error.get_path(),
        }
    }

    fn set_path(&mut self, path: &str, contents: &[String]) {
        match self {
            ConsoleError::Error(error) => error.set_path(path, contents),
            ConsoleError::Expression(error) => error.set_path(path, contents),
        }
    }
}

impl ConsoleError {
    fn new_from_span(message: String, span: &Span) -> Self {
        ConsoleError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn length(containers: usize, parameters: usize, span: &Span) -> Self {
        let message = format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        );

        Self::new_from_span(message, span)
    }

    pub fn assertion_depends_on_input(span: &Span) -> Self {
        let message =
            "console.assert() failed to evaluate. This error is caused by empty input file values".to_string();

        Self::new_from_span(message, span)
    }

    pub fn assertion_failed(span: &Span) -> Self {
        let message = "Assertion failed".to_string();

        Self::new_from_span(message, span)
    }

    pub fn assertion_must_be_boolean(span: &Span) -> Self {
        let message = "Assertion expression must evaluate to a boolean value".to_string();

        Self::new_from_span(message, span)
    }
}
