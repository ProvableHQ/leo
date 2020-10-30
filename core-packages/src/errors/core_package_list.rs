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

use leo_core_ast::{Error as FormattedError, ImportSymbol, Span};

use crate::CorePackageError;
use std::path::Path;

#[derive(Debug, Error)]
pub enum CorePackageListError {
    #[error("{}", _0)]
    CorePackageError(#[from] CorePackageError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl CorePackageListError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            CorePackageListError::CorePackageError(error) => error.set_path(path),
            CorePackageListError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        CorePackageListError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn invalid_core_package(symbol: ImportSymbol) -> Self {
        let message = format!("No package `{}` in leo-core", symbol);
        let span = symbol.span;

        Self::new_from_span(message, span)
    }

    pub fn core_package_star(span: Span) -> Self {
        let message = "Cannot import star from leo-core".to_string();

        Self::new_from_span(message, span)
    }
}
