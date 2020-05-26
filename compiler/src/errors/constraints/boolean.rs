use crate::errors::ValueError;
use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Expected boolean parameter, got {}", _0)]
    InvalidBoolean(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}
