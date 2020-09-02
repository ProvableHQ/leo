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

use crate::{
    cli::*,
    cli_types::*,
    commands::{BuildCommand, LoginCommand},
    config::{read_token, PACKAGE_MANAGER_URL},
    errors::{
        commands::PublishError::{ConnectionUnavalaible, PackageNotPublished},
        CLIError,
        PublishError::{MissingPackageDescription, MissingPackageLicense, MissingPackageRemote},
    },
};
use leo_package::{
    outputs::OutputsDirectory,
    root::{Manifest, ZipFile},
};

use clap::ArgMatches;
use reqwest::{
    blocking::{multipart::Form, Client},
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;
use std::{convert::TryFrom, env::current_dir};

const PUBLISH_URL: &str = "api/package/publish";

#[derive(Deserialize)]
struct ResponseJson {
    package_id: String,
}

#[derive(Debug)]
pub struct PublishCommand;

impl CLI for PublishCommand {
    type Options = ();
    type Output = Option<String>;

    const ABOUT: AboutType = "Publish the current package to the Aleo Package Manager";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "publish";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(_options: Self::Options) -> Result<Self::Output, CLIError> {
        // Build all program files.
        let _output = BuildCommand::output(())?;

        // Begin "Publishing" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Publishing");
        let _enter = span.enter();

        // Get the package manifest
        let path = current_dir()?;
        let package_manifest = Manifest::try_from(&path)?;

        let package_name = package_manifest.get_package_name();
        let package_version = package_manifest.get_package_version();

        if package_manifest.get_package_description().is_none() {
            return Err(MissingPackageDescription.into());
        }

        if package_manifest.get_package_license().is_none() {
            return Err(MissingPackageLicense.into());
        }

        let package_remote = match package_manifest.get_package_remote() {
            Some(remote) => remote,
            None => return Err(MissingPackageRemote.into()),
        };

        // Create the output directory
        OutputsDirectory::create(&path)?;

        // Create zip file
        let zip_file = ZipFile::new(&package_name);
        if zip_file.exists_at(&path) {
            tracing::debug!("Existing package zip file found. Clearing it to regenerate.");
            // Remove the existing package zip file
            ZipFile::new(&package_name).remove(&path)?;
        }

        zip_file.write(&path)?;

        let form_data = Form::new()
            .text("name", package_name.clone())
            .text("remote", format!("{}/{}", package_remote.author, package_name))
            .text("version", package_version)
            .file("file", zip_file.get_file_path(&path))?;

        // Client for make POST request
        let client = Client::new();

        // Get token to make an authorized request
        let token = match read_token() {
            Ok(token) => token,

            // If not logged in, then try logging in using JWT.
            Err(_error) => {
                tracing::warn!("You should be logged in before attempting to publish a package");
                tracing::info!("Trying to log in using JWT...");
                let options = (None, None, None);

                LoginCommand::output(options)?
            }
        };

        // Headers for request to publish package
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("{} {}", "Bearer", token)).unwrap(),
        );

        // Make a request to publish a package
        let response = client
            .post(format!("{}{}", PACKAGE_MANAGER_URL, PUBLISH_URL).as_str())
            .headers(headers)
            .multipart(form_data)
            .send();

        // Get a response result
        let result = match response {
            Ok(json_result) => match json_result.json::<ResponseJson>() {
                Ok(json) => json,
                Err(error) => {
                    tracing::warn!("{:?}", error);
                    return Err(PackageNotPublished("Package not published".into()).into());
                }
            },
            Err(error) => {
                tracing::warn!("{:?}", error);
                return Err(ConnectionUnavalaible("Connection error".into()).into());
            }
        };

        tracing::info!("Package published successfully with id: {}", result.package_id);
        Ok(Some(result.package_id))
    }
}
