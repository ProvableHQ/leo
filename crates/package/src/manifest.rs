// Copyright (C) 2019-2026 Provable Inc.
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

use crate::*;

use leo_errors::Backtraced;

use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MANIFEST_FILENAME: &str = "program.json";

/// Struct representation of program's `program.json` specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub program: String,
    pub version: String,
    pub description: String,
    pub license: String,
    #[serde(default = "current_version")]
    pub leo: String,
    pub dependencies: Option<Vec<Dependency>>,
    pub dev_dependencies: Option<Vec<Dependency>>,
    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub no_std: bool,
}

impl Manifest {
    /// Write the manifest to the given `path` as a JSON string.
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Backtraced> {
        // Serialize the manifest to a JSON string.
        let mut contents = serde_json::to_string_pretty(&self)
            .map_err(|err| crate::errors::failed_to_serialize_manifest_file(path.as_ref().display(), err))?;

        // The seralized string doesn't end in a newline.
        contents.push('\n');

        // Write the manifest to the file.
        std::fs::write(path, contents).map_err(crate::errors::failed_to_write_manifest)
    }

    /// Read and validate a Manifest from the given JSON file.
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Backtraced> {
        // Read the manifest file.
        let contents = std::fs::read_to_string(&path)
            .map_err(|_| crate::errors::failed_to_load_package(path.as_ref().display()))?;
        // Deserialize the manifest.
        let manifest: Self = serde_json::from_str(&contents)
            .map_err(|err| crate::errors::failed_to_deserialize_manifest_file(path.as_ref().display(), err))?;
        manifest.validate_dependencies()?;
        manifest.validate_reserved_names()?;
        Ok(manifest)
    }

    fn validate_dependencies(&self) -> Result<(), Backtraced> {
        for dependency in self.dependencies.iter().flatten().chain(self.dev_dependencies.iter().flatten()) {
            dependency.validate_manifest_shape()?;
        }
        Ok(())
    }

    fn validate_reserved_names(&self) -> Result<(), Backtraced> {
        let program_bare = crate::bare_unit_name(&self.program);
        if program_bare == "std" {
            return Err(crate::errors::reserved_std_name("program name"));
        }
        for dep in self.dependencies.iter().flatten().chain(self.dev_dependencies.iter().flatten()) {
            let dep_bare = crate::bare_unit_name(&dep.name);
            if dep_bare == "std" {
                return Err(crate::errors::reserved_std_name("dependency name"));
            }
        }
        Ok(())
    }
}

// Returns the current version of Leo.
fn current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

impl Dependency {
    fn validate_manifest_shape(&self) -> Result<(), Backtraced> {
        // Reject any field that has no meaning for the dependency's location, so a stray field
        // (e.g. `git` on a `local` dependency) errors instead of being silently ignored.
        let invalid = |reason: String| Err(crate::errors::invalid_manifest_dependency(&self.name, reason));
        let location = match self.location {
            Location::Network => "network",
            Location::Local => "local",
            Location::Workspace => "workspace",
            Location::Test => "test",
            Location::Git => "git",
        };

        let path_required = matches!(self.location, Location::Local | Location::Test);
        if path_required && self.path.is_none() {
            return invalid(format!("`{location}` dependencies must specify `path`"));
        }
        if !path_required && self.path.is_some() {
            return invalid(format!("`{location}` dependencies cannot specify `path`"));
        }
        if self.location != Location::Network && self.edition.is_some() {
            return invalid(format!("`{location}` dependencies cannot specify `edition`"));
        }
        if self.location != Location::Git && self.git.is_some() {
            return invalid(format!("`{location}` dependencies cannot specify `git`"));
        }

        if self.location == Location::Git {
            let Some(git) = &self.git else {
                return invalid("`git` dependencies must specify `git`".to_string());
            };
            if let Err(reason) = git.reference() {
                return invalid(reason.to_string());
            }
            // The name is matched against package manifests in the checkout, so validate it at
            // this trust boundary.
            if !crate::is_valid_package_name(crate::bare_unit_name(&self.name)) {
                return invalid("a git dependency name must be a valid program or library name".to_string());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::{manifest_json, read_manifest};

    #[test]
    fn manifest_rejects_network_dependency_with_path() {
        let err =
            read_manifest(&manifest_json(r#"[{"name":"foo.aleo","location":"network","path":"../foo"}]"#, "null"))
                .unwrap_err();

        assert!(err.to_string().contains("invalid dependency `foo.aleo`"));
        assert!(err.to_string().contains("`network` dependencies cannot specify `path`"));
    }

    #[test]
    fn manifest_rejects_local_dependency_without_path() {
        let err = read_manifest(&manifest_json(r#"[{"name":"foo.aleo","location":"local"}]"#, "null")).unwrap_err();

        assert!(err.to_string().contains("invalid dependency `foo.aleo`"));
        assert!(err.to_string().contains("`local` dependencies must specify `path`"));
    }

    #[test]
    fn manifest_rejects_invalid_dev_dependency_shape() {
        let err =
            read_manifest(&manifest_json("null", r#"[{"name":"foo.aleo","location":"workspace","path":"../foo"}]"#))
                .unwrap_err();

        assert!(err.to_string().contains("invalid dependency `foo.aleo`"));
        assert!(err.to_string().contains("`workspace` dependencies cannot specify `path`"));
    }

    #[test]
    fn manifest_accepts_location_specific_dependency_fields() {
        let manifest = read_manifest(&manifest_json(
            r#"[
  {"name":"network_dep.aleo","location":"network","edition":1},
  {"name":"local_dep.aleo","location":"local","path":"../local_dep"},
  {"name":"workspace_dep.aleo","location":"workspace"}
]"#,
            "null",
        ))
        .unwrap();

        assert_eq!(manifest.dependencies.unwrap().len(), 3);
    }
}
