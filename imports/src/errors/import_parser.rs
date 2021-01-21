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
use leo_ast::{Error as FormattedError, Identifier, Span};
use leo_grammar::ParserError;
use leo_asg::AsgConvertError;

use std::{io, path::Path};

#[derive(Debug, Error)]
pub enum ImportParserError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    ParserError(#[from] ParserError),
    #[error("{}", _0)]
    AsgConvertError(#[from] AsgConvertError),
}

impl Into<AsgConvertError> for ImportParserError {
    fn into(self) -> AsgConvertError {
        match self {
            ImportParserError::Error(x) => AsgConvertError::ImportError(x),
            ImportParserError::ParserError(x) => x.into(),
            ImportParserError::AsgConvertError(x) => x,
        }
    }
}

impl ImportParserError {
    fn new_from_span(message: String, span: Span) -> Self {
        ImportParserError::Error(FormattedError::new_from_span(message, span))
    }

    fn new_from_span_with_path(message: String, span: Span, path: &Path) -> Self {
        ImportParserError::Error(FormattedError::new_from_span_with_path(message, span, path))
    }

    ///
    /// An imported package has the same name as an imported core_package.
    ///
    pub fn conflicting_imports(identifier: Identifier) -> Self {
        let message = format!("conflicting imports found for `{}`.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    pub fn recursive_imports(package: &str, span: &Span) -> Self {
        let message = format!("recursive imports for `{}`.", package);

        Self::new_from_span(message, span.clone())
    }

    ///
    /// A core package name has been imported twice.
    ///
    pub fn duplicate_core_package(identifier: Identifier) -> Self {
        let message = format!("Duplicate core_package import `{}`.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    ///
    /// Failed to convert a file path into an os string.
    ///
    pub fn convert_os_string(span: Span) -> Self {
        let message = "Failed to convert file string name, maybe an illegal character?".to_string();

        Self::new_from_span(message, span)
    }

    ///
    /// Failed to find the directory of the current file.
    ///
    pub fn current_directory_error(error: io::Error) -> Self {
        let span = Span {
            text: "".to_string(),
            line: 0,
            start: 0,
            end: 0,
        };
        let message = format!("Compilation failed trying to find current directory - {:?}.", error);

        Self::new_from_span(message, span)
    }

    ///
    /// Failed to open or get the name of a directory.
    ///
    pub fn directory_error(error: io::Error, span: Span, path: &Path) -> Self {
        let message = format!("Compilation failed due to directory error - {:?}.", error);

        Self::new_from_span_with_path(message, span, path)
    }

    ///
    /// Failed to import all symbols at a package path.
    ///
    pub fn star(path: &Path, span: Span) -> Self {
        let message = format!("Cannot import `*` from path `{:?}`.", path);

        Self::new_from_span(message, span)
    }

    ///
    /// Failed to find a library file for the current package.
    ///
    pub fn expected_lib_file(entry: String, span: Span) -> Self {
        let message = format!(
            "Expected library file`{}` when looking for symbol `{}`.",
            entry, span.text
        );

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

        Self::new_from_span(message, identifier.span)
    }
}
