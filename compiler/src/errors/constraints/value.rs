use crate::errors::{BooleanError, FieldError, GroupError};
use leo_types::{Error as FormattedError, IntegerError, Span};

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
    fn new_from_span(message: String, span: Span) -> Self {
        ValueError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn implicit(value: String, span: Span) -> Self {
        let message = format!("explicit type needed for `{}`", value);

        Self::new_from_span(message, span)
    }
}
