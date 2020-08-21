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
    errors::{
        CLIError::LoginError,
        LoginError::{CannotGetToken, NoConnectionFound, NoCredentialsProvided, WrongLoginOrPassword},
    },
};

use std::collections::HashMap;

pub const LOGIN_URL: &str = "api/account/authenticate";

#[derive(Debug)]
pub struct LoginCommand;

impl CLI for LoginCommand {
    // Format: token, username, password
    type Options = (Option<String>, Option<String>, Option<String>);
    type Output = String;

    const ABOUT: AboutType = "Login to the Aleo Package Manager";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        (
            "NAME",
            "Sets the authentication token for login to the package manager",
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

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        let token = match options {
            // Login using existing token
            (Some(token), _, _) => Some(token),

            // Login using username and password
            (None, Some(username), Some(password)) => {
                let client = reqwest::blocking::Client::new();
                let url = format!("{}{}", PACKAGE_MANAGER_URL, LOGIN_URL);

                let mut json = HashMap::new();
                json.insert("email_username", username);
                json.insert("password", password);

                let response: HashMap<String, String> = match client.post(&url).json(&json).send() {
                    Ok(result) => match result.json() {
                        Ok(json) => json,
                        Err(_error) => {
                            tracing::error!("Wrong login or password");
                            return Err(WrongLoginOrPassword("Wrong login or password".into()).into());
                        }
                    },
                    //Cannot connect to the server
                    Err(_error) => {
                        return Err(LoginError(NoConnectionFound(
                            "Could not connect to the package manager".into(),
                        )));
                    }
                };

                match response.get("token") {
                    Some(token) => Some(token.clone()),
                    None => {
                        return Err(CannotGetToken("No token was provided in the response".into()).into());
                    }
                }
            }

            // Login using stored JWT credentials.
            // TODO (raychu86) Package manager re-authentication from token
            (_, _, _) => Some(read_token()?),
        };

        match token {
            Some(token) => {
                write_token(token.as_str())?;

                tracing::info!("login successful.");

                Ok(token)
            }
            _ => {
                tracing::error!("Failed to login. Please run `leo login -h` for help.");

                Err(NoCredentialsProvided.into())
            }
        }
    }
}
