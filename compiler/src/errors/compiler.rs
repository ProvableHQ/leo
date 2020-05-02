use std::io;
use std::path::PathBuf;

#[derive(Debug, Fail)]
pub enum CompilerError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),

    #[fail(display = "Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[fail(display = "Cannot parse the file")]
    FileParsingError,

    #[fail(display = "Unable to construct abstract syntax tree")]
    SyntaxTreeError,

    #[fail(display = "writing: {}", _0)]
    Writing(io::Error),
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::Crate("std::io", format!("{}", error))
    }
}
