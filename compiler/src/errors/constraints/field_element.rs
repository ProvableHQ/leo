use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum FieldElementError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Expected field element parameter, got {}", _0)]
    InvalidField(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<std::io::Error> for FieldElementError {
    fn from(error: std::io::Error) -> Self {
        FieldElementError::Crate("std::io", format!("{}", error))
    }
}

impl From<SynthesisError> for FieldElementError {
    fn from(error: SynthesisError) -> Self {
        FieldElementError::SynthesisError(error)
    }
}
