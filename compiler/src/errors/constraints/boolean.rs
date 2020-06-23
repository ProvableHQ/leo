use snarkos_errors::gadgets::SynthesisError;

use std::str::ParseBoolError;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
