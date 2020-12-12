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

use crate::errors::{AddressError, BooleanError, FieldError, FunctionError, GroupError, IntegerError, ValueError};
use leo_ast::{ArrayDimensions, Error as FormattedError, Identifier, PositiveNumber, Span};
use leo_core::LeoCorePackageError;

use snarkos_errors::gadgets::SynthesisError;
use std::path::Path;

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
    LeoCoreError(#[from] LeoCorePackageError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}

impl ExpressionError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            ExpressionError::AddressError(error) => error.set_path(path),
            ExpressionError::BooleanError(error) => error.set_path(path),
            ExpressionError::Error(error) => error.set_path(path),
            ExpressionError::FieldError(error) => error.set_path(path),
            ExpressionError::FunctionError(error) => error.set_path(path),
            ExpressionError::GroupError(error) => error.set_path(path),
            ExpressionError::IntegerError(error) => error.set_path(path),
            ExpressionError::LeoCoreError(error) => error.set_path(path),
            ExpressionError::ValueError(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        ExpressionError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the gadget operation `{}` failed due to synthesis error `{:?}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn cannot_evaluate(operation: String, span: Span) -> Self {
        let message = format!("Mismatched types found for operation `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn conditional_boolean(actual: String, span: Span) -> Self {
        let message = format!("if, else conditional must resolve to a boolean, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn expected_circuit_member(expected: String, span: Span) -> Self {
        let message = format!("expected circuit member `{}`, not found", expected);

        Self::new_from_span(message, span)
    }

    pub fn incompatible_types(operation: String, span: Span) -> Self {
        let message = format!("no implementation for `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn index_out_of_bounds(index: usize, span: Span) -> Self {
        let message = format!("cannot access index {} of tuple out of bounds", index);

        Self::new_from_span(message, span)
    }

    pub fn invalid_dimensions(expected: &ArrayDimensions, actual: &ArrayDimensions, span: Span) -> Self {
        let message = format!(
            "expected array dimensions {}, found array dimensions {}",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_first_dimension(expected: &PositiveNumber, actual: &PositiveNumber, span: Span) -> Self {
        let message = format!(
            "expected array dimension {}, found array dimension {}",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_index(actual: String, span: &Span) -> Self {
        let message = format!("index must resolve to an integer, found `{}`", actual);

        Self::new_from_span(message, span.to_owned())
    }

    pub fn invalid_length(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!("expected array length {}, found one with length {}", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_spread(actual: String, span: Span) -> Self {
        let message = format!("spread should contain an array, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_member_access(member: String, span: Span) -> Self {
        let message = format!("non-static member `{}` must be accessed using `.` syntax", member);

        Self::new_from_span(message, span)
    }

    pub fn invalid_static_access(member: String, span: Span) -> Self {
        let message = format!("static member `{}` must be accessed using `::` syntax", member);

        Self::new_from_span(message, span)
    }

    pub fn function_no_return(function: String, span: Span) -> Self {
        let message = format!("inline function call to `{}` did not return", function);

        Self::new_from_span(message, span)
    }

    pub fn self_keyword(span: Span) -> Self {
        let message = "cannot call keyword `Self` outside of a circuit function".to_string();

        Self::new_from_span(message, span)
    }

    pub fn undefined_array(actual: String, span: Span) -> Self {
        let message = format!("array `{}` must be declared before it is used in an expression", actual);

        Self::new_from_span(message, span)
    }

    pub fn undefined_tuple(actual: String, span: Span) -> Self {
        let message = format!("tuple `{}` must be declared before it is used in an expression", actual);

        Self::new_from_span(message, span)
    }

    pub fn undefined_circuit(actual: String, span: Span) -> Self {
        let message = format!(
            "circuit `{}` must be declared before it is used in an expression",
            actual
        );

        Self::new_from_span(message, span)
    }

    pub fn undefined_first_dimension(span: Span) -> Self {
        let message = "the first dimension of the array must be a number".to_string();

        Self::new_from_span(message, span)
    }

    pub fn undefined_function(function: String, span: Span) -> Self {
        let message = format!(
            "function `{}` must be declared before it is used in an inline expression",
            function
        );

        Self::new_from_span(message, span)
    }

    pub fn undefined_identifier(identifier: Identifier) -> Self {
        let message = format!("Cannot find value `{}` in this scope", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    pub fn undefined_member_access(circuit: String, member: String, span: Span) -> Self {
        let message = format!("Circuit `{}` has no member `{}`", circuit, member);

        Self::new_from_span(message, span)
    }

    pub fn undefined_static_access(circuit: String, member: String, span: Span) -> Self {
        let message = format!("Circuit `{}` has no static member `{}`", circuit, member);

        Self::new_from_span(message, span)
    }

    pub fn unexpected_array(expected: String, span: Span) -> Self {
        let message = format!("expected type `{}`, found array with elements", expected);

        Self::new_from_span(message, span)
    }

    pub fn unexpected_tuple(expected: String, actual: String, span: Span) -> Self {
        let message = format!("expected type `{}`, found tuple with values `{}`", expected, actual);

        Self::new_from_span(message, span)
    }
}
