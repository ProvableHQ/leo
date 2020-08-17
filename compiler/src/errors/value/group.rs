use leo_typed::{Error as FormattedError, Span};

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

    pub fn negate_operation(error: SynthesisError, span: Span) -> Self {
        let message = format!("group negation failed due to the synthesis error `{}`", error,);

        Self::new_from_span(message, span)
    }

    pub fn binary_operation(operation: String, error: SynthesisError, span: Span) -> Self {
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

    pub fn synthesis_error(error: SynthesisError, span: Span) -> Self {
        let message = format!("compilation failed due to group synthesis error `{}`", error);

        Self::new_from_span(message, span)
    }

    pub fn x_invalid(x: String, span: Span) -> Self {
        let message = format!("invalid x coordinate `{}`", x);

        Self::new_from_span(message, span)
    }

    pub fn y_invalid(y: String, span: Span) -> Self {
        let message = format!("invalid y coordinate `{}`", y);

        Self::new_from_span(message, span)
    }

    pub fn not_on_curve(element: String, span: Span) -> Self {
        let message = format!("group element `{}` is not on the supported curve", element);

        Self::new_from_span(message, span)
    }

    pub fn x_recover(span: Span) -> Self {
        let message = format!("could not recover group element from x coordinate");

        Self::new_from_span(message, span)
    }

    pub fn y_recover(span: Span) -> Self {
        let message = format!("could not recover group element from y coordinate");

        Self::new_from_span(message, span)
    }
}
