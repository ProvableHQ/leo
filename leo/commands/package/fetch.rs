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
    context::Context,
};

use anyhow::{anyhow, Result};
use indexmap::set::IndexSet;
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

    fn apply(self, context: Context, mut tree: Self::Input) -> Result<Self::Output> {
        let dependencies = context
            .manifest()
            .map_err(|_| anyhow!("Package Manifest not found"))?
            .get_package_dependencies();

        let dependencies = match dependencies {
            Some(value) => value,
            None => return Ok(()),
        };

        // Go through each dependency in Leo.toml and add it to the imports.
        // While adding, pull dependencies of this package as well and check for recursion.
        for (_name, dependency) in dependencies.iter() {
            let package_name = dependency.package.clone();
            let path = Add::new(
                None,
                Some(dependency.author.clone()),
                Some(package_name.clone()),
                Some(dependency.version.clone()),
            )
            .apply(context.clone(), ())?;

            // Try inserting a new dependency to the branch. If not inserted,
            // then fail because this dependency was added on a higher level.
            if !tree.insert(package_name.clone()) {
                // Pretty format for the message - show dependency structure.
                let mut message: Vec<String> = tree
                    .into_iter()
                    .enumerate()
                    .map(|(i, val)| format!("{}└─{}", " ".repeat(i * 2), val))
                    .collect();

                message.push(format!("{}└─{} (FAILURE)", " ".repeat(message.len() * 2), package_name));

                return Err(anyhow!("recursive dependency found \n{}", message.join("\n")));
            }

            // Run the same command for installed dependency.
            let mut new_context = context.clone();
            new_context.path = Some(path);
            (Fetch {}).apply(new_context, tree.clone())?;
        }

        Ok(())
    }
}
