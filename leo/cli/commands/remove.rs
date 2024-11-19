// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use super::*;
use leo_retriever::{Dependency, Manifest};

/// Remove a dependency from the current package.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Leo Team <leo@provable.com>", version)]
pub struct LeoRemove {
    #[clap(
        name = "NAME",
        help = "The dependency name. Ex: `credits.aleo` or `credits`.",
        required_unless_present = "all"
    )]
    pub(crate) name: Option<String>,

    #[clap(short = 'd', long, help = "Whether the dependency is a dev dependency", default_value = "false")]
    pub(crate) dev: bool,

    #[clap(long, help = "Clear all previous dependencies.", default_value = "false")]
    pub(crate) all: bool,
}

impl Command for LeoRemove {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let path = context.dir()?;

        // TODO: Dedup with Add Command. Requires merging utils/retriever/program_context with leo/package as both involve modifying the manifest.
        // Deserialize the manifest.
        let program_data: String = std::fs::read_to_string(path.join("program.json"))
            .map_err(|err| PackageError::failed_to_read_file(path.to_str().unwrap(), err))?;
        let manifest: Manifest = serde_json::from_str(&program_data)
            .map_err(|err| PackageError::failed_to_deserialize_manifest_file(path.to_str().unwrap(), err))?;

        // Destructure the manifest.
        let Manifest { program, version, description, license, dependencies, dev_dependencies } = manifest;

        // Add the dependency to the appropriate section.
        let (dependencies, dev_dependencies) = if self.all {
            if self.dev { (Some(Vec::new()), dev_dependencies) } else { (dependencies, Some(Vec::new())) }
        } else {
            // Note that this unwrap is safe since `name` is required if `all` is `false`.
            let name = self.name.unwrap();
            if self.dev {
                (dependencies, remove_dependency(dev_dependencies, name)?)
            } else {
                (remove_dependency(dependencies, name)?, dev_dependencies)
            }
        };

        // Update the manifest file.
        let new_manifest = Manifest::new(
            program.as_str(),
            version.as_str(),
            description.as_str(),
            license.as_str(),
            dependencies,
            dev_dependencies,
        );
        new_manifest.write_to_dir(&path)?;

        Ok(())
    }
}

// A helper function to remove a dependency from either the `dependencies` or `dev_dependencies` section of the manifest.
fn remove_dependency(dependencies: Option<Vec<Dependency>>, name: String) -> Result<Option<Vec<Dependency>>> {
    // Remove the dependency from the list, returning an error if it was not found.
    match dependencies {
        None => Err(PackageError::dependency_not_found(name).into()),
        Some(mut dependencies) => {
            if let Some(index) = dependencies.iter().position(|dep| dep.name() == &name) {
                dependencies.remove(index);
                Ok(Some(dependencies))
            } else {
                Err(PackageError::dependency_not_found(name).into())
            }
        }
    }
}
