use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum FieldError {
    #[error("Expected field element parameter, got {}", _0)]
    Invalid(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<SynthesisError> for FieldError {
    fn from(error: SynthesisError) -> Self {
        FieldError::SynthesisError(error)
    }
}
