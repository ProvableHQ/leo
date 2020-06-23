use crate::errors::{BooleanError, Error as FormattedError, FieldError, FunctionError, GroupError, ValueError};
use leo_types::{Identifier, IntegerError, Span};

use snarkos_errors::gadgets::SynthesisError;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum ExpressionError {
    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    FunctionError(#[from] Box<FunctionError>),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}

impl ExpressionError {
    fn new_from_span(message: String, span: Span) -> Self {
        ExpressionError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn conditional_boolean(actual: String, span: Span) -> Self {
        let message = format!("If, else conditional must resolve to a boolean, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn expected_circuit_member(expected: String, span: Span) -> Self {
        let message = format!("Expected circuit member `{}`, not found", expected);

        Self::new_from_span(message, span)
    }

    pub fn unexpected_array(expected: String, actual: String, span: Span) -> Self {
        let message = format!("expected type `{}`, found array with elements `{}`", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn incompatible_types(operation: String, span: Span) -> Self {
        let message = format!("no implementation for `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn invalid_index(actual: String, span: Span) -> Self {
        let message = format!("Index must resolve to an integer, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_length(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!("Expected array length {}, found one with length {}", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_spread(actual: String, span: Span) -> Self {
        let message = format!("Spread should contain an array, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_member_access(member: String, span: Span) -> Self {
        let message = format!("Non-static member `{}` must be accessed using `.` syntax", member);

        Self::new_from_span(message, span)
    }

    pub fn invalid_static_access(member: String, span: Span) -> Self {
        let message = format!("Static member `{}` must be accessed using `::` syntax", member);

        Self::new_from_span(message, span)
    }

    pub fn function_no_return(function: String, span: Span) -> Self {
        let message = format!("Inline function call to `{}` did not return", function);

        Self::new_from_span(message, span)
    }

    pub fn undefined_array(actual: String, span: Span) -> Self {
        let message = format!("Array `{}` must be declared before it is used in an expression", actual);

        Self::new_from_span(message, span)
    }

    pub fn undefined_circuit(actual: String, span: Span) -> Self {
        let message = format!(
            "Circuit `{}` must be declared before it is used in an expression",
            actual
        );

        Self::new_from_span(message, span)
    }

    pub fn undefined_identifier(identifier: Identifier) -> Self {
        let message = format!("cannot find value `{}` in this scope", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    pub fn undefined_function(function: String, span: Span) -> Self {
        let message = format!(
            "Function `{}` must be declared before it is used in an inline expression",
            function
        );

        Self::new_from_span(message, span)
    }

    pub fn undefined_member_access(circuit: String, member: String, span: Span) -> Self {
        let message = format!("Circuit `{}` has no member `{}`", circuit, member);

        Self::new_from_span(message, span)
    }

    pub fn undefined_static_access(circuit: String, member: String, span: Span) -> Self {
        let message = format!("Circuit `{}` has no static member `{}`", circuit, member);

        Self::new_from_span(message, span)
    }
}
