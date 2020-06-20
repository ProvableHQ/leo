use crate::errors::{BooleanError, Error, FieldError, FunctionError, GroupError, ValueError};
use leo_types::{Identifier, IntegerError, Span};

use snarkos_errors::gadgets::SynthesisError;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum ExpressionError {
    #[error("{}", _0)]
    Error(#[from] Error),

    // Types
    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    IncompatibleTypes(String),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

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

    #[error("Array {} must be declared before it is used in an inline expression", _0)]
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

    #[error("Circuit {} must be declared before it is used in an inline expression", _0)]
    UndefinedCircuit(String),

    #[error("Circuit {} has no member {}", _0, _1)]
    UndefinedMemberAccess(String, String),

    #[error("Circuit {} has no static member {}", _0, _1)]
    UndefinedStaticAccess(String, String),

    // Functions
    #[error("Inline function call to {} did not return", _0)]
    FunctionDidNotReturn(String),

    #[error("{}", _0)]
    FunctionError(#[from] Box<FunctionError>),

    #[error("Function {} must be declared before it is used in an inline expression", _0)]
    UndefinedFunction(String),

    // Conditionals
    #[error("If, else statements.conditional must resolve to a boolean, got {}", _0)]
    IfElseConditional(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}

impl ExpressionError {
    fn new_from_span(message: String, span: Span) -> Self {
        ExpressionError::Error(Error::new_from_span(message, span))
    }

    pub fn undefined_identifier(identifier: Identifier) -> Self {
        let message = format!("cannot find value `{}` in this scope", identifier.name);

        Self::new_from_span(message, identifier.span)
    }
}
