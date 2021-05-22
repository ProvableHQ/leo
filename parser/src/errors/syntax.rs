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

use crate::{DeprecatedError, SyntaxResult, Token, TokenError};

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    TokenError(#[from] TokenError),

    #[error("{}", _0)]
    DeprecatedError(#[from] DeprecatedError),
}

impl LeoError for SyntaxError {}

pub fn assert_no_whitespace(left_span: &Span, right_span: &Span, left: &str, right: &str) -> SyntaxResult<()> {
    if left_span.col_stop != right_span.col_start {
        let mut error_span = left_span + right_span;
        error_span.col_start = left_span.col_stop - 1;
        error_span.col_stop = right_span.col_start - 1;
        return Err(SyntaxError::unexpected_whitespace(left, right, &error_span));
    }

    Ok(())
}

impl SyntaxError {
    fn new_from_span(message: String, span: &Span) -> Self {
        SyntaxError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn invalid_import_list(span: &Span) -> Self {
        Self::new_from_span("Cannot import empty list".to_string(), span)
    }

    pub fn unexpected_eof(span: &Span) -> Self {
        Self::new_from_span("unexpected EOF".to_string(), span)
    }

    pub fn unexpected_whitespace(left: &str, right: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("Unexpected white space between terms {} and {}", left, right),
            span,
        )
    }

    pub fn unexpected(got: &Token, expected: &[Token], span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "expected {} -- got '{}'",
                expected
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(", "),
                got.to_string()
            ),
            span,
        )
    }

    pub fn mixed_commas_and_semicolons(span: &Span) -> Self {
        Self::new_from_span(
            "Cannot mix use of commas and semi-colons for circuit member variable declarations.".to_string(),
            span,
        )
    }

    pub fn unexpected_ident(got: &str, expected: &[&str], span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "expected identifier {} -- got '{}'",
                expected
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(", "),
                got
            ),
            span,
        )
    }

    pub fn unexpected_str(got: &Token, expected: &str, span: &Span) -> Self {
        Self::new_from_span(format!("expected '{}', got '{}'", expected, got.to_string()), span)
    }

    pub fn spread_in_array_init(span: &Span) -> Self {
        Self::new_from_span("illegal spread in array initializer".to_string(), span)
    }

    pub fn invalid_assignment_target(span: &Span) -> Self {
        Self::new_from_span("invalid assignment target".to_string(), span)
    }

    pub fn invalid_package_name(span: &Span) -> Self {
        Self::new_from_span(
            "package names must be lowercase alphanumeric ascii with underscores and singular dashes".to_string(),
            span,
        )
    }

    pub fn illegal_self_const(span: &Span) -> Self {
        Self::new_from_span("cannot have const self".to_string(), span)
    }
}
