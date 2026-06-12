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
use leo_package::{Location, Lock, Manifest, Workspace};

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

        // Names of removed git dependencies, whose `leo.lock` pins are pruned below.
        let mut removed_git_names: Vec<String> = Vec::new();

        if self.all {
            removed_git_names
                .extend(dependencies.iter().filter(|dep| dep.location == Location::Git).map(|dep| dep.name.clone()));
            *dependencies = Vec::new();
        } else {
            // Accept both `math_lib` and `math_lib.aleo` on the command line.
            // Program deps are stored with `.aleo`; library deps are stored without it.
            // Match whichever form appears in the manifest.
            let raw = self.name.unwrap();
            let alt =
                if raw.ends_with(".aleo") { raw.trim_end_matches(".aleo").to_string() } else { format!("{raw}.aleo") };
            let name = dependencies
                .iter()
                .find_map(|dep| if dep.name == raw || dep.name == alt { Some(dep.name.clone()) } else { None })
                .unwrap_or(alt); // fall back to the .aleo form for the error message

            let original_len = dependencies.len();
            for dependency in dependencies.iter() {
                if dependency.name == name {
                    match dependency.location {
                        leo_package::Location::Local | leo_package::Location::Test => tracing::warn!(
                            "✅ Successfully removed the local dependency {} with path {}.",
                            dependency.name,
                            dependency.path.as_ref().map(|p| p.display().to_string()).unwrap_or_default()
                        ),
                        leo_package::Location::Workspace => {
                            tracing::warn!("✅ Successfully removed the workspace dependency {}.", dependency.name)
                        }
                        leo_package::Location::Git => {
                            removed_git_names.push(dependency.name.clone());
                            tracing::warn!("✅ Successfully removed the git dependency {}.", dependency.name)
                        }
                        leo_package::Location::Network => {
                            tracing::warn!("✅ Successfully removed the network dependency {}.", dependency.name)
                        }
                    }
                }
            }

            dependencies.retain(|dep| dep.name != name);

            if dependencies.len() == original_len {
                return Err(crate::errors::dependency_not_found(name).into());
            }
        }

        manifest.write_to_file(&manifest_path)?;

        // Prune the removed git dependencies' pins so the lock doesn't accumulate dead entries.
        if !removed_git_names.is_empty() {
            let lock_dir = Workspace::discover_root(&path)?.unwrap_or_else(|| path.clone());
            let mut lock = Lock::read(&lock_dir);
            for name in &removed_git_names {
                lock.remove_name(name);
            }
            lock.write(&lock_dir)?;
        }

        Ok(())
    }
}
