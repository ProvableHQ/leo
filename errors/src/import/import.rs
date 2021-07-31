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

use crate::{ErrorCode, FormattedError, LeoErrorCode, Span};

#[derive(Debug, Error)]
pub enum ImportError {
    #[error(transparent)]
    FormattedError(#[from] FormattedError),
}

impl LeoErrorCode for ImportError {}

impl ErrorCode for ImportError {
    #[inline(always)]
    fn exit_code_mask() -> u32 {
        3000
    }

    #[inline(always)]
    fn error_type() -> String {
        "I".to_string()
    }

    fn new_from_span(message: String, help: Option<String>, exit_code: u32, span: &Span) -> Self {
        Self::FormattedError(FormattedError::new_from_span(
            message,
            help,
            exit_code ^ Self::exit_code_mask(),
            Self::code_identifier(),
            Self::error_type(),
            span,
        ))
    }
}

impl ImportError {
    ///
    /// An imported package has the same name as an imported core_package.
    ///
    pub fn conflicting_imports(name: &str, span: &Span) -> Self {
        let message = format!("conflicting imports found for `{}`.", name);

        Self::new_from_span(message, None, 0, span)
    }

    pub fn recursive_imports(package: &str, span: &Span) -> Self {
        let message = format!("recursive imports for `{}`.", package);

        Self::new_from_span(message, None, 1, span)
    }

    ///
    /// Failed to convert a file path into an os string.
    ///
    pub fn convert_os_string(span: &Span) -> Self {
        let message = "Failed to convert file string name, maybe an illegal character?".to_string();

        Self::new_from_span(message, None, 2, span)
    }

    ///
    /// Failed to find the directory of the current file.
    ///
    pub fn current_directory_error(error: std::io::Error) -> Self {
        let message = format!("Compilation failed trying to find current directory - {:?}.", error);

        Self::new_from_span(message, None, 3, &Span::default())
    }

    ///
    /// Failed to open or get the name of a directory.
    ///
    pub fn directory_error(error: std::io::Error, span: &Span, path: &std::path::Path) -> Self {
        let message = format!(
            "Compilation failed due to directory error @ '{}' - {:?}.",
            path.to_str().unwrap_or_default(),
            error
        );

        Self::new_from_span(message, None, 4, span)
    }

    ///
    /// Failed to find a main file for the current package.
    ///
    pub fn expected_main_file(entry: String, span: &Span) -> Self {
        let message = format!("Expected main file at `{}`.", entry,);

        Self::new_from_span(message, None, 5, span)
    }

    ///
    /// Failed to import a package name.
    ///
    pub fn unknown_package(name: &str, span: &Span) -> Self {
        let message = format!(
            "Cannot find imported package `{}` in source files or import directory.",
            name
        );

        Self::new_from_span(message, None, 6, span)
    }

    pub fn io_error(span: &Span, path: &str, error: std::io::Error) -> Self {
        let message = format!("cannot read imported file '{}': {:?}", path, error,);

        Self::new_from_span(message, None, 7, span)
    }
}
