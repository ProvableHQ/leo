use crate::errors::ValueError;

use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("{}", _0)]
    ValueError(ValueError),

    #[error("Expected boolean parameter, got {}", _0)]
    InvalidBoolean(String),

    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<SynthesisError> for BooleanError {
    fn from(error: SynthesisError) -> Self {
        BooleanError::SynthesisError(error)
    }
}

impl From<ValueError> for BooleanError {
    fn from(error: ValueError) -> Self {
        BooleanError::ValueError(error)
    }
}
