use crate::{ast::Rule, errors::SyntaxError};

use pest::error::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,
}

impl From<Error<Rule>> for ParserError {
    fn from(error: Error<Rule>) -> Self {
        ParserError::SyntaxError(SyntaxError::from(error))
    }
}
