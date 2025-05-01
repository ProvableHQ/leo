// Copyright (C) 2019-2025 Provable Inc.
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
use leo_package::{Dependency, Location, Manifest, NetworkName};
use std::path::PathBuf;

/// Add a new on-chain or local dependency to the current package.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Leo Team <leo@provable.com>", version)]
pub struct LeoAdd {
    #[clap(name = "NAME", help = "The dependency name. Ex: `credits.aleo` or `credits`.")]
    pub(crate) name: String,

    #[clap(short = 'l', long, help = "Path to local dependency")]
    pub(crate) local: Option<PathBuf>,

    #[clap(short = 'n', long, help = "Name of the network to use", default_value = "testnet")]
    pub(crate) network: String,

    #[clap(short = 'c', long, help = "Clear all previous dependencies.", default_value = "false")]
    pub(crate) clear: bool,
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

        let manifest_path = path.join(leo_package::MANIFEST_FILENAME);
        let mut manifest = Manifest::read_from_file(&manifest_path)?;

        // Make sure the program name is valid.
        // Allow both `credits.aleo` and `credits` syntax.
        let name = if self.name.ends_with(".aleo") { self.name.clone() } else { format!("{}.aleo", self.name) };

        if !leo_package::is_valid_aleo_name(&name) {
            return Err(CliError::invalid_program_name(name).into());
        }

        let network: NetworkName = self.network.parse()?;

        let new_dependency = Dependency {
            name: name.clone(),
            location: if self.local.is_some() { Location::Local } else { Location::Network },
            network: if self.local.is_some() { None } else { Some(network) },
            path: self.local.clone(),
        };

        if let Some(matched_dep) =
            manifest.dependencies.get_or_insert_default().iter_mut().find(|dep| dep.name == new_dependency.name)
        {
            if let Some(path) = &matched_dep.path {
                tracing::warn!(
                    "⚠️  Program `{name}` already exists as a local dependency at `{}`. Overwriting.",
                    path.display()
                );
            } else {
                tracing::warn!("⚠️  Program `{name}` already exists as a network dependency. Overwriting.");
            }
            *matched_dep = new_dependency;
        } else {
            manifest.dependencies.as_mut().unwrap().push(new_dependency);
            if let Some(path) = self.local.as_ref() {
                tracing::info!("✅ Added local dependency to program `{name}` at path `{}`.", path.display());
            } else {
                tracing::info!("✅ Added network dependency `{name}` from network `{}`.", self.network);
            }
        }

        manifest.write_to_file(manifest_path)?;

        Ok(())
    }
}
