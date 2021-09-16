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
use leo_errors::{CliError, Result};

use std::{
    borrow::Cow,
    fs::{
        File, {self},
    },
    io::{Read, Write},
    path::Path,
};
use structopt::StructOpt;
use tracing::Span;

/// Clone a package from Aleo Package Manager
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Clone {
    #[structopt(name = "REMOTE")]
    remote: Option<String>,

    #[structopt(name = "author", help = "Specify a package author", long = "author", short = "a")]
    author: Option<String>,

    #[structopt(name = "package", help = "Specify a package name", long = "package", short = "p")]
    package: Option<String>,

    #[structopt(name = "version", help = "Specify a package version", long = "version", short = "v")]
    version: Option<String>,
}

impl Clone {
    pub fn new(
        remote: Option<String>,
        author: Option<String>,
        package: Option<String>,
        version: Option<String>,
    ) -> Self {
        Self {
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

    /// Creates a directory at the provided path with the given directory name.
    fn create_directory(path: &Path, directory_name: &str) -> Result<()> {
        let mut path = Cow::from(path);

        // Check that the path ends in the directory name.
        // If it does not, proceed to append the directory name to the path.
        if path.is_dir() && !path.ends_with(directory_name) {
            path.to_mut().push(directory_name);
        }

        Ok(fs::create_dir_all(&path).map_err(CliError::cli_io_error)?)
    }
}

impl Command for Clone {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Cloning")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let (author, package_name) = self.try_read_arguments()?;

        // Attempt to fetch the package.
        let reader = {
            let fetch = Fetch {
                author,
                package_name: package_name.clone(),
                version: self.version,
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
        path.push(package_name.clone());
        Self::create_directory(&path, &package_name)?;

        // Proceed to unzip and parse the fetched bytes.
        let mut zip_archive = zip::ZipArchive::new(reader).map_err(CliError::cli_io_error)?;

        for i in 0..zip_archive.len() {
            let file = zip_archive.by_index(i).map_err(CliError::cli_zip_error)?;

            let file_name = file.name();

            let mut file_path = path.clone();
            file_path.push(file_name);

            if file_name.ends_with('/') {
                fs::create_dir_all(file_path).map_err(CliError::cli_io_error)?;
            } else {
                if let Some(parent_directory) = path.parent() {
                    fs::create_dir_all(parent_directory).map_err(CliError::cli_io_error)?;
                }

                let mut created = File::create(file_path).map_err(CliError::cli_io_error)?;
                created
                    .write_all(&file.bytes().map(|e| e.unwrap()).collect::<Vec<u8>>())
                    .map_err(CliError::cli_bytes_conversion_error)?;
            }
        }

        tracing::info!("Successfully cloned {}", package_name);

        Ok(())
    }
}
