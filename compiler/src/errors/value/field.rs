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

use leo_ast::{FormattedError, LeoError, Span};

#[derive(Debug, Error)]
pub enum FieldError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl LeoError for FieldError {}

impl FieldError {
    fn new_from_span(message: String, span: &Span) -> Self {
        FieldError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn invalid_field(actual: String, span: &Span) -> Self {
        let message = format!("expected field element input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_field(expected: String, span: &Span) -> Self {
        let message = format!("expected field input `{}` not found", expected);

        Self::new_from_span(message, span)
    }

    pub fn no_inverse(field: String, span: &Span) -> Self {
        let message = format!("no multiplicative inverse found for field `{}`", field);

        Self::new_from_span(message, span)
    }
}
