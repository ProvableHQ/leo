use crate::errors::{BooleanError, ExpressionError, FieldError, IntegerError, ValueError};

use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum StatementError {
    #[error("Attempted to assign to unknown variable {}", _0)]
    UndefinedVariable(String),

    #[error("{}", _0)]
    ExpressionError(ExpressionError),

    #[error("{}", _0)]
    IntegerError(IntegerError),

    #[error("{}", _0)]
    FieldError(FieldError),

    #[error("{}", _0)]
    BooleanError(BooleanError),

    #[error("{}", _0)]
    ValueError(ValueError),

    // Arrays
    #[error("Cannot assign single index to array of values")]
    ArrayAssignIndex,

    #[error("Cannot assign range of array values to single value")]
    ArrayAssignRange,

    #[error("Cannot assign to unknown array {}", _0)]
    UndefinedArray(String),

    // Circuits
    #[error("Cannot mutate circuit function, {}", _0)]
    ImmutableCircuitFunction(String),

    #[error("Attempted to assign to unknown circuit {}", _0)]
    UndefinedCircuit(String),

    #[error("Attempted to assign to unknown circuit {}", _0)]
    UndefinedCircuitObject(String),

    // Statements
    #[error("Cannot assign to immutable variable {}", _0)]
    ImmutableAssign(String),

    #[error(
        "Multiple definition statement expected {} return values, got {}",
        _0,
        _1
    )]
    InvalidNumberOfDefinitions(usize, usize),

    #[error("Function return statement expected {} return values, got {}", _0, _1)]
    InvalidNumberOfReturns(usize, usize),

    #[error("If, else conditional must resolve to a boolean, got {}", _0)]
    IfElseConditional(String),

    #[error("Cannot assert equality between {} == {}", _0, _1)]
    AssertEq(String, String),

    #[error("Expected assignment of return values for expression {}", _0)]
    Unassigned(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}

impl From<ExpressionError> for StatementError {
    fn from(error: ExpressionError) -> Self {
        StatementError::ExpressionError(error)
    }
}

impl From<IntegerError> for StatementError {
    fn from(error: IntegerError) -> Self {
        StatementError::IntegerError(error)
    }
}

impl From<FieldError> for StatementError {
    fn from(error: FieldError) -> Self {
        StatementError::FieldError(error)
    }
}

impl From<BooleanError> for StatementError {
    fn from(error: BooleanError) -> Self {
        StatementError::BooleanError(error)
    }
}

impl From<ValueError> for StatementError {
    fn from(error: ValueError) -> Self {
        StatementError::ValueError(error)
    }
}
