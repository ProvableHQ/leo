use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum BooleanError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Expected boolean parameter, got {}", _0)]
    InvalidBoolean(String),

    #[error("Cannot evaluate {}", _0)]
    CannotEvaluate(String),

    #[error("Cannot enforce {}", _0)]
    CannotEnforce(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),
}

impl From<std::io::Error> for BooleanError {
    fn from(error: std::io::Error) -> Self {
        BooleanError::Crate("std::io", format!("{}", error))
    }
}

impl From<SynthesisError> for BooleanError {
    fn from(error: SynthesisError) -> Self {
        BooleanError::SynthesisError(error)
    }
}
