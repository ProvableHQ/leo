use crate::errors::{BooleanError, FieldError, GroupError, IntegerError};
use leo_types::{Error as FormattedError, Span};
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),
}

impl ValueError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            ValueError::BooleanError(error) => error.set_path(path),
            ValueError::Error(error) => error.set_path(path),
            ValueError::FieldError(error) => error.set_path(path),
            ValueError::GroupError(error) => error.set_path(path),
            ValueError::IntegerError(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        ValueError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn implicit(value: String, span: Span) -> Self {
        let message = format!("explicit type needed for `{}`", value);

        Self::new_from_span(message, span)
    }
}
