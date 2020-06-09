use crate::{ast::Rule, errors::SyntaxError};

use pest::error::Error;
use std::{path::PathBuf, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum InputParserError {
    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,
}

impl From<Error<Rule>> for InputParserError {
    fn from(error: Error<Rule>) -> Self {
        InputParserError::SyntaxError(SyntaxError::from(error))
    }
}
