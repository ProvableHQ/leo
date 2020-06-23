use crate::errors::{FunctionError, ImportError};
use leo_ast::ParserError;
use leo_inputs::InputParserError;

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("{}", _0)]
    ImportError(#[from] ImportError),

    #[error("{}", _0)]
    InputParserError(#[from] InputParserError),

    #[error("{}", _0)]
    FunctionError(#[from] FunctionError),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("`main` function not found")]
    NoMain,

    #[error("`main` must be a function")]
    NoMainFunction,

    #[error("{}", _0)]
    ParserError(#[from] ParserError),
}

impl CompilerError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            CompilerError::FunctionError(error) => error.set_path(path),
            _ => {}
        }
    }
}
