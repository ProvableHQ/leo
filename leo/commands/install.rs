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

use crate::{commands::Command, context::Context};

use anyhow::{anyhow, Result};
use structopt::StructOpt;
use tracing::span::Span;

/// Install dependencies Leo code command
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Install {}

impl Command for Install {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Installing")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let deps = context
            .manifest()?
            .get_package_dependencies()
            .ok_or_else(|| anyhow!("Package has no dependencies"))?;

        use crate::commands::package::Add;

        for (_name, dep) in deps.iter() {
            Add::new(
                None,
                Some(dep.author.clone()),
                Some(dep.name.clone()),
                Some(dep.version.clone()),
            )
            .execute(context.clone())?;
        }

        dbg!(deps);

        Ok(())
    }
}
