use leo_types::{Error as FormattedError, Span};

use snarkos_errors::gadgets::SynthesisError;

use std::str::ParseBoolError;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}

impl BooleanError {
    fn new_from_span(message: String, span: Span) -> Self {
        BooleanError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the boolean operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn cannot_evaluate(operation: String, span: Span) -> Self {
        let message = format!("no implementation found for `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn invalid_boolean(actual: String, span: Span) -> Self {
        let message = format!("expected boolean input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_boolean(expected: String, span: Span) -> Self {
        let message = format!("expected boolean input `{}` not found", expected);

        Self::new_from_span(message, span)
    }
}
