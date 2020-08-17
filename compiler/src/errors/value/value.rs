use crate::errors::{AddressError, BooleanError, FieldError, GroupError, IntegerError};
use leo_typed::{Error as FormattedError, Span};

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    AddressError(#[from] AddressError),

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
            ValueError::AddressError(error) => error.set_path(path),
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

    pub fn implicit_group(span: Span) -> Self {
        let message = format!("group coordinates should be in (x, y)group format");

        Self::new_from_span(message, span)
    }
}
