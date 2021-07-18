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
use std::{
    borrow::Cow,
    collections::HashMap,
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    path::Path,
};

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
    pub name: String,
    pub version: String,
    pub author: String,
    pub import_name: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub dependencies: HashMap<String, String>,
}

impl LockFile {
    pub fn new() -> Self {
        LockFile { package: vec![] }
    }

    /// Check if LockFile exists in a directory.
    pub fn exists_at(path: &Path) -> bool {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(LOCKFILE_FILENAME);
        }
        path.exists()
    }

    /// Add Package record to the lock file. Chainable.
    pub fn add_package(&mut self, package: Package) -> &mut Self {
        self.package.push(package);
        self
    }

    /// Print LockFile as toml.
    pub fn to_string(&self) -> Result<String, LockFileError> {
        Ok(toml::to_string(self)?)
    }

    /// Form a HashMap of kind:
    /// ``` imported_name => package_name ```
    /// for all imported packages.
    pub fn to_import_map(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for package in self.package.iter() {
            match &package.import_name {
                Some(name) => result.insert(name.clone(), package.to_string()),
                None => result.insert(package.name.clone(), package.to_string()),
            };
        }

        result
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

impl TryFrom<&Path> for LockFile {
    type Error = LockFileError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(LOCKFILE_FILENAME);
        }

        let mut file = File::open(path.clone()).map_err(|error| LockFileError::Opening(LOCKFILE_FILENAME, error))?;
        let size = file
            .metadata()
            .map_err(|error| LockFileError::Metadata(LOCKFILE_FILENAME, error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| LockFileError::Reading(LOCKFILE_FILENAME, error))?;

        toml::from_str(&buffer).map_err(|error| LockFileError::Parsing(LOCKFILE_FILENAME, error))
    }
}

impl Package {
    /// Fill dependencies from Leo Manifest data.
    pub fn add_dependencies(&mut self, dependencies: &HashMap<String, Dependency>) {
        for (import_name, dependency) in dependencies.iter() {
            self.dependencies
                .insert(import_name.clone(), Package::from(dependency).to_string());
        }
    }

    /// Form an path identifier for a package. It is the path under which package is stored
    /// inside the `imports/` directory.
    pub fn to_string(&self) -> String {
        format!("{}-{}@{}", self.author, self.name, self.version)
    }
}

impl From<&Dependency> for Package {
    fn from(dependency: &Dependency) -> Package {
        Package {
            name: dependency.package.clone(),
            author: dependency.author.clone(),
            version: dependency.version.clone(),
            dependencies: Default::default(),
            import_name: None,
        }
    }
}
