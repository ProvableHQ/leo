use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("expected integer parameter type, got {}", _0)]
    InvalidType(String),

    #[error("Expected integer {} parameter, got {}", _0, _1)]
    InvalidInteger(String, String),

    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<std::io::Error> for IntegerError {
    fn from(error: std::io::Error) -> Self {
        IntegerError::Crate("std::io", format!("{}", error))
    }
}

impl From<SynthesisError> for IntegerError {
    fn from(error: SynthesisError) -> Self {
        IntegerError::SynthesisError(error)
    }
}
