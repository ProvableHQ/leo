use crate::errors::{
    BooleanError, ExpressionError, FieldError, GroupError, IntegerError, StatementError, ValueError,
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
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("Expected function input array, got {}", _0)]
    InvalidArray(String),

    #[error("{}", _0)]
    StatementError(#[from] StatementError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}
