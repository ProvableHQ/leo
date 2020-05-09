use std::io;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum VerificationKeyFileError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("Verification key file was corrupted")]
    IncorrectVerificationKey,

    #[error("writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for VerificationKeyFileError {
    fn from(error: std::io::Error) -> Self {
        VerificationKeyFileError::Crate("std::io", format!("{}", error))
    }
}
