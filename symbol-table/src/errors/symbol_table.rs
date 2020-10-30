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

use crate::{TypeError, UserDefinedType};
use leo_core_ast::{Error as FormattedError, ImportSymbol, Program, Span};
use leo_core_packages::{CorePackageListError, LeoCorePackageError};

use std::path::Path;

/// Errors encountered when tracking variable, function, and circuit names in a program.
#[derive(Debug, Error)]
pub enum SymbolTableError {
    #[error("{}", _0)]
    CorePackageListError(#[from] CorePackageListError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    LeoCorePackageError(#[from] LeoCorePackageError),

    #[error("{}", _0)]
    TypeError(#[from] TypeError),
}

impl SymbolTableError {
    ///
    /// Sets the filepath for the error stacktrace.
    ///
    pub fn set_path(&mut self, path: &Path) {
        match self {
            SymbolTableError::CorePackageListError(error) => error.set_path(path),
            SymbolTableError::Error(error) => error.set_path(path),
            SymbolTableError::LeoCorePackageError(error) => error.set_path(path),
            SymbolTableError::TypeError(error) => error.set_path(path),
        }
    }

    ///
    /// Returns a new formatted error with a given message and span information.
    ///
    fn new_from_span(message: String, span: Span) -> Self {
        SymbolTableError::Error(FormattedError::new_from_span(message, span))
    }

    ///
    /// Two circuits have been defined with the same name.
    ///
    pub fn duplicate_circuit(variable: UserDefinedType) -> Self {
        let message = format!("Duplicate circuit definition found for `{}`", variable.identifier);

        Self::new_from_span(message, variable.identifier.span)
    }

    ///
    /// Two functions have been defined with the same name.
    ///
    pub fn duplicate_function(variable: UserDefinedType) -> Self {
        let message = format!("Duplicate function definition found for `{}`", variable.identifier);

        Self::new_from_span(message, variable.identifier.span)
    }

    ///
    /// Attempted to access a package name that is not defined.
    ///
    pub fn unknown_package(name: &str, span: &Span) -> Self {
        let message = format!(
            "Cannot find imported package `{}` in source files or import directory",
            name
        );

        Self::new_from_span(message, span.to_owned())
    }

    ///
    /// Attempted to import a name that is not defined in the current file.
    ///
    pub fn unknown_symbol(symbol: &ImportSymbol, program: &Program) -> Self {
        let message = format!(
            "Cannot find imported symbol `{}` in imported file `{}`",
            symbol, program.name
        );

        Self::new_from_span(message, symbol.span.to_owned())
    }
}
