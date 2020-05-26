use crate::errors::{
    BooleanError, FieldElementError, FunctionError, GroupElementError, IntegerError, ValueError,
};

use snarkos_errors::gadgets::SynthesisError;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum ExpressionError {
    // Identifiers
    #[error("Identifier \"{}\" not found", _0)]
    UndefinedIdentifier(String),

    // Types
    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    FieldElementError(#[from] FieldElementError),

    #[error("{}", _0)]
    GroupElementError(#[from] GroupElementError),

    #[error("{}", _0)]
    IncompatibleTypes(String),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("Exponent must be an integer, got field {}", _0)]
    InvalidExponent(String),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("Expected single type for implicit number {}", _0)]
    SingleType(String),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),

    // Arrays
    #[error("Cannot access array {}", _0)]
    InvalidArrayAccess(String),

    #[error("Index must resolve to an integer, got {}", _0)]
    InvalidIndex(String),

    #[error("Expected array length {}, got {}", _0, _1)]
    InvalidLength(usize, usize),

    #[error("Spread should contain an array, got {}", _0)]
    InvalidSpread(String),

    #[error(
        "Array {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedArray(String),

    // Circuits
    #[error("Expected circuit member {}", _0)]
    ExpectedCircuitMember(String),

    #[error("Cannot access circuit {}", _0)]
    InvalidCircuitAccess(String),

    #[error("Non-static member {} must be accessed using `.` syntax", _0)]
    InvalidMemberAccess(String),

    #[error("Static member {} must be accessed using `::` syntax", _0)]
    InvalidStaticAccess(String),

    #[error("Circuit {} has no member {}", _0, _1)]
    UndefinedMemberAccess(String, String),

    #[error("Circuit {} has no static member {}", _0, _1)]
    UndefinedStaticAccess(String, String),

    #[error(
        "Circuit {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedCircuit(String),

    // Functions
    #[error("{}", _0)]
    FunctionError(#[from] Box<FunctionError>),

    #[error("Inline function call to {} did not return", _0)]
    FunctionDidNotReturn(String),

    #[error(
        "Function {} must be declared before it is used in an inline expression",
        _0
    )]
    UndefinedFunction(String),

    // Conditionals
    #[error("If, else conditional must resolve to a boolean, got {}", _0)]
    IfElseConditional(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
