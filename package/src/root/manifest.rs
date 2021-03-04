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

use crate::errors::ManifestError;
use crate::package::Package;

use serde::Deserialize;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub const MANIFEST_FILENAME: &str = "Leo.toml";

#[derive(Clone, Deserialize)]
pub struct Remote {
    pub author: String,
}

#[derive(Deserialize)]
pub struct Manifest {
    pub project: Package,
    pub remote: Option<Remote>,
}

impl Manifest {
    pub fn new(package_name: &str) -> Result<Self, ManifestError> {
        Ok(Self {
            project: Package::new(package_name)?,
            remote: None,
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

    pub fn get_package_license(&self) -> Option<String> {
        self.project.license.clone()
    }

    pub fn get_package_remote(&self) -> Option<Remote> {
        self.remote.clone()
    }

    pub fn write_to(self, path: &Path) -> Result<(), ManifestError> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(MANIFEST_FILENAME);
        }

        let mut file = File::create(&path).map_err(|error| ManifestError::Creating(MANIFEST_FILENAME, error))?;
        file.write_all(self.template().as_bytes())
            .map_err(|error| ManifestError::Writing(MANIFEST_FILENAME, error))
    }

    fn template(&self) -> String {
        format!(
            r#"[project]
name = "{name}"
version = "0.1.0"
description = "The {name} package"
license = "MIT"

[remote]
author = "[AUTHOR]" # Add your Aleo Package Manager username, team's name, or organization's name.
"#,
            name = self.project.name
        )
    }
}

impl TryFrom<&Path> for Manifest {
    type Error = ManifestError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            path.to_mut().push(MANIFEST_FILENAME);
        }

        let mut file = File::open(path.clone()).map_err(|error| ManifestError::Opening(MANIFEST_FILENAME, error))?;
        let size = file
            .metadata()
            .map_err(|error| ManifestError::Metadata(MANIFEST_FILENAME, error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| ManifestError::Reading(MANIFEST_FILENAME, error))?;

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
                    .collect::<Vec<&str>>()[1]; // Fetch just '"{author}/{package_name}"'
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
                    .collect::<Vec<&str>>()[0] // Fetch just the '"{author}'
                    .replace(&['\"', ' '][..], ""); // Remove the quotes from the author string

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
            let mut file = File::create(&path).map_err(|error| ManifestError::Creating(MANIFEST_FILENAME, error))?;
            file.write_all(refactored_toml.as_bytes())
                .map_err(|error| ManifestError::Writing(MANIFEST_FILENAME, error))?;
        }

        // Read the toml file
        toml::from_str(&final_toml).map_err(|error| ManifestError::Parsing(MANIFEST_FILENAME, error))
    }
}
