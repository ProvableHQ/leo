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

use crate::{errors::ImportParserError, ImportParser};
use leo_ast::{Program, Span};
use leo_grammar::Grammar;

use std::fs::DirEntry;

static LIBRARY_FILE: &str = "src/lib.leo";

impl ImportParser {
    ///
    /// Returns a Leo syntax tree from a given package.
    ///
    /// Builds an abstract syntax tree from the given file and then builds the Leo syntax tree.
    ///
    pub(crate) fn parse_import_file(package: &DirEntry, span: &Span) -> Result<Program, ImportParserError> {
        // Get the package file type.
        let file_type = package
            .file_type()
            .map_err(|error| ImportParserError::directory_error(error, span.clone(), &package.path()))?;
        let file_name = package
            .file_name()
            .into_string()
            .map_err(|_| ImportParserError::convert_os_string(span.clone()))?;

        let mut file_path = package.path();
        if file_type.is_dir() {
            file_path.push(LIBRARY_FILE);

            if !file_path.exists() {
                return Err(ImportParserError::expected_lib_file(
                    format!("{:?}", file_path.as_path()),
                    span.clone(),
                ));
            }
        }

        // Build the package abstract syntax tree.
        let program_string = &Grammar::load_file(&file_path)?;
        let ast = &Grammar::new(&file_path, &program_string)?;

        // Build the package Leo syntax tree from the package abstract syntax tree.
        Ok(Program::from(&file_name, ast.as_repr()))
    }
}
