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

use crate::package::Package;
use crate::PackageFile;
use leo_errors::{PackageError, Result};

use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const MANIFEST_FILENAME: &str = "Leo.toml";
pub const AUTHOR_PLACEHOLDER: &str = "[AUTHOR]";

#[derive(Clone, Deserialize)]
pub struct Remote {
    pub author: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Dependency {
    pub author: String,
    pub version: String,
    pub package: String,
}

#[derive(Deserialize)]
pub struct Manifest {
    pub project: Package,
    pub remote: Option<Remote>,
    pub dependencies: Option<IndexMap<String, Dependency>>,
}

impl PackageFile for Manifest {
    type ParentDirectory = super::RootDirectory;

    fn template(&self) -> String {
        let author = self
            .remote
            .clone()
            .map_or(AUTHOR_PLACEHOLDER.to_string(), |remote| remote.author);

        format!(
            r#"[project]
name = "{name}"
version = "0.1.0"
description = "The {name} package"
license = "MIT"

[remote]
author = "{author}" # Add your Aleo Package Manager username or team name.

[target]
curve = "bls12_377"
proving_system = "groth16"

[dependencies]
# Define dependencies here in format:
# name = {{ package = "package-name", author = "author", version = "version" }}
"#,
            name = self.project.name,
            author = author
        )
    }
}

impl std::fmt::Display for Manifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Leo.toml")
    }
}

impl Manifest {
    pub fn new(package_name: &str, author: Option<String>) -> Result<Self> {
        Ok(Self {
            project: Package::new(package_name)?,
            remote: author.map(|author| Remote { author }),
            dependencies: Some(IndexMap::<String, Dependency>::new()),
        })
    }

    pub fn get_package_name(&self) -> String {
        self.project.name.clone()
    }

    pub fn get_package_version(&self) -> String {
        self.project.version.clone()
    }

    pub fn get_package_description(&self) -> Option<String> {
        self.project.description.clone()
    }

    pub fn get_package_dependencies(&self) -> Option<IndexMap<String, Dependency>> {
        self.dependencies.clone()
    }

    /// Get HashMap of kind:
    ///     import name => import directory
    /// Which then used in AST/ASG to resolve import paths.
    pub fn get_imports_map(&self) -> Option<HashMap<String, String>> {
        self.dependencies.clone().map(|dependencies| {
            dependencies
                .into_iter()
                .map(|(name, dependency)| {
                    (
                        name,
                        format!("{}-{}@{}", dependency.author, dependency.package, dependency.version),
                    )
                })
                .collect()
        })
    }

    pub fn get_package_license(&self) -> Option<String> {
        self.project.license.clone()
    }

    pub fn get_package_remote(&self) -> Option<Remote> {
        self.remote.clone()
    }
}

impl TryFrom<&Path> for Manifest {
    type Error = PackageError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut path = std::borrow::Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(MANIFEST_FILENAME);
        }

        let mut file = File::open(path.clone())
            .map_err(|error| PackageError::failed_to_open_lock_file(MANIFEST_FILENAME, error))?;
        let size = file
            .metadata()
            .map_err(|error| PackageError::failed_to_get_lock_file_metadata(MANIFEST_FILENAME, error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| PackageError::failed_to_read_manifest_file(MANIFEST_FILENAME, error))?;

        toml::from_str(&buffer).map_err(|error| PackageError::failed_to_parse_manifest_file(MANIFEST_FILENAME, error))
    }
}
