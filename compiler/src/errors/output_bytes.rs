use leo_typed::{Error as FormattedError, Span};

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum OutputBytesError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl OutputBytesError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            OutputBytesError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        OutputBytesError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn not_enough_registers(span: Span) -> Self {
        let message = format!("number of input registers must be greater than or equal to output registers");

        Self::new_from_span(message, span)
    }
}
