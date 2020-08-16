use crate::{ast::Rule, errors::SyntaxError};

use pest::error::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("{}", _0)]
    JsonError(#[from] serde_json::error::Error),

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("Unable to construct program abstract syntax tree")]
    SyntaxTreeError,
}

impl From<Error<Rule>> for ParserError {
    fn from(error: Error<Rule>) -> Self {
        ParserError::SyntaxError(SyntaxError::from(error))
    }
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::Crate("std::io", format!("{}", error))
    }
}
