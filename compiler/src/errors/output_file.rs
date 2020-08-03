use std::{io, path::PathBuf};

#[derive(Debug, Error)]
pub enum OutputsFileError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for OutputsFileError {
    fn from(error: std::io::Error) -> Self {
        OutputsFileError::Crate("std::io", format!("{}", error))
    }
}
