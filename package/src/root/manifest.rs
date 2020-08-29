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

use crate::errors::ManifestError;

use serde::Deserialize;
use std::{
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub const MANIFEST_FILE_NAME: &str = "Leo.toml";

#[derive(Clone, Deserialize)]
pub struct Remote {
    pub author: String,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub remote: Option<Remote>,
}

#[derive(Deserialize)]
pub struct Manifest {
    pub package: Package,
}

impl Manifest {
    pub fn new(package_name: &str) -> Self {
        Self {
            package: Package {
                name: package_name.to_owned(),
                version: "0.1.0".to_owned(),
                description: None,
                license: None,
                remote: None,
            },
        }
    }

    pub fn exists_at(path: &PathBuf) -> bool {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(MANIFEST_FILE_NAME));
        }
        path.exists()
    }

    pub fn get_package_name(&self) -> String {
        self.package.name.clone()
    }

    pub fn get_package_version(&self) -> String {
        self.package.version.clone()
    }

    pub fn get_package_description(&self) -> Option<String> {
        self.package.description.clone()
    }

    pub fn get_package_license(&self) -> Option<String> {
        self.package.license.clone()
    }

    pub fn get_package_remote(&self) -> Option<Remote> {
        self.package.remote.clone()
    }

    pub fn write_to(self, path: &PathBuf) -> Result<(), ManifestError> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(MANIFEST_FILE_NAME));
        }

        let mut file = File::create(&path).map_err(|error| ManifestError::Creating(MANIFEST_FILE_NAME, error))?;
        file.write_all(self.template().as_bytes())
            .map_err(|error| ManifestError::Writing(MANIFEST_FILE_NAME, error))
    }

    fn template(&self) -> String {
        format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
description = "The {name} package"
license = "MIT"

[remote]
author = "[AUTHOR]" # Add your Aleo Package Manager username, team's name, or organization's name.
"#,
            name = self.package.name
        )
    }
}

impl TryFrom<&PathBuf> for Manifest {
    type Error = ManifestError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let mut path = path.to_owned();
        if path.is_dir() {
            path.push(PathBuf::from(MANIFEST_FILE_NAME));
        }

        let mut file = File::open(path.clone()).map_err(|error| ManifestError::Opening(MANIFEST_FILE_NAME, error))?;
        let size = file
            .metadata()
            .map_err(|error| ManifestError::Metadata(MANIFEST_FILE_NAME, error))?
            .len() as usize;

        let mut buffer = String::with_capacity(size);
        file.read_to_string(&mut buffer)
            .map_err(|error| ManifestError::Reading(MANIFEST_FILE_NAME, error))?;

        // Determine if the old remote format is being used, and update to new convention

        let mut old_remote_format: Option<&str> = None;
        let mut new_remote_format_exists = false;

        let mut new_toml = "".to_owned();

        // Read each individual line of the toml file
        for line in buffer.lines() {
            // Determine if the old remote format is being used
            if line.starts_with("remote") {
                let remote = line
                    .split("=") // Split the line as 'remote' = '"{author}/{package_name}"'
                    .collect::<Vec<&str>>()[1]; // Fetch just '"{author}/{package_name}"'
                old_remote_format = Some(remote);
                continue;
            }

            // Determine if the new remote format is being used
            if line.starts_with("[remote]") {
                new_remote_format_exists = true;
            }
            new_toml += line;
            new_toml += "\n";
        }

        // Update the remote format
        if let Some(old_remote) = old_remote_format {
            // If both the old remote and new remote are missing,
            // then skip appending the new remote, just keep the old remote.
            if !new_remote_format_exists {
                // Fetch the author from the old remote.
                let remote_author = old_remote
                    .split("/") // Split the old remote as '"{author}' and '{package_name}"'
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
                new_toml += &new_remote;
            }
        }

        // Rewrite the toml file if it has been updated
        if buffer != new_toml {
            let mut file = File::create(&path).map_err(|error| ManifestError::Creating(MANIFEST_FILE_NAME, error))?;
            file.write_all(new_toml.as_bytes())
                .map_err(|error| ManifestError::Writing(MANIFEST_FILE_NAME, error))?;
        }

        // Read the toml file
        Ok(toml::from_str(&new_toml).map_err(|error| ManifestError::Parsing(MANIFEST_FILE_NAME, error))?)
    }
}
