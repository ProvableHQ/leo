use std::io;

#[derive(Debug, Error)]
pub enum MainFileError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for MainFileError {
    fn from(error: std::io::Error) -> Self {
        MainFileError::Crate("std::io", format!("{}", error))
    }
}
