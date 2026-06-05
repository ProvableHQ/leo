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
        Ok(manifest)
    }

    fn validate_dependencies(&self) -> Result<(), Backtraced> {
        for dependency in self.dependencies.iter().flatten().chain(self.dev_dependencies.iter().flatten()) {
            dependency.validate_manifest_shape()?;
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
        match self.location {
            Location::Network => {
                if self.path.is_some() {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &self.name,
                        "`network` dependencies cannot specify `path`",
                    ));
                }
            }
            Location::Local => {
                if self.path.is_none() {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &self.name,
                        "`local` dependencies must specify `path`",
                    ));
                }
                if self.edition.is_some() {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &self.name,
                        "`local` dependencies cannot specify `edition`",
                    ));
                }
            }
            Location::Workspace => {
                if self.path.is_some() {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &self.name,
                        "`workspace` dependencies cannot specify `path`",
                    ));
                }
                if self.edition.is_some() {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &self.name,
                        "`workspace` dependencies cannot specify `edition`",
                    ));
                }
            }
            Location::Test => {
                if self.path.is_none() {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &self.name,
                        "`test` dependencies must specify `path`",
                    ));
                }
                if self.edition.is_some() {
                    return Err(crate::errors::invalid_manifest_dependency(
                        &self.name,
                        "`test` dependencies cannot specify `edition`",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        process,
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    static NEXT_TEST_DIR_ID: AtomicU64 = AtomicU64::new(0);

    fn manifest_json(dependencies: &str, dev_dependencies: &str) -> String {
        format!(
            r#"{{
  "program": "test.aleo",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "dependencies": {dependencies},
  "dev_dependencies": {dev_dependencies}
}}"#
        )
    }

    fn read_manifest(contents: &str) -> Result<Manifest, Backtraced> {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let sequence = NEXT_TEST_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("leo-manifest-test-{}-{nanos}-{sequence}", process::id()));
        fs::create_dir(&dir).unwrap();
        let path = dir.join(MANIFEST_FILENAME);
        fs::write(&path, contents).unwrap();
        let result = Manifest::read_from_file(&path);
        fs::remove_dir_all(dir).unwrap();
        result
    }

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
