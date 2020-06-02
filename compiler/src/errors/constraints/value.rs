use crate::errors::{GroupError, IntegerError};

use std::{num::ParseIntError, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    ParseIntError(ParseIntError),

    #[error("{}", _0)]
    ParseBoolError(ParseBoolError),

    #[error("{}", _0)]
    IntegerError(IntegerError),
    /// Unexpected array length
    #[error("{}", _0)]
    ArrayLength(String),

    #[error("Expected type array, got {}", _0)]
    ArrayModel(String),

    /// Unexpected circuit name
    #[error("Expected circuit name {} got {}", _0, _1)]
    CircuitName(String, String),

    /// Unexpected type
    #[error("{}", _0)]
    TypeError(String),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),
}

impl From<ParseIntError> for ValueError {
    fn from(error: ParseIntError) -> Self {
        ValueError::ParseIntError(error)
    }
}

impl From<ParseBoolError> for ValueError {
    fn from(error: ParseBoolError) -> Self {
        ValueError::ParseBoolError(error)
    }
}

impl From<IntegerError> for ValueError {
    fn from(error: IntegerError) -> Self {
        ValueError::IntegerError(error)
    }
}
