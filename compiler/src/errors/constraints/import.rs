use leo_ast::ParserError;

use std::{io, path::PathBuf};

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("Attempt to access current directory failed - {:?}", _0)]
    DirectoryError(io::Error),

    #[error("Syntax error. Cannot parse the file")]
    FileParsingError,

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("{}", _0)]
    ParserError(#[from] ParserError),

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,
}
