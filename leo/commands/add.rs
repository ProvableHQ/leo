use crate::{cli::*, cli_types::*, commands::LoginCommand, errors::CLIError};
use clap::ArgMatches;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{create_dir, create_dir_all, File},
    io::{prelude::*, Error, ErrorKind},
    path::Path,
};

const PACKAGE_MANAGER_URL: &str = "https://apm-backend-dev.herokuapp.com/";
const FETCH_URL: &str = "api/package/fetch";

pub const LEO_PACKAGES_DIR: &str = "leo_packages";

#[derive(Debug)]
pub struct AddCommand;

impl CLI for AddCommand {
    type Options = (Option<String>, Option<String>);
    type Output = ();

    const ABOUT: AboutType = "Install a package from the package manager (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        ("NAME", "Installs the package to the current directory", true, 1u64),
    ];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "add";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        match arguments.clone().value_of("NAME") {
            Some(name) => {
                let name_version = name.split("@").collect::<Vec<&str>>();
                match name_version[..] {
                    [package_name, version] => Ok((Some(package_name.to_string()), Some(version.to_string()))),
                    [package_name] => Ok((Some(package_name.to_string()), Some("latest".to_string()))),
                    _ => unreachable!(),
                }
            }
            None => Ok((None, None)),
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let _path = current_dir()?;
        let _build_options = ();

        fn download_package(package_name: String, version: String) -> Result<(), std::io::Error> {
            let client = Client::new();
            let mut params = HashMap::new();

            // TODO waiting for the new API with package name request
            params.insert("package_id", &package_name);
            params.insert("version", &version);

            let token = LoginCommand::get_token();

            // Headers for request to fetch package
            let mut headers = HeaderMap::new();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(&format!("{} {}", "Bearer", token)).unwrap(),
            );

            let url = format!("{}{}", PACKAGE_MANAGER_URL, FETCH_URL);
            let response = client.post(url.as_str()).headers(headers).json(&params).send();

            let bytes = match response {
                Ok(result) => result.bytes().unwrap(),
                Err(_e) => {
                    return Err(Error::new(ErrorKind::NotConnected, "Connection unavailable"));
                }
            };

            // Create Leo packages directory if it does not exist
            if !Path::new(LEO_PACKAGES_DIR).exists() {
                create_dir(LEO_PACKAGES_DIR)?;
            }

            let reader = std::io::Cursor::new(bytes);
            let mut zip_arhive = zip::ZipArchive::new(reader)?;
            create_dir(format!("{}/{}", LEO_PACKAGES_DIR, package_name))?;

            for i in 0..zip_arhive.len() {
                let file = zip_arhive.by_index(i)?;
                let file_path = format!("{}/{}/{}", LEO_PACKAGES_DIR, package_name, file.name());
                let path = Path::new(&file_path);

                if file_path.ends_with("/") {
                    create_dir_all(path)?;
                } else {
                    File::create(path)?.write_all(&file.bytes().map(|e| e.unwrap()).collect::<Vec<u8>>())?;
                }
            }
            Ok(())
        }

        match options {
            // Download the package with specified name and version
            (Some(package_name), Some(version)) => {
                download_package(package_name, version)?;
            }

            // Download the latest version of the package with specified name
            (Some(package_name), None) => {
                download_package(package_name, "latest".to_string())?;
            }

            _ => unreachable!(),
        };

        Ok(())

        // match BuildCommand::output(build_options)? {
        //     Some((_program, _checksum_differs)) => {
        //         // Get the package name
        //         let _package_name = Manifest::try_from(&path)?.get_package_name();

        //         log::info!("Unimplemented - `leo load`");

        //         Ok(())
        //     }
        //     None => {
        //         let mut main_file_path = path.clone();
        //         main_file_path.push(SOURCE_DIRECTORY_NAME);
        //         main_file_path.push(MAIN_FILE_NAME);

        //         Err(CLIError::RunError(RunError::MainFileDoesNotExist(
        //             main_file_path.into_os_string(),
        //         )))
        //     }
        // }
    }
}
