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

use crate::{command::Command, context::Context};

use leo_package::LeoPackage;

use anyhow::Result;
use structopt::StructOpt;
use tracing::span::Span;

/// Remove imported package
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Remove {
    #[structopt(name = "PACKAGE")]
    name: String,
}

impl Remove {
    pub fn new(name: String) -> Remove {
        Remove { name }
    }
}

impl Command for Remove {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Removing")
    }

    fn prelude(&self) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, ctx: Context, _: Self::Input) -> Result<Self::Output> {
        let path = ctx.dir()?;
        let package_name = self.name;

        LeoPackage::remove_imported_package(&package_name, &path)?;
        tracing::info!("Successfully removed package \"{}\"\n", package_name);

        Ok(())
    }
}
