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

use crate::{errors::LockFileError, root::Dependency};

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap, fs::File, io::Write, path::Path};

pub const LOCKFILE_FILENAME: &str = "Leo.lock";

/// Lock-file struct, contains all information about imported dependencies
/// and their relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockFile {
    pub package: Vec<Package>,
}

/// Single dependency record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    // pub import_name: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub dependencies: HashMap<String, String>,
}

impl LockFile {
    pub fn new() -> Self {
        LockFile { package: vec![] }
    }

    /// Add Package record to the lock file. Chainable.
    pub fn add_package(&mut self, package: Package) -> &mut Self {
        self.package.push(package);
        self
    }

    pub fn to_string(&self) -> Result<String, LockFileError> {
        Ok(toml::to_string(self)?)
    }

    /// Write Leo.lock to the given location.
    pub fn write_to(self, path: &Path) -> Result<(), LockFileError> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(LOCKFILE_FILENAME);
        }

        let mut file = File::create(&path).map_err(|error| LockFileError::Creating(LOCKFILE_FILENAME, error))?;
        file.write_all(self.to_string()?.as_bytes())
            .map_err(|error| LockFileError::Writing(LOCKFILE_FILENAME, error))
    }
}

impl Package {
    /// Fill dependencies from Leo Manifest data.
    pub fn add_dependencies(&mut self, dependencies: &HashMap<String, Dependency>) {
        for (import_name, dependency) in dependencies.iter() {
            self.dependencies
                .insert(import_name.clone(), dependency.package.clone());
        }
    }
}

impl From<Dependency> for Package {
    fn from(dependency: Dependency) -> Package {
        let Dependency {
            author,
            version,
            package,
        } = dependency;

        Package {
            name: package,
            author,
            version,
            dependencies: Default::default(),
        }
    }
}
