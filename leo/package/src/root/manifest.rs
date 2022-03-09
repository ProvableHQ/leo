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
use leo_errors::{PackageError, Result};

use indexmap::IndexMap;
use serde::Deserialize;
use std::{
    borrow::Cow,
    collections::HashMap,
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    path::Path,
};

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

impl Manifest {
    pub fn new(package_name: &str, author: Option<String>) -> Result<Self> {
        Ok(Self {
            project: Package::new(package_name)?,
            remote: author.map(|author| Remote { author }),
            dependencies: Some(IndexMap::<String, Dependency>::new()),
        })
    }

    pub fn filename() -> String {
        MANIFEST_FILENAME.to_string()
    }

    pub fn exists_at(path: &Path) -> bool {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(MANIFEST_FILENAME);
        }
        path.exists()
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

    pub fn write_to(self, path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(MANIFEST_FILENAME);
        }

        let mut file =
            File::create(&path).map_err(|e| PackageError::failed_to_create_manifest_file(MANIFEST_FILENAME, e))?;

        file.write_all(self.template().as_bytes())
            .map_err(PackageError::io_error_manifest_file)?;
        Ok(())
    }

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

impl TryFrom<&Path> for Manifest {
    type Error = PackageError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(MANIFEST_FILENAME);
        }

        let mut file =
            File::open(path.clone()).map_err(|e| PackageError::failed_to_open_manifest_file(MANIFEST_FILENAME, e))?;

        let size = file
            .metadata()
            .map_err(|e| PackageError::failed_to_get_manifest_metadata_file(MANIFEST_FILENAME, e))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);

        file.read_to_string(&mut buffer)
            .map_err(|e| PackageError::failed_to_read_manifest_file(MANIFEST_FILENAME, e))?;

        // Determine if the old remote format is being used, and update to new convention

        let mut old_remote_format: Option<&str> = None;
        let mut new_remote_format_exists = false;

        // Toml file adhering to the new format.
        let mut final_toml = "".to_owned();

        // New Toml file format that should be written based on feature flags.
        let mut refactored_toml = "".to_owned();

        // Read each individual line of the toml file
        for line in buffer.lines() {
            // Determine if the old remote format is being used
            if line.starts_with("remote") {
                let remote = line
                    .split('=') // Split the line as 'remote' = '"{author}/{package_name}"'
                    .nth(1).unwrap(); // Fetch just '"{author}/{package_name}"'
                old_remote_format = Some(remote);

                // Retain the old remote format if the `manifest_refactor_remote` is not enabled
                if cfg!(not(feature = "manifest_refactor_remote")) {
                    refactored_toml += line;
                    refactored_toml += "\n";
                }
                continue;
            }

            // Determine if the new remote format is being used
            if line.starts_with("[remote]") {
                new_remote_format_exists = true;
            }

            // If the old project format is being being used, update the toml file
            // to use the new format instead.
            if line.starts_with("[package]") {
                final_toml += "[project]";

                // Refactor the old project format if the `manifest_refactor_project` is enabled
                match cfg!(feature = "manifest_refactor_project") {
                    true => refactored_toml += "[project]",
                    false => refactored_toml += line,
                }
            } else {
                final_toml += line;
                refactored_toml += line;
            }

            final_toml += "\n";
            refactored_toml += "\n";
        }

        // Update the remote format
        if let Some(old_remote) = old_remote_format {
            // If both the old remote and new remote are missing,
            // then skip appending the new remote, just keep the old remote.
            if !new_remote_format_exists {
                // Fetch the author from the old remote.
                let remote_author = old_remote
                    .split('/') // Split the old remote as '"{author}' and '{package_name}"'
                    .nth(0).unwrap() // Fetch just the '"{author}'
                    .replace(['\"', ' '], ""); // Remove the quotes from the author string

                // Construct the new remote section.
                let new_remote = format!(
                    r#"
[remote]
author = "{author}"
"#,
                    author = remote_author
                );

                // Append the new remote to the bottom of the manifest file.
                final_toml += &new_remote;

                // Add the new remote format if the `manifest_refactor_remote` is enabled
                if cfg!(feature = "manifest_refactor_remote") {
                    refactored_toml += &new_remote;
                }
            }
        }

        // Rewrite the toml file if it has been updated
        if buffer != refactored_toml {
            let mut file =
                File::create(&path).map_err(|e| PackageError::failed_to_create_manifest_file(MANIFEST_FILENAME, e))?;

            file.write_all(refactored_toml.as_bytes())
                .map_err(|e| PackageError::failed_to_write_manifest_file(MANIFEST_FILENAME, e))?;
        }

        // Read the toml file
        toml::from_str(&final_toml).map_err(|e| PackageError::failed_to_parse_manifest_file(MANIFEST_FILENAME, e))
    }
}
