use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("Expected boolean parameter, got {}", _0)]
    InvalidBoolean(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
