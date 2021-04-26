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

use crate::errors::{AddressError, BooleanError, FieldError, FunctionError, GroupError, IntegerError, ValueError};
use leo_ast::{FormattedError, Identifier, LeoError, Span};
use snarkvm_r1cs::SynthesisError;

#[derive(Debug, Error)]
pub enum ExpressionError {
    #[error("{}", _0)]
    AddressError(#[from] AddressError),

    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    FunctionError(#[from] Box<FunctionError>),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}

impl LeoError for ExpressionError {}

impl ExpressionError {
    fn new_from_span(message: String, span: &Span) -> Self {
        ExpressionError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: &Span) -> Self {
        let message = format!(
            "the gadget operation `{}` failed due to synthesis error `{:?}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn cannot_evaluate(operation: String, span: &Span) -> Self {
        let message = format!("Mismatched types found for operation `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn array_length_out_of_bounds(span: &Span) -> Self {
        let message = "array length cannot be >= 2^32".to_string();

        Self::new_from_span(message, span)
    }

    pub fn array_index_out_of_legal_bounds(span: &Span) -> Self {
        let message = "array index cannot be >= 2^32".to_string();

        Self::new_from_span(message, span)
    }

    pub fn conditional_boolean(actual: String, span: &Span) -> Self {
        let message = format!("if, else conditional must resolve to a boolean, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn expected_circuit_member(expected: String, span: &Span) -> Self {
        let message = format!("expected circuit member `{}`, not found", expected);

        Self::new_from_span(message, span)
    }

    pub fn incompatible_types(operation: String, span: &Span) -> Self {
        let message = format!("no implementation for `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn tuple_index_out_of_bounds(index: usize, span: &Span) -> Self {
        let message = format!("cannot access index {} of tuple out of bounds", index);

        Self::new_from_span(message, span)
    }

    pub fn array_index_out_of_bounds(index: usize, span: &Span) -> Self {
        let message = format!("cannot access index {} of array out of bounds", index);

        Self::new_from_span(message, span)
    }

    pub fn array_invalid_slice_length(span: &Span) -> Self {
        let message = "illegal length of slice".to_string();

        Self::new_from_span(message, span)
    }

    pub fn invalid_index(actual: String, span: &Span) -> Self {
        let message = format!("index must resolve to an integer, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_length(expected: usize, actual: usize, span: &Span) -> Self {
        let message = format!("expected array length {}, found one with length {}", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_static_access(member: String, span: &Span) -> Self {
        let message = format!("static member `{}` must be accessed using `::` syntax", member);

        Self::new_from_span(message, span)
    }

    pub fn undefined_array(actual: String, span: &Span) -> Self {
        let message = format!("array `{}` must be declared before it is used in an expression", actual);

        Self::new_from_span(message, span)
    }

    pub fn undefined_circuit(actual: String, span: &Span) -> Self {
        let message = format!(
            "circuit `{}` must be declared before it is used in an expression",
            actual
        );

        Self::new_from_span(message, span)
    }

    pub fn undefined_identifier(identifier: Identifier) -> Self {
        let message = format!("Cannot find value `{}` in this scope", identifier.name);

        Self::new_from_span(message, &identifier.span)
    }

    pub fn undefined_member_access(circuit: String, member: String, span: &Span) -> Self {
        let message = format!("Circuit `{}` has no member `{}`", circuit, member);

        Self::new_from_span(message, span)
    }
}
