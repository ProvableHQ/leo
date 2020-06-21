use crate::{error::Error as FormattedError, Span};
use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl IntegerError {
    fn new_from_span(message: String, span: Span) -> Self {
        IntegerError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the integer operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn cannot_evaluate(operation: String, span: Span) -> Self {
        let message = format!(
            "the integer binary operation `{}` can only be enforced on integers of the same type",
            operation
        );

        Self::new_from_span(message, span)
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
