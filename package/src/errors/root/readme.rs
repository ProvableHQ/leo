use std::io;

#[derive(Debug, Error)]
pub enum READMEError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for READMEError {
    fn from(error: std::io::Error) -> Self {
        READMEError::Crate("std::io", format!("{}", error))
    }
}
