use crate::ast::Rule;
use crate::errors::{FunctionError, ImportError, IntegerError, SyntaxError};

use pest::error::Error;
use std::{io, path::PathBuf};

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("Attempt to access current directory failed - {:?}", _0)]
    DirectoryError(io::Error),

    #[error("{}", _0)]
    ImportError(#[from] ImportError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    FunctionError(#[from] FunctionError),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("Syntax error. Cannot parse the file")]
    FileParsingError,

    #[error("Main function not found")]
    NoMain,

    #[error("Main must be a function")]
    NoMainFunction,

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,

    #[error("writing: {}", _0)]
    Writing(io::Error),
}

impl From<Error<Rule>> for CompilerError {
    fn from(error: Error<Rule>) -> Self {
        CompilerError::SyntaxError(SyntaxError::from(error))
    }
}
