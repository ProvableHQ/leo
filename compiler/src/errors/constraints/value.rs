use crate::errors::IntegerError;

use std::num::ParseIntError;
use std::str::ParseBoolError;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    ArrayLength(String),

    #[error("Expected type array, got {}", _0)]
    ArrayModel(String),

    #[error("Expected circuit name {} got {}", _0, _1)]
    CircuitName(String, String),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    TypeError(String),
}
