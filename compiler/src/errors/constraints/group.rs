use leo_types::{Error as FormattedError, Span};

use snarkos_errors::gadgets::SynthesisError;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl GroupError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            GroupError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        GroupError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn cannot_enforce(operation: String, error: SynthesisError, span: Span) -> Self {
        let message = format!(
            "the group binary operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        );

        Self::new_from_span(message, span)
    }

    pub fn invalid_group(actual: String, span: Span) -> Self {
        let message = format!("expected group affine point input type, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn missing_group(expected: String, span: Span) -> Self {
        let message = format!("expected group input `{}` not found", expected);

        Self::new_from_span(message, span)
    }
}
