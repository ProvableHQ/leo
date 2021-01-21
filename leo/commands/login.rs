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
//    leo login <token>
//    leo login -u username -p password
//

use crate::{
    cli::CLI,
    cli_types::*,
    config::*,
    errors::{CLIError, LoginError::*},
};

use std::collections::HashMap;

pub const LOGIN_URL: &str = "v1/account/authenticate";
pub const PROFILE_URL: &str = "v1/account/my_profile";

#[derive(Debug)]
pub struct LoginCommand;

impl CLI for LoginCommand {
    // Format: token, username, password
    type Options = (Option<String>, Option<String>, Option<String>);
    type Output = String;

    const ABOUT: AboutType = "Login to the Aleo Package Manager";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, possible_values, required, index)
        (
            "NAME",
            "Sets the authentication token for login to the package manager",
            &[],
            false,
            1u64,
        ),
    ];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "login";
    const OPTIONS: &'static [OptionType] = &[
        // (argument, conflicts, possible_values, requires)
        ("[username] -u --user=[username] 'Sets a username'", &[], &[], &[]),
        ("[password] -p --password=[password] 'Sets a password'", &[], &[], &[]),
    ];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    fn parse(arguments: &clap::ArgMatches) -> Result<Self::Options, crate::errors::CLIError> {
        if arguments.is_present("username") && arguments.is_present("password") {
            return Ok((
                None,
                Some(arguments.value_of("username").unwrap().to_string()),
                Some(arguments.value_of("password").unwrap().to_string()),
            ));
        }

        match arguments.value_of("NAME") {
            Some(name) => Ok((Some(name.to_string()), None, None)),
            None => Ok((None, None, None)),
        }
    }

    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        // Begin "Login" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Login");
        let _enter = span.enter();

        let token = match options {
            // Login using existing token
            (Some(token), _, _) => Some(token),

            // Login using username and password
            (None, Some(username), Some(password)) => {
                // prepare JSON data to be sent
                let mut json = HashMap::new();
                json.insert("email_username", username);
                json.insert("password", password);

                let client = reqwest::blocking::Client::new();
                let url = format!("{}{}", PACKAGE_MANAGER_URL, LOGIN_URL);
                let response: HashMap<String, String> = match client.post(&url).json(&json).send() {
                    Ok(result) => match result.json() {
                        Ok(json) => json,
                        Err(_error) => {
                            return Err(WrongLoginOrPassword.into());
                        }
                    },
                    //Cannot connect to the server
                    Err(_error) => {
                        return Err(NoConnectionFound.into());
                    }
                };

                match response.get("token") {
                    Some(token) => Some(token.clone()),
                    None => {
                        return Err(CannotGetToken.into());
                    }
                }
            }

            // Login using stored JWT credentials.
            // TODO (raychu86) Package manager re-authentication from token
            (_, _, _) => {
                let token = read_token().map_err(|_| -> CLIError { NoCredentialsProvided.into() })?;

                Some(token)
            }
        };

        match token {
            Some(token) => {
                write_token(token.as_str())?;

                tracing::info!("success");

                Ok(token)
            }
            _ => {
                tracing::error!("Failed to login. Please run `leo login -h` for help.");

                Err(NoCredentialsProvided.into())
            }
        }
    }

    fn new<'a, 'b>() -> clap::App<'a, 'b> {
        let arguments = &Self::ARGUMENTS
            .iter()
            .map(|a| {
                let mut args = clap::Arg::with_name(a.0).help(a.1).required(a.3).index(a.4);
                if !a.2.is_empty() {
                    args = args.possible_values(a.2);
                }
                args
            })
            .collect::<Vec<clap::Arg<'static, 'static>>>();
        let flags = &Self::FLAGS
            .iter()
            .map(|a| clap::Arg::from_usage(a))
            .collect::<Vec<clap::Arg<'static, 'static>>>();
        let options = &Self::OPTIONS
            .iter()
            .map(|a| match !a.2.is_empty() {
                true => clap::Arg::from_usage(a.0)
                    .conflicts_with_all(a.1)
                    .possible_values(a.2)
                    .requires_all(a.3),
                false => clap::Arg::from_usage(a.0).conflicts_with_all(a.1).requires_all(a.3),
            })
            .collect::<Vec<clap::Arg<'static, 'static>>>();
        let subcommands = Self::SUBCOMMANDS.iter().map(|s| {
            clap::SubCommand::with_name(s.0)
                .about(s.1)
                .args(
                    &s.2.iter()
                        .map(|a| {
                            let mut args = clap::Arg::with_name(a.0).help(a.1).required(a.3).index(a.4);
                            if !a.2.is_empty() {
                                args = args.possible_values(a.2);
                            }
                            args
                        })
                        .collect::<Vec<clap::Arg<'static, 'static>>>(),
                )
                .args(
                    &s.3.iter()
                        .map(|a| clap::Arg::from_usage(a))
                        .collect::<Vec<clap::Arg<'static, 'static>>>(),
                )
                .args(
                    &s.4.iter()
                        .map(|a| match !a.2.is_empty() {
                            true => clap::Arg::from_usage(a.0)
                                .conflicts_with_all(a.1)
                                .possible_values(a.2)
                                .requires_all(a.3),
                            false => clap::Arg::from_usage(a.0).conflicts_with_all(a.1).requires_all(a.3),
                        })
                        .collect::<Vec<clap::Arg<'static, 'static>>>(),
                )
                .settings(s.5)
        });

        clap::SubCommand::with_name(Self::NAME)
            .about(Self::ABOUT)
            .settings(&[
                clap::AppSettings::ColoredHelp,
                clap::AppSettings::DisableHelpSubcommand,
                clap::AppSettings::DisableVersion,
            ])
            .args(arguments)
            .args(flags)
            .args(options)
            .subcommands(subcommands)
    }

    fn process(arguments: &clap::ArgMatches) -> Result<(), CLIError> {
        // Set logging environment
        match arguments.is_present("debug") {
            true => crate::logger::init_logger("leo", 2),
            false => crate::logger::init_logger("leo", 1),
        }

        if arguments.subcommand().0 != "update" {
            crate::updater::Updater::print_cli();
        }

        let options = Self::parse(arguments)?;
        let _output = Self::output(options)?;
        Ok(())
    }
}
