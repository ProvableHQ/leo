use crate::errors::{BooleanError, ExpressionError, ValueError};

use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum StatementError {
    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    ExpressionError(#[from] ExpressionError),

    #[error("Attempted to assign to unknown variable {}", _0)]
    UndefinedVariable(String),

    // Arrays
    #[error("Cannot assign single index to array of values")]
    ArrayAssignIndex,

    #[error("Cannot assign range of array values to single value")]
    ArrayAssignRange,

    // Circuits
    #[error("Cannot mutate circuit function, {}", _0)]
    ImmutableCircuitFunction(String),

    #[error("Attempted to assign to unknown circuit {}", _0)]
    UndefinedCircuit(String),

    #[error("Attempted to assign to unknown circuit {}", _0)]
    UndefinedCircuitObject(String),

    // Statements
    #[error("Cannot assert equality between {} == {}", _0, _1)]
    AssertEq(String, String),

    #[error("Assertion {:?} == {:?} failed", _0, _1)]
    AssertionFailed(String, String),

    #[error("If, else statements.conditional must resolve to a boolean, got {}", _0)]
    IfElseConditional(String),

    #[error("Cannot assign to immutable variable {}", _0)]
    ImmutableAssign(String),

    #[error("Multiple definition statement expected {} return values, got {}", _0, _1)]
    InvalidNumberOfDefinitions(usize, usize),

    #[error("Function return statement expected {} return values, got {}", _0, _1)]
    InvalidNumberOfReturns(usize, usize),

    #[error("Conditional select gadget failed to select between {} or {}", _0, _1)]
    SelectFail(String, String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),

    #[error("Expected assignment of return values for expression {}", _0)]
    Unassigned(String),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}
