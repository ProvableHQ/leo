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

use std::{collections::HashMap, env::current_dir, fs::File, io::Write};

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
        if arguments.is_present("author") && arguments.is_present("package_name") {
            return Ok((
                arguments.value_of("author").map(|s| s.to_string()),
                arguments.value_of("package_name").map(|s| s.to_string()),
                arguments.value_of("version").map(|s| s.to_string()),
            ));
        } else {
            return Err(AddError(MissingAuthorOrPackageName));
        }
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        let token = read_token()?;

        let (mut result, package_name) = match options {
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
                    Ok(result) => (result, package_name),
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
        path.push(format!("{}.zip", package_name));

        // TODO (raychu86) Display package download progress
        let mut buffer: Vec<u8> = vec![];
        result.copy_to(&mut buffer).unwrap();

        let mut file = File::create(path)?;
        file.write_all(&buffer)?;

        log::info!("Successfully added package");
        Ok(())
    }
}
