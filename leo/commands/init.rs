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

use crate::{commands::Command, config::*, context::Context};
use leo_package::LeoPackage;

use anyhow::{anyhow, Result};
use std::env::current_dir;
use structopt::StructOpt;
use tracing::span::Span;

/// Init Leo project command within current directory
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Init {}

impl Command for Init {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Initializing")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _: Context, _: Self::Input) -> Result<Self::Output> {
        // Derive the package directory path.
        let path = current_dir()?;

        // Check that the current package directory path exists.
        if !path.exists() {
            return Err(anyhow!("Directory does not exist"));
        }

        // Check that the given package name is valid.
        let package_name = path
            .file_stem()
            .ok_or_else(|| anyhow!("Project name invalid"))?
            .to_string_lossy()
            .to_string();
        if !LeoPackage::is_package_name_valid(&package_name) {
            return Err(anyhow!("Invalid Leo project name: {}", package_name));
        }

        let username = read_username().ok();

        LeoPackage::initialize(&package_name, &path, username)?;

        Ok(())
    }
}
