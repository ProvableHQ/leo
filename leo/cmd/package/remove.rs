// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{cmd::Cmd, context::Context};

use leo_package::LeoPackage;

use anyhow::Error;
use structopt::StructOpt;

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

impl Cmd for Remove {
    type Output = ();

    fn apply(self, ctx: Context) -> Result<Self::Output, Error> {
        // Begin "Removing" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Removing");
        let _enter = span.enter();

        let path = ctx.dir()?;
        let package_name = self.name;

        LeoPackage::remove_imported_package(&package_name, &path)?;
        tracing::info!("Successfully removed package \"{}\"\n", package_name);

        Ok(())
    }
}
