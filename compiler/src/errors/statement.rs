use crate::errors::{AddressError, BooleanError, ExpressionError, IntegerError, ValueError};
use leo_types::{Error as FormattedError, Span, Type};

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum StatementError {
    #[error("{}", _0)]
    AddressError(#[from] AddressError),

    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    ExpressionError(#[from] ExpressionError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}

impl StatementError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            StatementError::AddressError(error) => error.set_path(path),
            StatementError::BooleanError(error) => error.set_path(path),
            StatementError::Error(error) => error.set_path(path),
            StatementError::ExpressionError(error) => error.set_path(path),
            StatementError::IntegerError(error) => error.set_path(path),
            StatementError::ValueError(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        StatementError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn arguments_type(expected: &Type, actual: &Type, span: Span) -> Self {
        let message = format!("expected return argument type `{}`, found type `{}`", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn array_assign_index(span: Span) -> Self {
        let message = format!("Cannot assign single index to array of values");

        Self::new_from_span(message, span)
    }

    pub fn array_assign_range(span: Span) -> Self {
        let message = format!("Cannot assign range of array values to single value");

        Self::new_from_span(message, span)
    }

    pub fn assertion_failed(left: String, right: String, span: Span) -> Self {
        let message = format!("Assertion `{} == {}` failed", left, right);

        Self::new_from_span(message, span)
    }

    pub fn conditional_boolean(actual: String, span: Span) -> Self {
        let message = format!("If, else conditional must resolve to a boolean, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn immutable_assign(name: String, span: Span) -> Self {
        let message = format!("Cannot assign to immutable variable `{}`", name);

        Self::new_from_span(message, span)
    }

    pub fn immutable_circuit_function(name: String, span: Span) -> Self {
        let message = format!("Cannot mutate circuit function, `{}`", name);

        Self::new_from_span(message, span)
    }

    pub fn indicator_calculation(name: String, span: Span) -> Self {
        let message = format!(
            "Constraint system failed to evaluate branch selection indicator `{}`",
            name
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_number_of_definitions(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Multiple definition statement expected {} return values, found {} values",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_number_of_returns(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Function return statement expected {} return values, found {} values",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    pub fn select_fail(first: String, second: String, span: Span) -> Self {
        let message = format!(
            "Conditional select gadget failed to select between `{}` or `{}`",
            first, second
        );

        Self::new_from_span(message, span)
    }

    pub fn unassigned(name: String, span: Span) -> Self {
        let message = format!("Expected assignment of return values for expression `{}`", name);

        Self::new_from_span(message, span)
    }

    pub fn undefined_variable(name: String, span: Span) -> Self {
        let message = format!("Attempted to assign to unknown variable `{}`", name);

        Self::new_from_span(message, span)
    }

    pub fn undefined_circuit(name: String, span: Span) -> Self {
        let message = format!("Attempted to assign to unknown circuit `{}`", name);

        Self::new_from_span(message, span)
    }

    pub fn undefined_circuit_object(name: String, span: Span) -> Self {
        let message = format!("Attempted to assign to unknown circuit object `{}`", name);

        Self::new_from_span(message, span)
    }
}
