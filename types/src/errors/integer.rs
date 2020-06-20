use crate::{error::Error as FormattedError, Span};
use snarkos_errors::gadgets::SynthesisError;

use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("Expected integer parameter, got {}", _0)]
    InvalidInteger(String),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}

impl IntegerError {
    fn new_from_span(message: String, span: Span) -> Self {
        IntegerError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn invalid_integer(actual: String, span: Span) -> Self {
        let message = format!("expected integer input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_integer(expected: String, span: Span) -> Self {
        let message = format!("expected integer input `{}` not found", expected);

        Self::new_from_span(message, span)
    }
}
