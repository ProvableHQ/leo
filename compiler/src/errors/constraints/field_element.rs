use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum FieldElementError {
    #[error("Expected field element parameter, got {}", _0)]
    Invalid(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<SynthesisError> for FieldElementError {
    fn from(error: SynthesisError) -> Self {
        FieldElementError::SynthesisError(error)
    }
}
