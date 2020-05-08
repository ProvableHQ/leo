#[derive(Debug, Error)]
pub enum ImportError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Cannot read from the provided file path - {}", _0)]
    FileReadError(String),

    #[error("Syntax error. Cannot parse the file")]
    FileParsingError,

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,
}

impl From<std::io::Error> for ImportError {
    fn from(error: std::io::Error) -> Self {
        ImportError::Crate("std::io", format!("{}", error))
    }
}
