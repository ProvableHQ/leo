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

use crate::errors::{AddressError, BooleanError, FieldError, GroupError, IntegerError};
use leo_typed::{Error as FormattedError, Span};

use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("{}", _0)]
    AddressError(#[from] AddressError),

    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),
}

impl ValueError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            ValueError::AddressError(error) => error.set_path(path),
            ValueError::BooleanError(error) => error.set_path(path),
            ValueError::Error(error) => error.set_path(path),
            ValueError::FieldError(error) => error.set_path(path),
            ValueError::GroupError(error) => error.set_path(path),
            ValueError::IntegerError(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        ValueError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn implicit(value: String, span: Span) -> Self {
        let message = format!("explicit type needed for `{}`", value);

        Self::new_from_span(message, span)
    }

    pub fn implicit_group(span: Span) -> Self {
        let message = "group coordinates should be in (x, y)group format".to_string();

        Self::new_from_span(message, span)
    }
}
