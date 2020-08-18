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

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum MacroError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    Expression(#[from] ExpressionError),
}

impl MacroError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            MacroError::Expression(error) => error.set_path(path),
            MacroError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        MacroError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn length(containers: usize, parameters: usize, span: Span) -> Self {
        let message = format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        );

        Self::new_from_span(message, span)
    }
}
