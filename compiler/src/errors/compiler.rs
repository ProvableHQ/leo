// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::errors::{ExpressionError, FunctionError, ImportError, StatementError};
use leo_asg::{AsgConvertError, FormattedError};
use leo_ast::{AstError, LeoError};
use leo_input::InputParserError;
use leo_parser::SyntaxError;
use leo_state::LocalDataVerificationError;

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("{}", _0)]
    AsgPassError(FormattedError),

    #[error("{}", _0)]
    ExpressionError(#[from] ExpressionError),

    #[error("{}", _0)]
    ImportError(#[from] ImportError),

    #[error("{}", _0)]
    InputParserError(#[from] InputParserError),

    #[error("Cannot find input files with context name `{}`", _0)]
    InvalidTestContext(String),

    #[error("{}", _0)]
    FunctionError(#[from] FunctionError),

    #[error("Cannot read from the provided file path '{:?}': {}", _0, _1)]
    FileReadError(PathBuf, std::io::Error),

    #[error("{}", _0)]
    LocalDataVerificationError(#[from] LocalDataVerificationError),

    #[error("`main` must be a function")]
    NoMainFunction,

    #[error("Failed to find input files for the current test")]
    NoTestInput,

    #[error("{}", _0)]
    AsgConvertError(#[from] AsgConvertError),

    #[error("{}", _0)]
    AstError(#[from] AstError),

    #[error("{}", _0)]
    StatementError(#[from] StatementError),
}

impl LeoError for CompilerError {}
