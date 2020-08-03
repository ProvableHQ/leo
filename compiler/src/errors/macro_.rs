use crate::errors::ExpressionError;
use leo_typed::{Error as FormattedError, Span};

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum MacroError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    Expression(#[from] ExpressionError),
}

impl MacroError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            MacroError::Expression(error) => error.set_path(path),
            MacroError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        MacroError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn length(containers: usize, parameters: usize, span: Span) -> Self {
        let message = format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        );

        Self::new_from_span(message, span)
    }
}
