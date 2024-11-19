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
use leo_retriever::{Dependency, Location, Manifest, NetworkName};
use std::path::PathBuf;

/// Add a new on-chain or local dependency to the current package.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Leo Team <leo@provable.com>", version)]
pub struct LeoAdd {
    #[clap(name = "NAME", help = "The dependency name. Ex: `credits.aleo` or `credits`.")]
    pub(crate) name: String,

    #[clap(short = 'l', long, help = "Path to local dependency")]
    pub(crate) local: Option<PathBuf>,

    #[clap(short = 'd', long, help = "Whether the dependency is a dev dependency", default_value = "false")]
    pub(crate) dev: bool,

    #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
    pub(crate) network: String,
}

impl Command for LeoAdd {
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

        // Deserialize the manifest.
        let program_data: String = std::fs::read_to_string(path.join("program.json"))
            .map_err(|err| PackageError::failed_to_read_file(path.to_str().unwrap(), err))?;
        let manifest: Manifest = serde_json::from_str(&program_data)
            .map_err(|err| PackageError::failed_to_deserialize_manifest_file(path.to_str().unwrap(), err))?;

        // Make sure the program name is valid.
        // Allow both `credits.aleo` and `credits` syntax.
        let name: String = match &self.name {
            name if name.ends_with(".aleo") && Package::is_aleo_name_valid(&name[0..self.name.len() - 5]) => {
                name.clone()
            }
            name if Package::is_aleo_name_valid(name) => format!("{name}.aleo"),
            name => return Err(PackageError::invalid_file_name_dependency(name).into()),
        };

        // Destructure the manifest.
        let Manifest { program, version, description, license, dependencies, dev_dependencies } = manifest;

        // Add the dependency to the appropriate section.
        let (dependencies, dev_dependencies) = if self.dev {
            (dependencies, add_dependency(dev_dependencies, name, self.local, self.network)?)
        } else {
            (add_dependency(dependencies, name, self.local, self.network)?, dev_dependencies)
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

// A helper function to add a dependency to either the `dependencies` or `dev_dependencies` section of the manifest.
fn add_dependency(
    dependencies: Option<Vec<Dependency>>,
    name: String,
    location: Option<PathBuf>,
    network: String,
) -> Result<Option<Vec<Dependency>>> {
    // Check if the dependency already exists, returning the original list if it does.
    let mut dependencies = if let Some(dependencies) = dependencies {
        if dependencies.iter().any(|dependency| dependency.name() == &name) {
            tracing::warn!(
                "⚠️  Program `{name}` already exists as a dependency. If you wish to update it, explicitly remove it using `leo remove` and add it again."
            );
            return Ok(Some(dependencies));
        }
        dependencies
    } else {
        Vec::new()
    };
    // Add the new dependency to the list.
    dependencies.push(match location {
        Some(local_path) => {
            tracing::info!(
                "✅ Added local dependency to program `{name}` at path `{}`.",
                local_path.to_str().unwrap().replace('\"', "")
            );
            Dependency::new(name, Location::Local, None, Some(local_path))
        }
        None => {
            tracing::info!("✅ Added network dependency to program `{name}` from network `{network}`.");
            Dependency::new(name, Location::Network, Some(NetworkName::try_from(network.as_str())?), None)
        }
    });
    // Return the updated list of dependencies.
    Ok(Some(dependencies))
}
