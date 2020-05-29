use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("Expected group element parameter, got {}", _0)]
    InvalidGroup(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<SynthesisError> for GroupError {
    fn from(error: SynthesisError) -> Self {
        GroupError::SynthesisError(error)
    }
}
