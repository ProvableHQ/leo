use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("expected integer parameter type, got {}", _0)]
    InvalidType(String),

    #[error("Expected integer parameter, got {}", _0)]
    InvalidInteger(String),

    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
