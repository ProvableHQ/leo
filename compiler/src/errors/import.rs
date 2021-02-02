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

use leo_ast::{Error as FormattedError, Identifier, ImportSymbol, Span};
use leo_core::LeoCorePackageError;

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    LeoCoreError(#[from] LeoCorePackageError),
}

impl ImportError {
    fn new_from_span(message: String, span: Span) -> Self {
        ImportError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn unknown_package(identifier: Identifier) -> Self {
        let message = format!(
            "cannot find imported package `{}` in source files or import directory",
            identifier.name
        );

        Self::new_from_span(message, identifier.span)
    }

    pub fn unknown_symbol(symbol: ImportSymbol, file: String) -> Self {
        let message = format!("cannot find imported symbol `{}` in imported file `{}`", symbol, file);
        let error = FormattedError::new_from_span(message, symbol.span);

        ImportError::Error(error)
    }
}
