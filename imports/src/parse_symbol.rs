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

use crate::{errors::ImportError, ImportParser};
use leo_ast::LeoAst;
use leo_typed::{ImportSymbol, Program, Span};

use std::{ffi::OsString, fs::DirEntry, path::PathBuf};

static LIBRARY_FILE: &str = "src/lib.leo";
static FILE_EXTENSION: &str = "leo";

fn parse_import_file(entry: &DirEntry, span: &Span) -> Result<Program, ImportError> {
    // make sure the given entry is file
    let file_type = entry
        .file_type()
        .map_err(|error| ImportError::directory_error(error, span.clone(), entry.path()))?;
    let file_name = entry
        .file_name()
        .to_os_string()
        .into_string()
        .map_err(|_| ImportError::convert_os_string(span.clone()))?;

    let mut file_path = entry.path().to_path_buf();
    if file_type.is_dir() {
        file_path.push(LIBRARY_FILE);

        if !file_path.exists() {
            return Err(ImportError::expected_lib_file(
                format!("{:?}", file_path.as_path()),
                span.clone(),
            ));
        }
    }

    // Builds the abstract syntax tree.
    let program_string = &LeoAst::load_file(&file_path)?;
    let ast = &LeoAst::new(&file_path, &program_string)?;

    // Generates the Leo program from file.
    Ok(Program::from(&file_name, ast.as_repr()))
}

impl ImportParser {
    pub fn parse_import_star(&mut self, entry: &DirEntry, span: &Span) -> Result<(), ImportError> {
        let path = entry.path();
        let is_dir = path.is_dir();
        let is_leo_file = path
            .extension()
            .map_or(false, |ext| ext.eq(&OsString::from(FILE_EXTENSION)));

        let mut package_path = path.to_path_buf();
        package_path.push(LIBRARY_FILE);

        let is_package = is_dir && package_path.exists();

        // import * can only be invoked on a package with a library file or a leo file
        if is_package || is_leo_file {
            // Generate aleo program from file
            let program = parse_import_file(entry, &span)?;

            // Store program's imports in imports hashmap
            program
                .imports
                .iter()
                .map(|import| self.parse_package(entry.path(), &import.package))
                .collect::<Result<Vec<()>, ImportError>>()?;

            // Store program in imports hashmap
            let file_name_path = PathBuf::from(entry.file_name());
            let file_name = file_name_path
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(); // the file exists so these will not fail

            self.insert_import(file_name, program);

            Ok(())
        } else {
            // importing * from a directory or non-leo file in `package/src/` is illegal
            Err(ImportError::star(entry.path().to_path_buf(), span.clone()))
        }
    }

    pub fn parse_import_symbol(&mut self, entry: &DirEntry, symbol: &ImportSymbol) -> Result<(), ImportError> {
        // Generate aleo program from file
        let program = parse_import_file(entry, &symbol.span)?;

        // Store program's imports in imports hashmap
        program
            .imports
            .iter()
            .map(|import| self.parse_package(entry.path(), &import.package))
            .collect::<Result<Vec<()>, ImportError>>()?;

        // Store program in imports hashmap
        let file_name_path = PathBuf::from(entry.file_name());
        let file_name = file_name_path
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(); // the file exists so these will not fail

        self.insert_import(file_name, program);

        Ok(())
    }
}
