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
use leo_ast::{ImportSymbol, Program, Span};
use leo_grammar::Grammar;

use std::{ffi::OsString, fs::DirEntry, path::PathBuf};

static LIBRARY_FILE: &str = "src/lib.leo";
static FILE_EXTENSION: &str = "leo";

///
/// Returns a typed syntax tree from a given package.
///
/// Builds an abstract syntax tree from the given file and then builds the typed syntax tree.
///
fn parse_import_file(package: &DirEntry, span: &Span) -> Result<Program, ImportParserError> {
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

    // Build the package typed syntax tree from the package abstract syntax tree.
    Ok(Program::from(&file_name, ast.as_repr()))
}

impl ImportParser {
    ///
    /// Import all symbols from a given package.
    ///
    /// If the package is a Leo file, import all symbols from the file.
    /// If the package is a directory, import all symbol from the library file.
    ///
    pub fn parse_import_star(&mut self, package: &DirEntry, span: &Span) -> Result<(), ImportParserError> {
        let path = package.path();
        let is_dir = path.is_dir();

        // Check if the package is a Leo file.
        let is_leo_file = path
            .extension()
            .map_or(false, |ext| ext.eq(&OsString::from(FILE_EXTENSION)));

        let mut package_path = path;
        package_path.push(LIBRARY_FILE);

        // Check if the package is a directory.
        let is_package = is_dir && package_path.exists();

        // import * can only be invoked on a package with a library file or a leo file
        if is_package || is_leo_file {
            self.parse_import_package(package, span)
        } else {
            // importing * from a directory or non-leo file in `package/src/` is illegal
            Err(ImportParserError::star(&package.path(), span.clone()))
        }
    }

    ///
    /// Import a symbol from a given package.
    ///
    pub fn parse_import_symbol(&mut self, package: &DirEntry, symbol: &ImportSymbol) -> Result<(), ImportParserError> {
        // Get the package typed syntax tree.
        self.parse_import_package(package, &symbol.span)
    }

    ///
    /// Import a symbol from a given package.
    ///
    pub fn parse_import_package(&mut self, package: &DirEntry, span: &Span) -> Result<(), ImportParserError> {
        // Get the package typed syntax tree.
        let program = parse_import_file(package, span)?;

        // Insert the package's imports into the import parser.
        for import in &program.imports {
            self.parse_package(package.path(), &import.package)?;
        }

        // Get the package file name from the path.
        let file_name_path = PathBuf::from(package.file_name());
        let file_name = file_name_path
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(); // the file exists so these will not fail

        // Attempt to insert the typed syntax tree for the imported package.
        self.insert_import(file_name, program);

        Ok(())
    }
}
