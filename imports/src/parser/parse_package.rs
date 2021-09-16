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

use crate::ImportParser;
use leo_ast::Program;
use leo_errors::{ImportError, Result, Span};

use std::{fs, fs::DirEntry, path::PathBuf};

static SOURCE_FILE_EXTENSION: &str = ".leo";
static SOURCE_DIRECTORY_NAME: &str = "src/";
static IMPORTS_DIRECTORY_NAME: &str = "imports/";

impl ImportParser {
    fn parse_package_access(
        &mut self,
        package: &DirEntry,
        remaining_segments: &[&str],
        span: &Span,
    ) -> Result<Program> {
        if !remaining_segments.is_empty() {
            return self.parse_package(package.path(), remaining_segments, span);
        }

        Self::parse_import_file(package, span)
    }

    ///
    /// Create the Leo syntax tree for an imported package.
    ///
    /// Inserts the Leo syntax tree into the `ImportParser`.
    ///
    pub(crate) fn parse_package(&mut self, mut path: PathBuf, segments: &[&str], span: &Span) -> Result<Program> {
        let error_path = path.clone();
        let package_name = segments[0];

        // Fetch a core package
        let core_package = package_name.eq("core");
        if core_package {
            panic!("attempted to import core package from filesystem");
        }

        // Trim path if importing from another file
        if path.is_file() {
            path.pop();
        }

        // Search for package name in local directory
        let mut source_directory = path.clone();
        source_directory.push(SOURCE_DIRECTORY_NAME);

        // Search for package name in `imports` directory
        let mut imports_directory = path.clone();
        imports_directory.pop(); // path leads to src/ folder, imports is one level below
        imports_directory.push(IMPORTS_DIRECTORY_NAME);

        // Read from local `src` directory or the current path
        if source_directory.exists() {
            path = source_directory
        }

        // Get a vector of all packages in the source directory.
        let entries = fs::read_dir(path)
            .map_err(|error| ImportError::directory_error(error, &error_path, span))?
            .collect::<Result<Vec<_>, std::io::Error>>()
            .map_err(|error| ImportError::directory_error(error, &error_path, span))?;

        // Check if the imported package name is in the source directory.
        let matched_source_entry = entries.into_iter().find(|entry| {
            entry
                .file_name()
                .into_string()
                .unwrap()
                .trim_end_matches(SOURCE_FILE_EXTENSION)
                .eq(package_name)
        });

        if imports_directory.exists() {
            // Get a vector of all packages in the imports directory.
            let entries = fs::read_dir(imports_directory)
                .map_err(|error| ImportError::directory_error(error, &error_path, span))?
                .collect::<Result<Vec<_>, std::io::Error>>()
                .map_err(|error| ImportError::directory_error(error, error_path, span))?;

            // Keeping backward compatibilty for existing packages.
            // If index_map contains key, use it or try to access directly.
            // TODO: Remove when migration is possible.
            let package_name = self
                .imports_map
                .get(package_name)
                .unwrap_or(&package_name.to_string())
                .clone();

            // Check if the imported package name is in the imports directory.
            let matched_import_entry = entries
                .into_iter()
                .find(|entry| entry.file_name().into_string().unwrap().eq(&package_name));

            // Check if the package name was found in both the source and imports directory.
            match (matched_source_entry, matched_import_entry) {
                (Some(_), Some(_)) => Err(ImportError::conflicting_imports(package_name, span).into()),
                (Some(source_entry), None) => self.parse_package_access(&source_entry, &segments[1..], span),
                (None, Some(import_entry)) => self.parse_package_access(&import_entry, &segments[1..], span),
                (None, None) => Err(ImportError::unknown_package(package_name, span).into()),
            }
        } else {
            // Enforce local package access with no found imports directory
            match matched_source_entry {
                Some(source_entry) => self.parse_package_access(&source_entry, &segments[1..], span),
                None => Err(ImportError::unknown_package(package_name, span).into()),
            }
        }
    }
}
