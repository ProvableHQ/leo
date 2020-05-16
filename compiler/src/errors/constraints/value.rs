use crate::errors::IntegerError;

#[derive(Debug, Error)]
pub enum ValueError {
    /// Unexpected array length
    #[error("{}", _0)]
    ArrayLength(String),

    #[error("Expected type array, got {}", _0)]
    ArrayModel(String),

    #[error("{}", _0)]
    IntegerError(IntegerError),

    /// Unexpected circuit name
    #[error("Expected circuit name {} got {}", _0, _1)]
    CircuitName(String, String),

    /// Unexpected type
    #[error("{}", _0)]
    TypeError(String),
}

impl From<IntegerError> for ValueError {
    fn from(error: IntegerError) -> Self {
        ValueError::IntegerError(error)
    }
}
