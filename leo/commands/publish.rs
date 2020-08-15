use crate::{
    cli::*,
    cli_types::*,
    commands::{BuildCommand, LoginCommand},
    errors::{
        commands::PublishError::{ConnectionUnavalaible, PackageNotPublished},
        CLIError,
        CLIError::PublishError,
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

const PACKAGE_MANAGER_URL: &str = "https://apm-backend-dev.herokuapp.com/";
const PUBLISH_URL: &str = "api/package/publish";

#[derive(Deserialize)]
struct ResponseJson {
    package_id: String,
    _success: bool,
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
        // let _output = BuildCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();
        let package_version = Manifest::try_from(&path)?.get_package_version();

        // Create the output directory
        OutputsDirectory::create(&path)?;

        // Create zip file
        let zip_file = ZipFile::new(&package_name);
        if zip_file.exists_at(&path) {
            log::debug!("Existing package zip file found. Clearing it to regenerate.");
            // Remove the existing package zip file
            ZipFile::new(&package_name).remove(&path)?;
        } else {
            zip_file.write(&path)?;
        }

        let form_data = Form::new()
            .text("name", package_name)
            .text("version", package_version)
            .file("file", zip_file.get_file_path(&path))?;

        // Client for make POST request
        let client = Client::new();

        // Get token to make an authorized request
        let token = match LoginCommand::read_token() {
            Ok(token) => token,

            // If not logged in, then try logging in using JWT.
            Err(_error) => {
                log::warn!("You should be logged before publish the package");
                log::info!("Trying to log in using JWT...");
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
                    log::warn!("{:?}", error);
                    return Err(PublishError(PackageNotPublished("Package not published".into())));
                }
            },
            Err(error) => {
                log::warn!("{:?}", error);
                return Err(PublishError(ConnectionUnavalaible("Connection error".into())));
            }
        };

        log::info!("Package published successfully");
        Ok(Some(result.package_id))
    }
}
