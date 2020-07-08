use leo_types::{Error as FormattedError, Span};

use snarkos_errors::gadgets::SynthesisError;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum FieldError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl FieldError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            FieldError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        FieldError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the field binary operation `{}` failed due to synthesis error `{}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_field(actual: String, span: Span) -> Self {
        let message = format!("expected field element input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_field(expected: String, span: Span) -> Self {
        let message = format!("expected field input `{}` not found", expected);

        Self::new_from_span(message, span)
    }

    pub fn no_inverse(field: String, span: Span) -> Self {
        let message = format!("no multiplicative inverse found for field `{}`", field);

        Self::new_from_span(message, span)
    }

    pub fn synthesis_error(error: SynthesisError, span: Span) -> Self {
        let message = format!("compilation failed due to field synthesis error `{}`", error);

        Self::new_from_span(message, span)
    }
}
