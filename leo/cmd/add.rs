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

use leo_package::imports::{ImportsDirectory, IMPORTS_DIRECTORY_NAME};

use std::{
    collections::HashMap,
    env::current_dir,
    fs::{create_dir_all, File},
    io::{Read, Write},
};

use crate::{
    cmd::Cmd,
    context::{Context, PACKAGE_MANAGER_URL},
};

use anyhow::{anyhow, Error};
use structopt::StructOpt;

pub const ADD_URL: &str = "v1/package/fetch";

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
    pub fn try_read_remote(self) -> Result<(String, String), Error> {
        if let Some(val) = self.remote {
            let v: Vec<&str> = val.split('/').collect();
            if v.len() == 2 {
                return Ok((v[0].to_string(), v[1].to_string()));
            }
        }

        Err(anyhow!("Unable to parse author and/or package from remote"))
    }
}

impl Cmd for Add {
    type Output = ();

    fn apply(self, ctx: Context) -> Result<Self::Output, Error> {
        // checking that manifest exists...
        let _ = ctx.manifest()?;

        // TODO: Add remote parsing feature in the future
        // let _ = self.try_read_remote();

        let (response, package_name) = match (self.author, self.package, self.version) {
            (Some(author), Some(package_name), version) => {
                let client = reqwest::blocking::Client::new();
                let url = format!("{}{}", PACKAGE_MANAGER_URL, ADD_URL);

                let mut json = HashMap::new();
                json.insert("author", author);
                json.insert("package_name", package_name.clone());

                if let Some(version) = version {
                    json.insert("version", version);
                }

                match client.post(&url).json(&json).send() {
                    Ok(response) => (response, package_name),
                    // Cannot connect to the server
                    Err(_error) => return Err(anyhow!("Could not connect to the Aleo Package Manager")),
                }
            }
            _ => return Err(anyhow!("Could not define package name and/or author")),
        };

        let mut path = current_dir()?;
        ImportsDirectory::create(&path)?;
        path.push(IMPORTS_DIRECTORY_NAME);
        path.push(package_name);
        create_dir_all(&path)?;

        let bytes = response.bytes()?;
        let reader = std::io::Cursor::new(bytes);

        let mut zip_arhive = match zip::ZipArchive::new(reader) {
            Ok(zip) => zip,
            Err(error) => return Err(anyhow!(error)),
        };

        for i in 0..zip_arhive.len() {
            let file = match zip_arhive.by_index(i) {
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

        tracing::info!("Successfully added a package\n");

        Ok(())
    }
}
