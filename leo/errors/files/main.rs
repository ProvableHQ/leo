use std::io;

#[derive(Debug, Fail)]
pub enum MainFileError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),

    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for MainFileError {
    fn from(error: std::io::Error) -> Self {
        MainFileError::Crate("std::io", format!("{}", error))
    }
}
