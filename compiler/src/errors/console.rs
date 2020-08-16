use crate::errors::ExpressionError;
use leo_typed::{Error as FormattedError, Span};

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ConsoleError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    Expression(#[from] ExpressionError),
}

impl ConsoleError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            ConsoleError::Expression(error) => error.set_path(path),
            ConsoleError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        ConsoleError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn length(containers: usize, parameters: usize, span: Span) -> Self {
        let message = format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        );

        Self::new_from_span(message, span)
    }

    pub fn assertion_depends_on_input(span: Span) -> Self {
        let message = format!("console.assert() failed to evaluate. This error is caused by empty input file values");

        Self::new_from_span(message, span)
    }

    pub fn assertion_failed(expression: String, span: Span) -> Self {
        let message = format!("Assertion `{}` failed", expression);

        Self::new_from_span(message, span)
    }

    pub fn assertion_must_be_boolean(expression: String, span: Span) -> Self {
        let message = format!("Assertion expression `{}` must evaluate to a boolean value", expression);

        Self::new_from_span(message, span)
    }
}
