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

use crate::{api::Fetch, commands::Command, context::Context};
use leo_package::imports::{ImportsDirectory, IMPORTS_DIRECTORY_NAME};

use anyhow::{anyhow, Result};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
};
use structopt::StructOpt;
use tracing::Span;

/// Add package from Aleo Package Manager
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Add {
    #[structopt(name = "REMOTE")]
    remote: Option<String>,

    #[structopt(name = "author", help = "Specify a package author", long = "author", short = "a")]
    author: Option<String>,

    #[structopt(name = "package", help = "Specify a package name", long = "package", short = "p")]
    package: Option<String>,

    #[structopt(name = "version", help = "Specify a package version", long = "version", short = "v")]
    version: Option<String>,
}

impl Add {
    pub fn new(
        remote: Option<String>,
        author: Option<String>,
        package: Option<String>,
        version: Option<String>,
    ) -> Add {
        Add {
            remote,
            author,
            package,
            version,
        }
    }

    /// Try to parse author/package string from self.remote
    pub fn try_read_arguments(&self) -> Result<(String, String)> {
        if let Some(val) = &self.remote {
            let v: Vec<&str> = val.split('/').collect();
            if v.len() == 2 {
                Ok((v[0].to_string(), v[1].to_string()))
            } else {
                Err(anyhow!(
                    "Incorrect argument, please use --help for information on command use"
                ))
            }
        } else if let (Some(author), Some(package)) = (&self.author, &self.package) {
            Ok((author.clone(), package.clone()))
        } else {
            Err(anyhow!(
                "Incorrect argument, please use --help for information on command use"
            ))
        }
    }
}

impl Command for Add {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Adding")
    }

    fn prelude(&self) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, ctx: Context, _: Self::Input) -> Result<Self::Output> {
        // checking that manifest exists...
        if ctx.manifest().is_err() {
            return Err(anyhow!("Package Manifest not found, try running leo init or leo new"));
        };

        let (author, package_name) = match self.try_read_arguments() {
            Ok((author, package)) => (author, package),
            Err(err) => return Err(err),
        };
        let version = self.version;

        // build request body (Options are skipped when sealizing)
        let fetch = Fetch {
            author,
            package_name: package_name.clone(),
            version,
        };

        let bytes = ctx.api.run_route(fetch)?.bytes()?;
        let mut path = ctx.dir()?;

        {
            // setup directory structure since request was success
            ImportsDirectory::create(&path)?;
            path.push(IMPORTS_DIRECTORY_NAME);
            path.push(package_name);
            create_dir_all(&path)?;
        };

        let reader = std::io::Cursor::new(bytes);

        let mut zip_archive = match zip::ZipArchive::new(reader) {
            Ok(zip) => zip,
            Err(error) => return Err(anyhow!(error)),
        };

        for i in 0..zip_archive.len() {
            let file = match zip_archive.by_index(i) {
                Ok(file) => file,
                Err(error) => return Err(anyhow!(error)),
            };

            let file_name = file.name();

            let mut file_path = path.clone();
            file_path.push(file_name);

            if file_name.ends_with('/') {
                create_dir_all(file_path)?;
            } else {
                if let Some(parent_directory) = path.parent() {
                    create_dir_all(parent_directory)?;
                }

                File::create(file_path)?.write_all(&file.bytes().map(|e| e.unwrap()).collect::<Vec<u8>>())?;
            }
        }

        tracing::info!("Successfully added a package");

        Ok(())
    }
}
