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

use super::build::Build;
use crate::{
    commands::Command,
    context::{Context, PACKAGE_MANAGER_URL},
};
use leo_package::{outputs::OutputsDirectory, root::ZipFile};

use anyhow::{anyhow, Result};
use reqwest::{
    blocking::{multipart::Form, Client},
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;
use structopt::StructOpt;

pub const PUBLISH_URL: &str = "v1/package/publish";

#[derive(Deserialize)]
struct ResponseJson {
    package_id: String,
}

/// Publish package to Aleo Package Manager
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Publish {}

impl Command for Publish {
    type Input = <Build as Command>::Output;
    type Output = Option<String>;

    /// Build program before publishing
    fn prelude(&self) -> Result<Self::Input> {
        (Build {}).execute()
    }

    fn apply(self, context: Context, _input: Self::Input) -> Result<Self::Output> {
        // Get the package manifest
        let path = context.dir()?;
        let manifest = context.manifest()?;

        let package_name = manifest.get_package_name();
        let package_version = manifest.get_package_version();

        match (
            manifest.get_package_description(),
            manifest.get_package_license(),
            manifest.get_package_remote(),
        ) {
            (None, _, _) => return Err(anyhow!("No package description")),
            (_, None, _) => return Err(anyhow!("Missing package license")),
            (_, _, None) => return Err(anyhow!("Missing package remote")),
            (_, _, _) => (),
        };

        let package_remote = manifest.get_package_remote().unwrap();

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

        let token = match context.api.auth_token() {
            Some(token) => token,
            None => return Err(anyhow!("Login before publishing package: try leo login --help")),
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
        let result: ResponseJson = match response {
            Ok(json_result) => {
                let text = json_result.text()?;

                match serde_json::from_str(&text) {
                    Ok(json) => json,
                    Err(_) => {
                        return Err(anyhow!("Package not published: {}", text));
                    }
                }
            }
            Err(error) => {
                tracing::warn!("{:?}", error);
                return Err(anyhow!("Connection unavailable"));
            }
        };

        tracing::info!("Package published successfully with id: {}", result.package_id);
        Ok(Some(result.package_id))
    }
}
