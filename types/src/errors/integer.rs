use snarkos_errors::gadgets::SynthesisError;

use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("Expected integer parameter, got {}", _0)]
    InvalidInteger(String),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
