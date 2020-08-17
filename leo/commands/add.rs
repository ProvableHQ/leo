//
// Usage:
//
//    leo add -a author -p package_name -v version
//    leo add -a author -p package_name
//

use crate::{
    cli::CLI,
    cli_types::*,
    credentials::*,
    errors::{AddError::*, CLIError::AddError},
};
use leo_package::{
    imports::{ImportsDirectory, IMPORTS_DIRECTORY_NAME},
    root::Manifest,
};

use std::{
    collections::HashMap,
    convert::TryFrom,
    env::current_dir,
    fs::{create_dir_all, File},
    io::{Read, Write},
};

pub const ADD_URL: &str = "api/package/fetch";

#[derive(Debug)]
pub struct AddCommand;

impl CLI for AddCommand {
    // Format: author, package_name, version
    type Options = (Option<String>, Option<String>, Option<String>);
    type Output = ();

    const ABOUT: AboutType = "Install a package from the package manager";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "add";
    const OPTIONS: &'static [OptionType] = &[
        // (argument, conflicts, possible_values, requires)
        ("[author] -a --author=<author> 'Specify a package author'", &[], &[], &[
            "package_name",
        ]),
        (
            "[package_name] -p --package_name=<package_name> 'Specify a package name'",
            &[],
            &[],
            &["author"],
        ),
        (
            "[version] -v --version=[version] 'Specify a package version'",
            &[],
            &[],
            &["author", "package_name"],
        ),
    ];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        // TODO update to new package manager API without an author field
        if arguments.is_present("author") && arguments.is_present("package_name") {
            return Ok((
                arguments.value_of("author").map(|s| s.to_string()),
                arguments.value_of("package_name").map(|s| s.to_string()),
                arguments.value_of("version").map(|s| s.to_string()),
            ));
        } else {
            return Ok((None, None, None));
        }
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        let token = read_token()?;

        let path = current_dir()?;
        // Enforce that the current directory is a leo package
        Manifest::try_from(&path)?;

        let (response, package_name) = match options {
            (Some(author), Some(package_name), version) => {
                let client = reqwest::blocking::Client::new();
                let url = format!("{}{}", PACKAGE_MANAGER_URL, ADD_URL);

                let mut json = HashMap::new();
                json.insert("author", author);
                json.insert("package_name", package_name.clone());

                if let Some(version) = version {
                    json.insert("version", version);
                }

                match client.post(&url).bearer_auth(token).json(&json).send() {
                    Ok(response) => (response, package_name),
                    //Cannot connect to the server
                    Err(_error) => {
                        return Err(AddError(ConnectionUnavailable(
                            "Could not connect to the package manager".into(),
                        )));
                    }
                }
            }
            _ => return Err(AddError(MissingAuthorOrPackageName)),
        };

        let mut path = current_dir()?;
        ImportsDirectory::create(&path)?;
        path.push(IMPORTS_DIRECTORY_NAME);
        path.push(package_name);
        create_dir_all(&path)?;

        let bytes = response.bytes()?;
        let reader = std::io::Cursor::new(bytes);

        let mut zip_arhive = zip::ZipArchive::new(reader).unwrap();

        for i in 0..zip_arhive.len() {
            let file = zip_arhive.by_index(i).unwrap();
            let file_name = file.name();

            let mut file_path = path.clone();
            file_path.push(file_name);

            if file_name.ends_with("/") {
                create_dir_all(file_path)?;
            } else {
                if let Some(parent_directory) = path.parent() {
                    create_dir_all(parent_directory)?;
                }

                File::create(file_path)?.write_all(&file.bytes().map(|e| e.unwrap()).collect::<Vec<u8>>())?;
            }
        }

        log::info!("Successfully added a package");
        Ok(())
    }
}
