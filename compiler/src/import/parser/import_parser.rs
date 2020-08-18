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

use crate::errors::ImportError;
use leo_typed::Program;

use std::{collections::HashMap, env::current_dir};

/// Parses all relevant import files for a program.
/// Stores compiled program structs.
#[derive(Clone)]
pub struct ImportParser {
    imports: HashMap<String, Program>,
}

impl ImportParser {
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, file_name: String, program: Program) {
        // todo: handle conflicting versions for duplicate imports here
        let _res = self.imports.insert(file_name, program);
    }

    pub fn get(&self, file_name: &String) -> Option<&Program> {
        self.imports.get(file_name)
    }

    pub fn parse(program: &Program) -> Result<Self, ImportError> {
        let mut imports = Self::new();

        // Find all imports relative to current directory
        let path = current_dir().map_err(|error| ImportError::current_directory_error(error))?;

        // Parse each imported file
        program
            .imports
            .iter()
            .map(|import| imports.parse_package(path.clone(), &import.package))
            .collect::<Result<Vec<()>, ImportError>>()?;

        Ok(imports)
    }
}
