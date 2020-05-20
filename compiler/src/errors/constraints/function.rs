use crate::errors::{
    BooleanError, ExpressionError, FieldElementError, IntegerError, StatementError, ValueError,
};

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("Function expected {} inputs, got {}", _0, _1)]
    ArgumentsLength(usize, usize),

    #[error("Function input type not defined {}", _0)]
    UndefinedInput(String),

    #[error("Function expected input type {}, got {}", _0, _1)]
    InvalidInput(String, String),

    #[error("Expected function input array, got {}", _0)]
    InvalidArray(String),

    #[error("Expected function input array length {}, got length {}", _0, _1)]
    InvalidArrayLength(usize, usize),

    #[error("{}", _0)]
    ValueError(ValueError),

    #[error("{}", _0)]
    IntegerError(IntegerError),

    #[error("{}", _0)]
    FieldElementError(FieldElementError),

    #[error("{}", _0)]
    BooleanError(BooleanError),

    #[error("{}", _0)]
    ExpressionError(ExpressionError),

    #[error("{}", _0)]
    StatementError(StatementError),
}

impl From<ValueError> for FunctionError {
    fn from(error: ValueError) -> Self {
        FunctionError::ValueError(error)
    }
}

impl From<IntegerError> for FunctionError {
    fn from(error: IntegerError) -> Self {
        FunctionError::IntegerError(error)
    }
}

impl From<FieldElementError> for FunctionError {
    fn from(error: FieldElementError) -> Self {
        FunctionError::FieldElementError(error)
    }
}

impl From<BooleanError> for FunctionError {
    fn from(error: BooleanError) -> Self {
        FunctionError::BooleanError(error)
    }
}

impl From<ExpressionError> for FunctionError {
    fn from(error: ExpressionError) -> Self {
        FunctionError::ExpressionError(error)
    }
}

impl From<StatementError> for FunctionError {
    fn from(error: StatementError) -> Self {
        FunctionError::StatementError(error)
    }
}
