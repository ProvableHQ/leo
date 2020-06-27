use std::io;

#[derive(Debug, Error)]
pub enum LibFileError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for LibFileError {
    fn from(error: std::io::Error) -> Self {
        LibFileError::Crate("std::io", format!("{}", error))
    }
}
