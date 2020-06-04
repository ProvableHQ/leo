use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum FieldError {
    #[error("Expected field element parameter, got {}", _0)]
    Invalid(String),

    #[error("No multiplicative inverse found for field {}", _0)]
    NoInverse(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
