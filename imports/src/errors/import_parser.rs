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
use leo_asg::AsgConvertError;
use leo_ast::{FormattedError, Identifier, LeoError, Span};
use leo_parser::SyntaxError;

use std::{io, path::Path};

#[derive(Debug, Error)]
pub enum ImportParserError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),
    #[error("{}", _0)]
    AsgConvertError(#[from] AsgConvertError),
}

impl LeoError for ImportParserError {}

impl Into<AsgConvertError> for ImportParserError {
    fn into(self) -> AsgConvertError {
        match self {
            ImportParserError::Error(x) => AsgConvertError::ImportError(x),
            ImportParserError::SyntaxError(x) => x.into(),
            ImportParserError::AsgConvertError(x) => x,
        }
    }
}

impl ImportParserError {
    fn new_from_span(message: String, span: &Span) -> Self {
        ImportParserError::Error(FormattedError::new_from_span(message, span))
    }

    ///
    /// An imported package has the same name as an imported core_package.
    ///
    pub fn conflicting_imports(identifier: Identifier) -> Self {
        let message = format!("conflicting imports found for `{}`.", identifier.name);

        Self::new_from_span(message, &identifier.span)
    }

    pub fn recursive_imports(package: &str, span: &Span) -> Self {
        let message = format!("recursive imports for `{}`.", package);

        Self::new_from_span(message, span)
    }

    ///
    /// Failed to convert a file path into an os string.
    ///
    pub fn convert_os_string(span: &Span) -> Self {
        let message = "Failed to convert file string name, maybe an illegal character?".to_string();

        Self::new_from_span(message, span)
    }

    ///
    /// Failed to find the directory of the current file.
    ///
    pub fn current_directory_error(error: io::Error) -> Self {
        let message = format!("Compilation failed trying to find current directory - {:?}.", error);

        Self::new_from_span(message, &Span::default())
    }

    ///
    /// Failed to open or get the name of a directory.
    ///
    pub fn directory_error(error: io::Error, span: &Span, path: &Path) -> Self {
        let message = format!(
            "Compilation failed due to directory error @ '{}' - {:?}.",
            path.to_str().unwrap_or_default(),
            error
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Failed to find a main file for the current package.
    ///
    pub fn expected_main_file(entry: String, span: &Span) -> Self {
        let message = format!("Expected main file at `{}`.", entry,);

        Self::new_from_span(message, span)
    }

    ///
    /// Failed to import a package name.
    ///
    pub fn unknown_package(identifier: Identifier) -> Self {
        let message = format!(
            "Cannot find imported package `{}` in source files or import directory.",
            identifier.name
        );

        Self::new_from_span(message, &identifier.span)
    }

    pub fn io_error(span: &Span, path: &str, error: std::io::Error) -> Self {
        let message = format!("cannot read imported file '{}': {:?}", path, error,);

        Self::new_from_span(message, span)
    }
}
