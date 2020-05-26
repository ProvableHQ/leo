use crate::errors::{
    BooleanError, ExpressionError, FieldElementError, GroupElementError, IntegerError,
    StatementError, ValueError,
};

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("Function expected {} inputs, got {}", _0, _1)]
    ArgumentsLength(usize, usize),

    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    ExpressionError(#[from] ExpressionError),

    #[error("{}", _0)]
    FieldElementError(#[from] FieldElementError),

    #[error("{}", _0)]
    GroupElementError(#[from] GroupElementError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("Expected function input array, got {}", _0)]
    InvalidArray(String),

    #[error("Expected function input array length {}, got length {}", _0, _1)]
    InvalidArrayLength(usize, usize),

    #[error("Function expected input type {}, got {}", _0, _1)]
    InvalidInput(String, String),

    #[error("{}", _0)]
    StatementError(#[from] StatementError),

    #[error("Function input type not defined {}", _0)]
    UndefinedInput(String),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}
