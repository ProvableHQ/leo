use crate::errors::{
    BooleanError, FieldElementError, FunctionError, GroupError, IntegerError, ValueError,
};

use snarkos_errors::gadgets::SynthesisError;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum ExpressionError {
    // Identifiers
    #[error("Identifier \"{}\" not found", _0)]
    UndefinedIdentifier(String),

    // Types
    #[error("Expected single type for implicit number {}", _0)]
    SingleType(String),

    #[error("{}", _0)]
    IncompatibleTypes(String),

    #[error("{}", _0)]
    ValueError(ValueError),

    #[error("{}", _0)]
    IntegerError(IntegerError),

    #[error("{}", _0)]
    ParseIntError(ParseIntError),

    #[error("{}", _0)]
    FieldElementError(FieldElementError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    BooleanError(BooleanError),

    #[error("Exponent must be an integer, got field {}", _0)]
    InvalidExponent(String),

    // Arrays
    #[error(
        "Array {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedArray(String),

    #[error("Cannot access array {}", _0)]
    InvalidArrayAccess(String),

    #[error("Spread should contain an array, got {}", _0)]
    InvalidSpread(String),

    #[error("Index must resolve to an integer, got {}", _0)]
    InvalidIndex(String),

    #[error("Expected array length {}, got {}", _0, _1)]
    InvalidLength(usize, usize),

    // Circuits
    #[error(
        "Circuit {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedCircuit(String),

    #[error("Cannot access circuit {}", _0)]
    InvalidCircuitAccess(String),

    #[error("Expected circuit member {}", _0)]
    ExpectedCircuitMember(String),

    #[error("Circuit {} has no member {}", _0, _1)]
    UndefinedMemberAccess(String, String),

    #[error("Non-static member {} must be accessed using `.` syntax", _0)]
    InvalidMemberAccess(String),

    #[error("Circuit {} has no static member {}", _0, _1)]
    UndefinedStaticAccess(String, String),

    #[error("Static member {} must be accessed using `::` syntax", _0)]
    InvalidStaticAccess(String),

    // Functions
    #[error(
        "Function {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedFunction(String),

    #[error("{}", _0)]
    FunctionError(Box<FunctionError>),

    #[error("Inline function call to {} did not return", _0)]
    FunctionDidNotReturn(String),

    // Conditionals
    #[error("If, else conditional must resolve to a boolean, got {}", _0)]
    IfElseConditional(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<SynthesisError> for ExpressionError {
    fn from(error: SynthesisError) -> Self {
        ExpressionError::SynthesisError(error)
    }
}

impl From<ValueError> for ExpressionError {
    fn from(error: ValueError) -> Self {
        ExpressionError::ValueError(error)
    }
}

impl From<IntegerError> for ExpressionError {
    fn from(error: IntegerError) -> Self {
        ExpressionError::IntegerError(error)
    }
}

impl From<ParseIntError> for ExpressionError {
    fn from(error: ParseIntError) -> Self {
        ExpressionError::ParseIntError(error)
    }
}

impl From<FieldElementError> for ExpressionError {
    fn from(error: FieldElementError) -> Self {
        ExpressionError::FieldElementError(error)
    }
}

impl From<BooleanError> for ExpressionError {
    fn from(error: BooleanError) -> Self {
        ExpressionError::BooleanError(error)
    }
}

impl From<Box<FunctionError>> for ExpressionError {
    fn from(error: Box<FunctionError>) -> Self {
        ExpressionError::FunctionError(error)
    }
}
