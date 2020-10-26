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

use crate::errors::ImportParserError;
use leo_typed::{Package, Program, Span};

use std::{collections::HashMap, env::current_dir};

/// Stores imported packages.
///
/// A program can import one or more packages. A package can be found locally in the source
/// directory, foreign in the imports directory, or part of the core package list.
#[derive(Clone)]
pub struct ImportParser {
    imports: HashMap<String, Program>,
    core_packages: Vec<Package>,
}

impl ImportParser {
    ///
    /// Creates a new empty `ImportParser`.
    ///
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
            core_packages: vec![],
        }
    }

    ///
    /// Inserts a (file name -> program) pair into the `ImportParser`.
    ///
    /// If the map did not have this file name present, `Ok()` is returned.
    ///
    /// If the map did have this file name present, a duplicate import error is thrown.
    ///
    pub(crate) fn insert_import(
        &mut self,
        file_name: String,
        program: Program,
        span: &Span,
    ) -> Result<(), ImportParserError> {
        // Insert the imported program.
        let duplicate = self.imports.insert(file_name.clone(), program);

        // Check for duplicate import name.
        if duplicate.is_some() {
            return Err(ImportParserError::duplicate_import(file_name, span.clone()));
        }

        Ok(())
    }

    ///
    /// Inserts a core package into the `ImportParser`.
    ///
    /// If the vector did not have this file_name present, `Ok()` is returned.
    ///
    /// If the vector did have this file_name present, a duplicate import error is thrown.
    ///
    pub(crate) fn insert_core_package(&mut self, package: &Package) -> Result<(), ImportParserError> {
        // Check for duplicate core package name.
        if self.core_packages.contains(package) {
            return Err(ImportParserError::duplicate_core_package(package.name.clone()));
        }

        // Append the core package.
        self.core_packages.push(package.clone());

        Ok(())
    }

    ///
    /// Returns a reference to the program corresponding to the file name.
    ///
    pub fn get_import(&self, file_name: &String) -> Option<&Program> {
        self.imports.get(file_name)
    }

    ///
    /// Returns a reference to the core package corresponding to the given package.
    ///
    pub fn get_core_package(&self, package: &Package) -> Option<&Package> {
        self.core_packages()
            .iter()
            .find(|core_package| core_package.eq(&package))
    }

    ///
    /// Returns a reference to the vector of core packages.
    ///
    pub fn core_packages(&self) -> &Vec<Package> {
        &self.core_packages
    }

    ///
    /// Returns a new `ImportParser` from a given `Program`.
    ///
    /// For every import statement in the program:
    ///     1. Check if the imported package exists.
    ///     2. Create the typed syntax tree for the imported package.
    ///     3. Insert the typed syntax tree into the `ImportParser`
    ///
    pub fn parse(program: &Program) -> Result<Self, ImportParserError> {
        let mut imports = Self::new();

        // Find all imports relative to current directory.
        let path = current_dir().map_err(|error| ImportParserError::current_directory_error(error))?;

        // Parse each import statement.
        program
            .imports
            .iter()
            .map(|import| imports.parse_package(path.clone(), &import.package))
            .collect::<Result<Vec<()>, ImportParserError>>()?;

        Ok(imports)
    }
}
