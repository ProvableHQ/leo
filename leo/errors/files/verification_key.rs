use std::io;
use std::path::PathBuf;

#[derive(Debug, Fail)]
pub enum VerificationKeyFileError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),

    #[fail(display = "Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[fail(display = "Verification key file was corrupted")]
    IncorrectVerificationKey,

    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for VerificationKeyFileError {
    fn from(error: std::io::Error) -> Self {
        VerificationKeyFileError::Crate("std::io", format!("{}", error))
    }
}
