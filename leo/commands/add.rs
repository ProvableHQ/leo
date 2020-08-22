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

//
// Usage:
//
//    leo add -a author -p package_name -v version
//    leo add -a author -p package_name
//

use crate::{cli::CLI, cli_types::*, config::*, errors::AddError::*};
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
        // Begin "Adding" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Adding");
        let _enter = span.enter();

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
                        return Err(ConnectionUnavailable("Could not connect to the package manager".into()).into());
                    }
                }
            }
            _ => return Err(MissingAuthorOrPackageName.into()),
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
            Err(error) => return Err(ZipError(error.to_string().into()).into()),
        };

        for i in 0..zip_arhive.len() {
            let file = match zip_arhive.by_index(i) {
                Ok(file) => file,
                Err(error) => return Err(ZipError(error.to_string().into()).into()),
            };

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

        tracing::info!("Successfully added a package\n");
        Ok(())
    }
}
