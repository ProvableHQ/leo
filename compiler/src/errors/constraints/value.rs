use crate::errors::IntegerError;

#[derive(Debug, Error)]
pub enum ValueError {
    /// Unexpected array length
    #[error("{}", _0)]
    ArrayLength(String),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    IntegerError(IntegerError),

    /// Unexpected struct name
    #[error("{}", _0)]
    StructName(String),

    /// Unexpected type
    #[error("{}", _0)]
    TypeError(String),
}

impl From<std::io::Error> for ValueError {
    fn from(error: std::io::Error) -> Self {
        ValueError::Crate("std::io", format!("{}", error))
    }
}

impl From<IntegerError> for ValueError {
    fn from(error: IntegerError) -> Self {
        ValueError::IntegerError(error)
    }
}
