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
use leo_retriever::{Dependency, Location, Manifest, NetworkName};
use std::path::PathBuf;

/// Add a new on-chain or local dependency to the current package.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Aleo Team <hello@aleo.org>", version)]
pub struct Add {
    #[clap(name = "NAME", help = "The dependency name. Ex: `credits.aleo` or `credits`.")]
    pub(crate) name: String,

    #[clap(short = 'l', long, help = "Path to local dependency")]
    pub(crate) local: Option<PathBuf>,

    #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
    pub(crate) network: String,

    #[clap(short = 'c', long, help = "Clear all previous dependencies.", default_value = "false")]
    pub(crate) clear: bool,
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

        // Add dependency section to manifest if it doesn't exist.
        let mut dependencies = match (self.clear, manifest.dependencies()) {
            (false, Some(ref dependencies)) => dependencies
                .iter()
                .filter_map(|dependency| {
                    // Overwrite old dependencies of the same name.
                    if dependency.name() == &name {
                        let msg = match (dependency.path(), dependency.network()) {
                            (Some(local_path), _) => {
                                format!("local dependency at path `{}`", local_path.to_str().unwrap().replace('\"', ""))
                            }
                            (_, Some(network)) => {
                                format!("network dependency from `{}`", network)
                            }
                            _ => "git dependency".to_string(),
                        };
                        tracing::warn!("⚠️  Program `{name}` already exists as a {msg}. Overwriting.");
                        None
                    } else if self.local.is_some() && &self.local == dependency.path() {
                        // Overwrite old dependencies at the same local path.
                        tracing::warn!(
                            "⚠️  Path `{}` already exists as the location for local dependency `{}`. Overwriting.",
                            self.local.clone().unwrap().to_str().unwrap().replace('\"', ""),
                            dependency.name()
                        );
                        None
                    } else {
                        Some(dependency.clone())
                    }
                })
                .collect(),
            _ => Vec::new(),
        };

        // Add new dependency to the manifest.
        dependencies.push(match self.local {
            Some(local_path) => {
                tracing::info!(
                    "✅ Added local dependency to program `{name}` at path `{}`.",
                    local_path.to_str().unwrap().replace('\"', "")
                );
                Dependency::new(name, Location::Local, None, Some(local_path))
            }
            None => {
                tracing::info!("✅ Added network dependency to program `{name}` from network `{}`.", self.network);
                Dependency::new(name, Location::Network, Some(NetworkName::try_from(self.network.as_str())?), None)
            }
        });

        // Update the manifest file.
        let new_manifest = Manifest::new(
            manifest.program(),
            manifest.version(),
            manifest.description(),
            manifest.license(),
            Some(dependencies),
        );
        new_manifest.write_to_dir(&path)?;

        Ok(())
    }
}
