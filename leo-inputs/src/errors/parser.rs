use crate::{ast::Rule, errors::SyntaxError};

use pest::error::Error;
use std::{num::ParseIntError, path::PathBuf, str::ParseBoolError};

#[derive(Debug, Error)]
pub enum InputParserError {
    #[error("expected array length {}, got {}", _0, _1)]
    InvalidArrayLength(usize, usize),

    #[error("expected type {}, got {}", _0, _1)]
    IncompatibleTypes(String, String),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("{}", _0)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("{}", _0)]
    ParseIntError(#[from] ParseIntError),

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,

    #[error("found an empty array dimension in type")]
    UndefinedArrayDimension,
}

impl From<Error<Rule>> for InputParserError {
    fn from(error: Error<Rule>) -> Self {
        InputParserError::SyntaxError(SyntaxError::from(error))
    }
}
