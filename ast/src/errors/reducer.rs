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

use crate::{CanonicalizeError, CombinerError, FormattedError, LeoError, Span};
use leo_parser::SyntaxError;

#[derive(Debug, Error)]
pub enum ReducerError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    CanonicalizeError(#[from] CanonicalizeError),

    #[error("{}", _0)]
    CombinerError(#[from] CombinerError),

    #[error("{}", _0)]
    ImportError(FormattedError),

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),
}

impl LeoError for ReducerError {}

impl ReducerError {
    fn new_from_span(message: String, span: &Span) -> Self {
        ReducerError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn empty_string(span: &Span) -> Self {
        let message =
            "Cannot constrcut an empty string: it has the type of [char; 0] which is not possible.".to_string();

        Self::new_from_span(message, span)
    }

    pub fn impossible_console_assert_call(span: &Span) -> Self {
        let message = "Console::Assert cannot be matched here, its handled in another case.".to_string();

        Self::new_from_span(message, span)
    }

    pub fn ast_err() -> Self {
	let message = "ahhhh";

	Self::new_from_span(mesage, Span::default())
    }
}
