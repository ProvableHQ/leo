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

use crate::cmd::Cmd;
use crate::context::{create_context, Context};
use anyhow::{anyhow, Error};
use leo_package::LeoPackage;
use std::env::current_dir;
use std::fs;
use structopt::StructOpt;

/// Create new Leo project
#[derive(StructOpt, Debug)]
pub struct New {
    #[structopt(name = "NAME", help = "Init as a library (containing lib.leo)")]
    name: String,

    #[structopt(help = "Init as a library (containing lib.leo)", long = "lib", short = "l")]
    is_lib: Option<bool>,
}

impl New {
    pub fn new(name: String, is_lib: Option<bool>) -> New {
        New { name, is_lib }
    }
}

impl Cmd for New {
    fn context(&self) -> Result<Context, Error> {
        create_context()
    }

    fn apply(self, _: Context) -> Result<(), Error> {
        let mut path = current_dir()?;
        let package_name = self.name;

        // Derive the package directory path
        path.push(&package_name);

        // Verify the package directory path does not exist yet
        if path.exists() {
            return Err(anyhow!("Directory already exists {:?}", path));
        }

        // Create the package directory
        fs::create_dir_all(&path).map_err(|err| anyhow!("Could not create directory {}", err))?;

        LeoPackage::initialize(&package_name, self.is_lib.unwrap_or(false), &path)?;

        Ok(())
    }
}
