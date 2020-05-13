#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Cannot read from the provided file path - {}", _0)]
    FileReadError(String),

    #[error("Syntax error. Cannot parse the file")]
    FileParsingError,

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,
}
