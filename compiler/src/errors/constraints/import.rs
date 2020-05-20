use std::io;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Attempt to access current directory failed - {:?}", _0)]
    DirectoryError(io::Error),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("Syntax error. Cannot parse the file")]
    FileParsingError,

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,
}
