use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum FieldElementError {
    #[error("Expected field element parameter, got {}", _0)]
    InvalidField(String),

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
