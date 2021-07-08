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
use snarkvm_r1cs::SynthesisError;

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("Failed to represent group as log string.")]
    LogError,
}

impl LeoError for GroupError {}

impl GroupError {
    fn new_from_span(message: String, span: &Span) -> Self {
        GroupError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn failed_to_create_log_string() -> Self {
        GroupError::LogError
    }

    pub fn negate_operation(error: SynthesisError, span: &Span) -> Self {
        let message = format!("group negation failed due to the synthesis error `{:?}`", error,);

        Self::new_from_span(message, span)
    }

    pub fn binary_operation(operation: String, error: SynthesisError, span: &Span) -> Self {
        let message = format!(
            "the group binary operation `{}` failed due to the synthesis error `{:?}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_group(actual: String, span: &Span) -> Self {
        let message = format!("expected group affine point input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_group(expected: String, span: &Span) -> Self {
        let message = format!("expected group input `{}` not found", expected);

        Self::new_from_span(message, span)
    }

    pub fn synthesis_error(error: SynthesisError, span: &Span) -> Self {
        let message = format!("compilation failed due to group synthesis error `{:?}`", error);

        Self::new_from_span(message, span)
    }

    pub fn x_invalid(x: String, span: &Span) -> Self {
        let message = format!("invalid x coordinate `{}`", x);

        Self::new_from_span(message, span)
    }

    pub fn y_invalid(y: String, span: &Span) -> Self {
        let message = format!("invalid y coordinate `{}`", y);

        Self::new_from_span(message, span)
    }

    pub fn not_on_curve(element: String, span: &Span) -> Self {
        let message = format!("group element `{}` is not on the supported curve", element);

        Self::new_from_span(message, span)
    }

    pub fn x_recover(span: &Span) -> Self {
        let message = "could not recover group element from x coordinate".to_string();

        Self::new_from_span(message, span)
    }

    pub fn y_recover(span: &Span) -> Self {
        let message = "could not recover group element from y coordinate".to_string();

        Self::new_from_span(message, span)
    }

    pub fn n_group(number: String, span: &Span) -> Self {
        let message = format!("cannot multiply group generator by \"{}\"", number);

        Self::new_from_span(message, span)
    }
}
