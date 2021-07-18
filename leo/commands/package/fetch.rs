// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{
    commands::{package::Add, Command},
    context::{create_context, Context},
};

use leo_package::root::{
    lock_file::{LockFile, Package},
    Dependency,
};

use anyhow::{anyhow, Result};
use indexmap::set::IndexSet;
use std::collections::HashMap;
use structopt::StructOpt;
use tracing::span::Span;

/// Install dependencies Leo code command
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Fetch {}

impl Command for Fetch {
    /// Names of dependencies in the current branch of a dependency tree.
    type Input = IndexSet<String>;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Fetching")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        let package_name = context.manifest()?.get_package_name();

        let mut set = IndexSet::new();
        set.insert(package_name);

        Ok(set)
    }

    fn apply(self, context: Context, tree: Self::Input) -> Result<Self::Output> {
        let dependencies = context
            .manifest()
            .map_err(|_| anyhow!("Package Manifest not found"))?
            .get_package_dependencies();

        // If program has no dependencies in the Leo.toml, exit with success.
        let dependencies = match dependencies {
            Some(dependencies) => dependencies,
            None => return Ok(()),
        };

        if dependencies.is_empty() {
            return Ok(());
        }

        let mut lock_file = LockFile::new();
        self.add_dependencies(context.clone(), tree, &mut lock_file, dependencies)?;
        lock_file.write_to(&context.dir()?)?;

        Ok(())
    }
}

impl Fetch {
    /// Pulls dependencies and fills in the lock file. Also checks for
    /// recursive dependencies with dependency tree.
    fn add_dependencies(
        &self,
        context: Context,
        mut tree: IndexSet<String>,
        lock_file: &mut LockFile,
        dependencies: HashMap<String, Dependency>,
    ) -> Result<()> {
        // Go through each dependency in Leo.toml and add it to the imports.
        // While adding, pull dependencies of this package as well and check for recursion.
        for (import_name, dependency) in dependencies.into_iter() {
            let mut package = Package::from(&dependency);
            package.import_name = Some(import_name);

            // Pull the dependency first.
            let path = Add::new(
                None,
                Some(package.author.clone()),
                Some(package.name.clone()),
                Some(package.version.clone()),
            )
            .apply(context.clone(), ())?;

            // Try inserting a new dependency to the branch. If not inserted,
            // then fail because this dependency was added on a higher level.
            if !tree.insert(package.name.clone()) {
                // Pretty format for the message - show dependency structure.
                let mut message: Vec<String> = tree
                    .into_iter()
                    .enumerate()
                    .map(|(i, val)| format!("{}└─{}", " ".repeat(i * 2), val))
                    .collect();

                message.push(format!("{}└─{} (FAILURE)", " ".repeat(message.len() * 2), package.name));

                return Err(anyhow!("recursive dependency found \n{}", message.join("\n")));
            }

            // Check imported dependency's dependencies.
            let imported_dependencies = create_context(path, None)?
                .manifest()
                .map_err(|_| anyhow!("Unable to parse imported dependency's manifest"))?
                .get_package_dependencies();

            if let Some(dependencies) = imported_dependencies {
                if !dependencies.is_empty() {
                    // Fill in the lock file with imported dependency and information about its dependencies.
                    package.add_dependencies(&dependencies);
                    lock_file.add_package(package);

                    // Recursively call this method for imported program.
                    self.add_dependencies(context.clone(), tree.clone(), lock_file, dependencies)?;

                    continue;
                }
            }

            // If there are no dependencies for the new import, add a single record.
            lock_file.add_package(package);
        }

        Ok(())
    }
}
