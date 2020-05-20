use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum GroupElementError {
    #[error("Expected group element parameter, got {}", _0)]
    InvalidGroup(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<SynthesisError> for GroupElementError {
    fn from(error: SynthesisError) -> Self {
        GroupElementError::SynthesisError(error)
    }
}
