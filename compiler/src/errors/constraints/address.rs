use leo_types::{Error as FormattedError, Span};

use snarkos_errors::gadgets::SynthesisError;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum AddressError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl AddressError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            AddressError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        AddressError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the address operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn cannot_evaluate(operation: String, span: Span) -> Self {
        let message = format!("no implementation found for `{}`", operation);

        Self::new_from_span(message, span)
    }

    pub fn invalid_address(actual: String, span: Span) -> Self {
        let message = format!("expected address input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_address(expected: String, span: Span) -> Self {
        let message = format!("expected address input `{}` not found", expected);

        Self::new_from_span(message, span)
    }
}
