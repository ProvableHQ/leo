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

use crate::errors::{
    AddressError,
    BooleanError,
    ExpressionError,
    FieldError,
    GroupError,
    IntegerError,
    OutputError,
    StatementError,
    ValueError,
};
use leo_ast::{Error as FormattedError, Span};

use std::path::Path;

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("{}", _0)]
    AddressError(#[from] AddressError),

    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    ExpressionError(#[from] ExpressionError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    OutputStringError(#[from] OutputError),

    #[error("{}", _0)]
    StatementError(#[from] StatementError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}

impl FunctionError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            FunctionError::AddressError(error) => error.set_path(path),
            FunctionError::BooleanError(error) => error.set_path(path),
            FunctionError::ExpressionError(error) => error.set_path(path),
            FunctionError::Error(error) => error.set_path(path),
            FunctionError::FieldError(error) => error.set_path(path),
            FunctionError::GroupError(error) => error.set_path(path),
            FunctionError::IntegerError(error) => error.set_path(path),
            FunctionError::OutputStringError(error) => error.set_path(path),
            FunctionError::StatementError(error) => error.set_path(path),
            FunctionError::ValueError(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        FunctionError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn invalid_array(actual: String, span: Span) -> Self {
        let message = format!("Expected function input array, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_tuple(actual: String, span: Span) -> Self {
        let message = format!("Expected function input tuple, found `{}`", actual);

        Self::new_from_span(message, span)
    }

    pub fn return_arguments_length(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!("function expected {} returns, found {} returns", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn return_argument_type(expected: String, actual: String, span: Span) -> Self {
        let message = format!("Expected function return type `{}`, found `{}`", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn input_not_found(expected: String, span: Span) -> Self {
        let message = format!("main function input {} not found", expected);

        Self::new_from_span(message, span)
    }
}
