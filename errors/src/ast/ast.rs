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

use crate::{ErrorCode, FormattedError, LeoErrorCode, Span};

#[derive(Debug, Error)]
pub enum AstError {
    #[error(transparent)]
    FormattedError(#[from] FormattedError),
}

impl LeoErrorCode for AstError {}

impl ErrorCode for AstError {
    #[inline(always)]
    fn exit_code_mask() -> u32 {
        1000
    }

    #[inline(always)]
    fn error_type() -> String {
        "T".to_string()
    }

    fn new_from_span(message: String, help: Option<String>, exit_code: u32, span: &Span) -> Self {
        Self::FormattedError(FormattedError::new_from_span(
            message,
            help,
            exit_code ^ Self::exit_code_mask(),
            Self::code_identifier(),
            Self::error_type(),
            span,
        ))
    }
}

impl AstError {
    pub fn big_self_outside_of_circuit(span: &Span) -> Self {
        let message = "cannot call keyword `Self` outside of a circuit function".to_string();

        Self::new_from_span(message, None, 1, span)
    }

    pub fn invalid_array_dimension_size(span: &Span) -> Self {
        let message = "received dimension size of 0, expected it to be 1 or larger.".to_string();

        Self::new_from_span(message, None, 2, span)
    }

    pub fn asg_statement_not_block(span: &Span) -> Self {
        let message = "AstStatement should be be a block".to_string();

        Self::new_from_span(message, None, 3, span)
    }

    pub fn empty_string(span: &Span) -> Self {
        let message =
            "Cannot constrcut an empty string: it has the type of [char; 0] which is not possible.".to_string();

        Self::new_from_span(message, None, 5, span)
    }

    pub fn impossible_console_assert_call(span: &Span) -> Self {
        let message = "Console::Assert cannot be matched here, its handled in another case.".to_string();

        Self::new_from_span(message, None, 6, span)
    }
}
