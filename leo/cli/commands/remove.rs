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
use leo_package::Manifest;

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

    #[clap(
        long,
        help = "Clear all previous dependencies (or dev dependencies, if used with --dev).",
        default_value = "false"
    )]
    pub(crate) all: bool,

    #[clap(long, help = "This is a dev dependency.", default_value = "false")]
    pub(crate) dev: bool,
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

        let manifest_path = path.join(leo_package::MANIFEST_FILENAME);
        let mut manifest = Manifest::read_from_file(&manifest_path)?;

        let dependencies = if self.dev {
            if manifest.dev_dependencies.is_none() {
                manifest.dev_dependencies = Some(Vec::new())
            }
            manifest.dev_dependencies.as_mut().unwrap()
        } else {
            if manifest.dependencies.is_none() {
                manifest.dependencies = Some(Vec::new())
            }
            manifest.dependencies.as_mut().unwrap()
        };

        if self.all {
            *dependencies = Vec::new();
        } else {
            let name =
                self.name.map(|name| if name.ends_with(".aleo") { name } else { format!("{name}.aleo") }).unwrap();
            let original_len = dependencies.len();
            for dependency in dependencies.iter() {
                if dependency.name == name {
                    if let Some(local_path) = &dependency.path {
                        tracing::warn!(
                            "✅ Successfully removed the local dependency {} with path {}.",
                            dependency.name,
                            local_path.display()
                        );
                    } else {
                        tracing::warn!("✅ Successfully removed the network dependency {}.", dependency.name);
                    }
                }
            }

            dependencies.retain(|dep| dep.name != name);

            if dependencies.len() == original_len {
                return Err(PackageError::dependency_not_found(name).into());
            }
        }

        manifest.write_to_file(&manifest_path)?;

        Ok(())
    }
}
