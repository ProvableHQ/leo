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

use super::*;
use leo_package::{Dependency, Location, Manifest};
use std::path::PathBuf;

/// Add a new on-chain or local dependency to the current package.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Leo Team <leo@provable.com>", version)]
pub struct LeoAdd {
    #[clap(name = "NAME", help = "The dependency name. Ex: `credits.aleo` or `credits`.")]
    pub(crate) name: String,

    #[clap(flatten)]
    pub(crate) source: DependencySource,

    #[clap(
        short = 'c',
        long,
        hide = true,
        help = "[UNUSED] Clear all previous dependencies.",
        default_value = "false"
    )]
    pub(crate) clear: bool,

    #[clap(long, help = "This is a development dependency.", default_value = "false")]
    pub(crate) dev: bool,
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
pub struct DependencySource {
    #[clap(short = 'l', long, help = "Whether the dependency is local to the machine.", group = "source")]
    pub(crate) local: Option<PathBuf>,

    #[clap(short = 'n', long, help = "Whether the dependency is on a live network.", group = "source")]
    pub(crate) network: bool,

    #[clap(
        short = 'e',
        long,
        help = "The expected edition of the program. DO NOT USE THIS UNLESS YOU KNOW WHAT YOU ARE DOING.",
        group = "source"
    )]
    pub(crate) edition: Option<u16>,
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

        let new_dependency = Dependency {
            name: name.clone(),
            location: if self.source.local.is_some() { Location::Local } else { Location::Network },
            path: self.source.local.clone(),
            edition: self.source.edition,
        };

        let deps = if self.dev { &mut manifest.dev_dependencies } else { &mut manifest.dependencies };

        if let Some(matched_dep) = deps.get_or_insert_default().iter_mut().find(|dep| dep.name == new_dependency.name) {
            if let Some(path) = &matched_dep.path {
                tracing::warn!(
                    "⚠️ Program `{name}` already exists as a local dependency at `{}`. Overwriting.",
                    path.display()
                );
            } else {
                tracing::warn!("⚠️ Program `{name}` already exists as a network dependency. Overwriting.");
            }
            *matched_dep = new_dependency;
        } else {
            deps.as_mut().unwrap().push(new_dependency);
            if let Some(path) = self.source.local.as_ref() {
                tracing::info!("✅ Added local dependency to program `{name}` at path `{}`.", path.display());
            } else {
                tracing::info!("✅ Added network dependency `{name}` from network `{}`.", self.source.network);
            }
        }

        manifest.write_to_file(manifest_path)?;

        Ok(())
    }
}
