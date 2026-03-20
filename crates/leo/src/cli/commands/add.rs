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
    #[clap(
        short = 'l',
        long,
        help = "Local path for the dependency (program or library, auto-detected).",
        group = "source"
    )]
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

        let current_is_library = !manifest.program.ends_with(".aleo");

        // Normalize a program dep name to always carry the `.aleo` suffix, and validate it.
        let normalize_program_name = |raw: &str| -> Result<String> {
            let name = if raw.ends_with(".aleo") { raw.to_string() } else { format!("{raw}.aleo") };
            if !leo_package::is_valid_program_name(&name) {
                return Err(CliError::invalid_package_name("program", name).into());
            }
            Ok(name)
        };

        // Determine dependency name, location, and path.
        let (name, location, dep_path) = if let Some(local_path) = &self.source.local {
            // Auto-detect whether the local dep is a library or a program by reading its manifest.
            let dep_manifest_path = local_path.join(leo_package::MANIFEST_FILENAME);
            let dep_manifest = Manifest::read_from_file(&dep_manifest_path).map_err(|_| {
                CliError::custom(format!(
                    "Could not read `{}` — is `{}` a valid Leo package?",
                    dep_manifest_path.display(),
                    local_path.display()
                ))
            })?;

            let dep_is_library = !dep_manifest.program.ends_with(".aleo");

            // Libraries can only depend on other libraries.
            if current_is_library && !dep_is_library {
                return Err(CliError::custom("A library package can only depend on other libraries.").into());
            }

            if dep_is_library {
                // The dep is a library: the name must not carry a `.aleo` suffix.
                if self.name.ends_with(".aleo") {
                    return Err(CliError::custom(format!(
                        "`{}` ends with `.aleo` but the package at `{}` is a library, not a program.",
                        self.name,
                        local_path.display()
                    ))
                    .into());
                }
                if !leo_package::is_valid_library_name(&self.name) {
                    return Err(CliError::invalid_package_name("library", &self.name).into());
                }
                // Confirm that src/lib.leo exists — the manifest says it's a library,
                // so a missing lib.leo means the package is incomplete.
                let lib_leo = local_path.join("src").join(leo_package::LIB_FILENAME);
                if !lib_leo.exists() {
                    return Err(CliError::custom(format!(
                        "The package at `{}` has a library manifest but is missing `src/{}`.",
                        local_path.display(),
                        leo_package::LIB_FILENAME,
                    ))
                    .into());
                }
                (self.name.clone(), Location::Local, Some(local_path.clone()))
            } else {
                // The dep is a program: normalize the name to include `.aleo`.
                (normalize_program_name(&self.name)?, Location::Local, Some(local_path.clone()))
            }
        } else {
            // Network or edition dependency — must be a program, not a library.
            if current_is_library {
                return Err(CliError::custom(
                    "A library package can only depend on other libraries. Use `--local <path>` to add a library dependency.",
                )
                .into());
            }
            (normalize_program_name(&self.name)?, Location::Network, None)
        };

        let new_dependency =
            Dependency { name: name.clone(), location, path: dep_path.clone(), edition: self.source.edition };

        // Choose dev or normal dependencies.
        let deps = if self.dev { &mut manifest.dev_dependencies } else { &mut manifest.dependencies };

        if let Some(existing) = deps.get_or_insert_default().iter_mut().find(|dep| dep.name == new_dependency.name) {
            if let Some(existing_path) = &existing.path {
                tracing::warn!(
                    "⚠️ Dependency `{name}` already exists as a local dependency at `{}`. Overwriting.",
                    existing_path.display()
                );
            } else {
                tracing::warn!("⚠️ Dependency `{name}` already exists as a network dependency. Overwriting.");
            }
            *existing = new_dependency;
        } else {
            deps.as_mut().unwrap().push(new_dependency);

            match dep_path {
                Some(p) => tracing::info!("✅ Added local dependency `{name}` at path `{}`.", p.display()),
                None => tracing::info!("✅ Added network dependency `{name}`."),
            }
        }

        manifest.write_to_file(manifest_path)?;

        Ok(())
    }
}
