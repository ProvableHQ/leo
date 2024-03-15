// Copyright (C) 2019-2023 Aleo Systems Inc.
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
use leo_retriever::{Dependency, Location, Manifest, Network};
use std::path::PathBuf;

/// Clean outputs folder command
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Aleo Team <hello@aleo.org>", version)]
pub struct Add {
    #[clap(name = "NAME", help = "The dependency name")]
    pub(crate) name: String,

    #[clap(short = 'l', long, help = "Optional path to local dependency")]
    pub(crate) local: Option<PathBuf>,

    #[clap(short = 'n', long, help = "Optional name of the network to use", default_value = "testnet3")]
    pub(crate) network: String,
}

impl Command for Add {
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

        // Deserialize the manifest
        let program_data: String = std::fs::read_to_string(path.join("program.json"))
            .map_err(|err| PackageError::failed_to_read_file(path.to_str().unwrap(), err))?;
        let manifest: Manifest = serde_json::from_str(&program_data)
            .map_err(|err| PackageError::failed_to_deserialize_manifest_file(path.to_str().unwrap(), err))?;

        // Allow both `credits.aleo` and `credits` syntax
        let name = match self.name {
            name if name.ends_with(".aleo") => name,
            name => format!("{}.aleo", name),
        };

        // Add dependency section to manifest if it doesn't exist
        let mut dependencies = match manifest.dependencies() {
            Some(ref dependencies) => dependencies
                .iter()
                .filter_map(|dependency| {
                    if dependency.name() == &name {
                        println!("{} already exists as a dependency. Overwriting.", name);
                        None
                    } else {
                        Some(dependency.clone())
                    }
                })
                .collect(),
            None => Vec::new(),
        };

        // Add new dependency to manifest
        dependencies.push(match self.local {
            Some(local_path) => Dependency::new(name, Location::Local, None, Some(local_path)),
            None => Dependency::new(name, Location::Network, Some(Network::from(&self.network)), None),
        });

        // Update manifest
        let new_manifest = Manifest::new(
            manifest.program(),
            manifest.version(),
            manifest.description(),
            manifest.license(),
            Some(dependencies),
        );
        let new_manifest_data = serde_json::to_string_pretty(&new_manifest)
            .map_err(|err| PackageError::failed_to_serialize_manifest_file(path.to_str().unwrap(), err))?;
        std::fs::write(path.join("program.json"), new_manifest_data).map_err(PackageError::failed_to_write_manifest)?;

        Ok(())
    }
}
