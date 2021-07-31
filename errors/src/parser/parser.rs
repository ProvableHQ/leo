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
pub enum ParserError {
    #[error(transparent)]
    FormattedError(#[from] FormattedError),
}

impl LeoErrorCode for ParserError {}

impl ErrorCode for ParserError {
    #[inline(always)]
    fn exit_code_mask() -> u32 {
        0
    }

    #[inline(always)]
    fn error_type() -> String {
        "P".to_string()
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

impl ParserError {
    pub fn unexpected_token(message: String, help: String, span: &Span) -> Self {
        Self::new_from_span(message, Some(help), 0, span)
    }

    pub fn invalid_address_lit(token: &str, span: &Span) -> Self {
        Self::new_from_span(format!("invalid address literal: '{}'", token), None, 1, span)
    }

    pub fn invalid_import_list(span: &Span) -> Self {
        Self::new_from_span("Cannot import empty list".to_string(), None, 2, span)
    }

    pub fn unexpected_eof(span: &Span) -> Self {
        Self::new_from_span("unexpected EOF".to_string(), None, 3, span)
    }

    pub fn unexpected_whitespace(left: &str, right: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("Unexpected white space between terms {} and {}", left, right),
            None,
            4,
            span,
        )
    }

    pub fn unexpected(got: String, expected: String, span: &Span) -> Self {
        Self::new_from_span(format!("expected {} -- got '{}'", expected, got), None, 5, span)
    }

    pub fn mixed_commas_and_semicolons(span: &Span) -> Self {
        Self::new_from_span(
            "Cannot mix use of commas and semi-colons for circuit member variable declarations.".to_string(),
            None,
            6,
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
            None,
            7,
            span,
        )
    }

    pub fn unexpected_statement(got: String, expected: &str, span: &Span) -> Self {
        Self::new_from_span(format!("expected '{}', got '{}'", expected, got), None, 8, span)
    }

    pub fn unexpected_str(got: String, expected: &str, span: &Span) -> Self {
        Self::new_from_span(format!("expected '{}', got '{}'", expected, got), None, 9, span)
    }

    pub fn spread_in_array_init(span: &Span) -> Self {
        Self::new_from_span("illegal spread in array initializer".to_string(), None, 10, span)
    }

    pub fn invalid_assignment_target(span: &Span) -> Self {
        Self::new_from_span("invalid assignment target".to_string(), None, 11, span)
    }

    pub fn invalid_package_name(span: &Span) -> Self {
        Self::new_from_span(
            "package names must be lowercase alphanumeric ascii with underscores and singular dashes".to_string(),
            None,
            12,
            span,
        )
    }

    pub fn illegal_self_const(span: &Span) -> Self {
        Self::new_from_span("cannot have const self".to_string(), None, 13, span)
    }

    pub fn mut_function_input(mut span: Span) -> Self {
        let message =
            "function func(mut a: u32) { ... } is deprecated. Passed variables are mutable by default.".to_string();
        span.col_start -= 1;
        span.col_stop -= 1;
        Self::new_from_span(message, None, 14, &span)
    }

    pub fn let_mut_statement(mut span: Span) -> Self {
        let message = "let mut = ... is deprecated. `let` keyword implies mutabality by default.".to_string();
        span.col_start -= 1;
        span.col_stop -= 1;
        Self::new_from_span(message, None, 15, &span)
    }

    pub fn test_function(span: &Span) -> Self {
        let message = "\"test function...\" is deprecated. Did you mean @test annotation?".to_string();
        Self::new_from_span(message, None, 16, span)
    }

    pub fn context_annotation(span: &Span) -> Self {
        let message = "\"@context(...)\" is deprecated. Did you mean @test annotation?".to_string();
        Self::new_from_span(message, None, 17, span)
    }
}
