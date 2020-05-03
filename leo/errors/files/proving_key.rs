use std::io;

#[derive(Debug, Fail)]
pub enum ProvingKeyFileError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),

    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for ProvingKeyFileError {
    fn from(error: std::io::Error) -> Self {
        ProvingKeyFileError::Crate("std::io", format!("{}", error))
    }
}
