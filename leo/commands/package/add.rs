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

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!
// COMMAND TEMPORARILY DISABLED
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::{api::Fetch, commands::Command, context::Context};
use leo_errors::{CliError, Result};
use leo_package::{imports::ImportsDirectory, PackageDirectory};

use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};
use structopt::StructOpt;
use tracing::Span;

/// Add a package from Aleo Package Manager
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
    fn try_read_arguments(&self) -> Result<(String, String)> {
        if let Some(val) = &self.remote {
            let v: Vec<&str> = val.split('/').collect();
            if v.len() == 2 {
                Ok((v[0].to_string(), v[1].to_string()))
            } else {
                Err(CliError::incorrect_command_argument().into())
            }
        } else if let (Some(author), Some(package)) = (&self.author, &self.package) {
            Ok((author.clone(), package.clone()))
        } else {
            Err(CliError::incorrect_command_argument().into())
        }
    }
}

impl<'a> Command<'a> for Add {
    type Input = ();
    type Output = PathBuf;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Adding")
    }

    fn prelude(&self, _: Context<'a>) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context<'a>, _: Self::Input) -> Result<Self::Output> {
        // Check that a manifest exists for the current package.
        context.manifest().map_err(|_| CliError::manifest_file_not_found())?;

        let (author, package_name) = self
            .try_read_arguments()
            .map_err(CliError::cli_bytes_conversion_error)?;

        tracing::info!("Package: {}/{}", &author, &package_name);

        // Attempt to fetch the package.
        let reader = {
            let fetch = Fetch {
                author: author.clone(),
                package_name: package_name.clone(),
                version: self.version.clone(),
            };
            let bytes = context
                .api
                .run_route(fetch)?
                .bytes()
                .map_err(CliError::cli_bytes_conversion_error)?;
            std::io::Cursor::new(bytes)
        };

        // Construct the directory structure.
        let mut path = context.dir()?;
        {
            ImportsDirectory::create(&path)?;
            path.push(ImportsDirectory::NAME);

            // Dumb compatibility hack.
            // TODO: Remove once `leo add` functionality is discussed.
            if self.version.is_some() {
                path.push(format!("{}-{}@{}", author, package_name, self.version.unwrap()));
            } else {
                path.push(package_name.clone());
            }
            create_dir_all(&path).map_err(CliError::cli_io_error)?;
        };

        // Proceed to unzip and parse the fetched bytes.
        let mut zip_archive = zip::ZipArchive::new(reader).map_err(CliError::cli_zip_error)?;
        for i in 0..zip_archive.len() {
            let file = zip_archive.by_index(i).map_err(CliError::cli_zip_error)?;

            let file_name = file.name();

            let mut file_path = path.clone();
            file_path.push(file_name);

            if file_name.ends_with('/') {
                create_dir_all(file_path).map_err(CliError::cli_io_error)?;
            } else {
                if let Some(parent_directory) = path.parent() {
                    create_dir_all(parent_directory).map_err(CliError::cli_io_error)?;
                }

                let mut created = File::create(file_path).map_err(CliError::cli_io_error)?;
                created
                    .write_all(&file.bytes().map(|e| e.unwrap()).collect::<Vec<u8>>())
                    .map_err(CliError::cli_bytes_conversion_error)?;
            }
        }

        tracing::info!("Successfully added package {}/{}", author, package_name);

        Ok(path)
    }
}
