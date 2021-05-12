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

use crate::errors::FieldError;
use leo_ast::{FormattedError, LeoError, Span};

#[derive(Debug, Error)]
pub enum CharError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),
}

impl LeoError for CharError {}

impl CharError {
    fn new_from_span(message: String, span: &Span) -> Self {
        CharError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn invalid_char(actual: String, span: &Span) -> Self {
        let message = format!("expected char element input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }
}
