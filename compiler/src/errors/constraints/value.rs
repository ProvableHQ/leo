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

    /// Unexpected struct name
    #[error("{}", _0)]
    StructName(String),

    /// Unexpected type
    #[error("{}", _0)]
    TypeError(String),
}

impl From<IntegerError> for ValueError {
    fn from(error: IntegerError) -> Self {
        ValueError::IntegerError(error)
    }
}
