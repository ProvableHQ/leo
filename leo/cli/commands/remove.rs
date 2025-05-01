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

        let manifest_path = path.join(leo_package::MANIFEST_FILENAME);
        let mut manifest = Manifest::read_from_file(&manifest_path)?;

        let mut dependencies = manifest.dependencies.unwrap_or_default();

        if self.all {
            dependencies = Vec::new();
        } else {
            let name =
                self.name.map(|name| if name.ends_with(".aleo") { name } else { format!("{name}.aleo") }).unwrap();
            let original_len = dependencies.len();
            let mut new_deps = Vec::with_capacity(original_len);
            for dependency in dependencies.into_iter() {
                if dependency.name == name {
                    if let Some(local_path) = &dependency.path {
                        tracing::warn!(
                            "✅ Successfully removed the local dependency {} from path {}.",
                            dependency.name,
                            local_path.display()
                        );
                    } else {
                        tracing::warn!("✅ Successfully removed the network dependency {}.", dependency.name,);
                    }
                } else {
                    new_deps.push(dependency);
                }
            }
            if new_deps.len() == original_len {
                return Err(PackageError::dependency_not_found(name).into());
            }
            dependencies = new_deps;
        }

        manifest.dependencies = Some(dependencies);

        manifest.write_to_file(&manifest_path)?;

        Ok(())
    }
}
