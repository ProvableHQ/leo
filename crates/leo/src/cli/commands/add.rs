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
use leo_package::{Dependency, GitReference, GitSource, Location, Lock, Manifest, Workspace};
use std::path::{Path, PathBuf};

/// Add a new on-chain, local, or git dependency to the current package.
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Leo Team <leo@provable.com>", version)]
pub struct LeoAdd {
    #[clap(name = "NAME", help = "The dependency name. Ex: `credits.aleo` or `credits`.")]
    pub(crate) name: String,

    #[clap(flatten)]
    pub(crate) source: DependencySource,

    #[clap(flatten)]
    pub(crate) git_ref: GitRef,

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

    #[clap(short = 'w', long, help = "Depend on another member of the enclosing workspace.", group = "source")]
    pub(crate) workspace: bool,

    #[clap(
        short = 'g',
        long,
        help = "Git repository URL for the dependency (program or library, auto-detected).",
        group = "source"
    )]
    pub(crate) git: Option<String>,
}

/// The git reference to track. Only meaningful with `--git`; at most one may be set.
#[derive(Parser, Debug)]
#[group(required = false, multiple = false)]
pub struct GitRef {
    #[clap(long, help = "Track a git branch (requires `--git`).", group = "gitref", requires = "git")]
    pub(crate) branch: Option<String>,

    #[clap(long, help = "Pin to a git tag (requires `--git`).", group = "gitref", requires = "git")]
    pub(crate) tag: Option<String>,

    #[clap(long, help = "Pin to a git revision (requires `--git`).", group = "gitref", requires = "git")]
    pub(crate) rev: Option<String>,
}

/// Normalize a program dep name to always carry the `.aleo` suffix, and validate it.
fn normalize_program_name(raw: &str) -> Result<String> {
    let name = if raw.ends_with(".aleo") { raw.to_string() } else { format!("{raw}.aleo") };
    if !leo_package::is_valid_program_name(&name) {
        return Err(crate::errors::invalid_package_name("program", name).into());
    }
    Ok(name)
}

/// Detect whether `located` is a library or program, validate `raw_name`, and return the canonical
/// dep name (programs carry `.aleo`, libraries don't). `origin` is used in error messages.
fn classify_and_validate_name(
    raw_name: &str,
    current_is_library: bool,
    located: &Path,
    origin: impl std::fmt::Display,
) -> Result<String> {
    let dep_is_library = if located.extension().and_then(|e| e.to_str()) == Some("aleo") && located.is_file() {
        false
    } else {
        let dep_manifest_path = located.join(leo_package::MANIFEST_FILENAME);
        let dep_manifest = Manifest::read_from_file(&dep_manifest_path).map_err(|_| {
            crate::errors::custom(format!(
                "Could not read `{}` — is `{origin}` a valid Leo package?",
                dep_manifest_path.display()
            ))
        })?;
        !dep_manifest.program.ends_with(".aleo")
    };

    if current_is_library && !dep_is_library {
        return Err(crate::errors::custom("A library package can only depend on other libraries.").into());
    }

    if !dep_is_library {
        return normalize_program_name(raw_name);
    }

    if raw_name.ends_with(".aleo") {
        return Err(crate::errors::custom(format!(
            "`{raw_name}` ends with `.aleo` but the package at `{origin}` is a library, not a program.",
        ))
        .into());
    }
    if !leo_package::is_valid_library_name(raw_name) {
        return Err(crate::errors::invalid_package_name("library", raw_name).into());
    }
    // A library manifest without src/lib.leo is an incomplete package.
    let lib_leo = located.join("src").join(leo_package::LIB_FILENAME);
    if !lib_leo.exists() {
        return Err(crate::errors::custom(format!(
            "The package at `{origin}` has a library manifest but is missing `src/{}`.",
            leo_package::LIB_FILENAME,
        ))
        .into());
    }
    Ok(raw_name.to_string())
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

        // Determine dependency name, location, and path.
        let (name, location, dep_path) = if let Some(local_path) = &self.source.local {
            let name = classify_and_validate_name(&self.name, current_is_library, local_path, local_path.display())?;
            (name, Location::Local, Some(local_path.clone()))
        } else if self.source.workspace {
            // Workspace dependency - validate that an enclosing workspace exists and the member is listed.
            let ws = leo_package::Workspace::discover(&path)?.ok_or_else(|| {
                crate::errors::custom(
                    "Cannot add a workspace dependency: no `workspace.json` found in any parent directory.",
                )
            })?;
            let name = normalize_program_name(&self.name)?;
            if ws.find_member(&name).is_none() {
                return Err(crate::errors::custom(format!(
                    "No workspace member named `{name}` found. Check the `members` list in `workspace.json`.",
                ))
                .into());
            }
            (name, Location::Workspace, None)
        } else if let Some(url) = &self.source.git {
            // Validate the name first, since it becomes part of the checkout path.
            if !leo_package::is_valid_package_name(leo_package::bare_unit_name(&self.name)) {
                return Err(crate::errors::custom(format!("`{}` is not a valid dependency name.", self.name)).into());
            }

            // Same derivation/validation the build uses, so `leo add` and the build agree.
            let reference = match GitReference::from_opts(&self.git_ref.branch, &self.git_ref.tag, &self.git_ref.rev) {
                Ok(reference) => reference,
                Err(reason) => return Err(crate::errors::custom(reason).into()),
            };

            let home = context.home()?;
            let (checkout, commit) = leo_package::git::resolve(&home, &self.name, url, &reference, None, false)?;

            let located = leo_package::find_in_checkout(&checkout, &self.name).map_err(|_| {
                crate::errors::custom(format!(
                    "Could not find a Leo package named `{}` in `{url}`. Check the name and the git reference.",
                    self.name
                ))
            })?;
            let name = classify_and_validate_name(&self.name, current_is_library, &located, url)?;

            // Pin the resolved commit so the next build reuses this checkout instead of re-fetching.
            let lock_dir = Workspace::discover_root(&path)?.unwrap_or_else(|| path.clone());
            let mut lock = Lock::read(&lock_dir);
            lock.record(name.clone(), url.clone(), reference.lock_string(), commit);
            lock.write(&lock_dir)?;

            (name, Location::Git, None)
        } else {
            // Network or edition dependency - must be a program, not a library.
            if current_is_library {
                return Err(crate::errors::custom(
                    "A library package can only depend on other libraries. Use `--local <path>` to add a library dependency.",
                )
                .into());
            }
            (normalize_program_name(&self.name)?, Location::Network, None)
        };

        let new_dependency = Dependency {
            name: name.clone(),
            location,
            path: dep_path.clone(),
            edition: self.source.edition,
            git: self.source.git.as_ref().map(|url| GitSource {
                url: url.clone(),
                branch: self.git_ref.branch.clone(),
                tag: self.git_ref.tag.clone(),
                rev: self.git_ref.rev.clone(),
            }),
        };

        // Choose dev or normal dependencies.
        let deps = if self.dev { &mut manifest.dev_dependencies } else { &mut manifest.dependencies };

        if let Some(existing) = deps.get_or_insert_default().iter_mut().find(|dep| dep.name == new_dependency.name) {
            match existing.location {
                Location::Local => tracing::warn!(
                    "⚠️ Dependency `{name}` already exists as a local dependency at `{}`. Overwriting.",
                    existing.path.as_ref().map(|p| p.display().to_string()).unwrap_or_default()
                ),
                Location::Workspace => {
                    tracing::warn!("⚠️ Dependency `{name}` already exists as a workspace dependency. Overwriting.")
                }
                Location::Git => {
                    tracing::warn!("⚠️ Dependency `{name}` already exists as a git dependency. Overwriting.")
                }
                _ => tracing::warn!("⚠️ Dependency `{name}` already exists as a network dependency. Overwriting."),
            }
            *existing = new_dependency;
        } else {
            deps.as_mut().unwrap().push(new_dependency);

            match location {
                Location::Local => tracing::info!(
                    "✅ Added local dependency `{name}` at path `{}`.",
                    dep_path.as_ref().map(|p| p.display().to_string()).unwrap_or_default()
                ),
                Location::Workspace => tracing::info!("✅ Added workspace dependency `{name}`."),
                Location::Git => tracing::info!("✅ Added git dependency `{name}`."),
                _ => tracing::info!("✅ Added network dependency `{name}`."),
            }
        }

        manifest.write_to_file(manifest_path)?;

        Ok(())
    }
}
