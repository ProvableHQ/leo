// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

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

impl ParserError {
    pub fn set_path(&mut self, path: PathBuf) {
        if let ParserError::SyntaxError(error) = self {
            let new_error: Error<Rule> = match error {
                SyntaxError::Error(error) => {
                    let new_error = error.clone();
                    new_error.with_path(path.to_str().unwrap())
                }
            };

            tracing::error!("{}", new_error);

            *error = SyntaxError::Error(new_error);
        }
    }
}

impl From<Error<Rule>> for ParserError {
    fn from(error: Error<Rule>) -> Self {
        ParserError::SyntaxError(SyntaxError::from(error))
    }
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::Crate("std::io", error.to_string())
    }
}
