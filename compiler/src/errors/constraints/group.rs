use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("Expected affine point, got {}", _0)]
    Invalid(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<SynthesisError> for GroupError {
    fn from(error: SynthesisError) -> Self {
        GroupError::SynthesisError(error)
    }
}
