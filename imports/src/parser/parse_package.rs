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

use crate::{errors::ImportParserError, ImportParser, CORE_PACKAGE_NAME};
use leo_ast::{Package, PackageAccess};

use std::{fs, fs::DirEntry, path::PathBuf};

static SOURCE_FILE_EXTENSION: &str = ".leo";
static SOURCE_DIRECTORY_NAME: &str = "src/";
static IMPORTS_DIRECTORY_NAME: &str = "imports/";

impl ImportParser {
    ///
    /// Import one or more symbols from a package.
    ///
    /// Will recursively traverse sub packages until the desired symbol is found.
    ///
    pub fn parse_package_access(
        &mut self,
        package: &DirEntry,
        access: &PackageAccess,
    ) -> Result<(), ImportParserError> {
        tracing::debug!("import {:?}", package.path());

        match access {
            PackageAccess::Star(span) => self.parse_import_star(package, span),
            PackageAccess::Symbol(symbol) => self.parse_import_symbol(package, symbol),
            PackageAccess::SubPackage(sub_package) => self.parse_package(package.path(), sub_package),
            PackageAccess::Multiple(accesses) => {
                for access in accesses {
                    self.parse_package_access(package, access)?;
                }

                Ok(())
            }
        }
    }

    ///
    /// Create the typed syntax tree for an imported package.
    ///
    /// Inserts the typed syntax tree into the `ImportParser`.
    ///
    pub fn parse_package(&mut self, mut path: PathBuf, package: &Package) -> Result<(), ImportParserError> {
        let error_path = path.clone();
        let package_name = package.name.clone();

        // Fetch a core package
        let core_package = package_name.name.eq(CORE_PACKAGE_NAME);

        // Trim path if importing from another file
        if path.is_file() {
            path.pop();
        }

        // Search for package name in local directory
        let mut source_directory = path.clone();
        source_directory.push(SOURCE_DIRECTORY_NAME);

        // Search for package name in `imports` directory
        let mut imports_directory = path.clone();
        imports_directory.push(IMPORTS_DIRECTORY_NAME);

        // Read from local `src` directory or the current path
        if source_directory.exists() {
            path = source_directory
        }

        // Get a vector of all packages in the source directory.
        let entries = fs::read_dir(path)
            .map_err(|error| ImportParserError::directory_error(error, package_name.span.clone(), &error_path))?
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(|error| ImportParserError::directory_error(error, package_name.span.clone(), &error_path))?;

        // Check if the imported package name is in the source directory.
        let matched_source_entry = entries.into_iter().find(|entry| {
            entry
                .file_name()
                .into_string()
                .unwrap()
                .trim_end_matches(SOURCE_FILE_EXTENSION)
                .eq(&package_name.name)
        });

        if core_package {
            // Enforce core package access.
            self.parse_core_package(&package)
        } else if imports_directory.exists() {
            // Get a vector of all packages in the imports directory.
            let entries = fs::read_dir(imports_directory)
                .map_err(|error| ImportParserError::directory_error(error, package_name.span.clone(), &error_path))?
                .collect::<Result<Vec<_>, std::io::Error>>()
                .map_err(|error| ImportParserError::directory_error(error, package_name.span.clone(), &error_path))?;

            // Check if the imported package name is in the imports directory.
            let matched_import_entry = entries
                .into_iter()
                .find(|entry| entry.file_name().into_string().unwrap().eq(&package_name.name));

            // Check if the package name was found in both the source and imports directory.
            match (matched_source_entry, matched_import_entry) {
                (Some(_), Some(_)) => Err(ImportParserError::conflicting_imports(package_name)),
                (Some(source_entry), None) => self.parse_package_access(&source_entry, &package.access),
                (None, Some(import_entry)) => self.parse_package_access(&import_entry, &package.access),
                (None, None) => Err(ImportParserError::unknown_package(package_name)),
            }
        } else {
            // Enforce local package access with no found imports directory
            match matched_source_entry {
                Some(source_entry) => self.parse_package_access(&source_entry, &package.access),
                None => Err(ImportParserError::unknown_package(package_name)),
            }
        }
    }
}
