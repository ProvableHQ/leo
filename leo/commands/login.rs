//
// Usage:
//
//    leo login <token>
//    leo login -u username -p password
//

use crate::{
    cli::CLI,
    cli_types::*,
    credentials::*,
    errors::{
        CLIError::LoginError,
        LoginError::{CannotGetToken, ConnectionUnavailable, WrongLoginOrPassword},
    },
};

use std::collections::HashMap;

pub const LOGIN_URL: &str = "api/account/authenticate";

#[derive(Debug)]
pub struct LoginCommand;

impl CLI for LoginCommand {
    // Format: token, username, password
    type Options = (Option<String>, Option<String>, Option<String>);
    type Output = ();

    const ABOUT: AboutType = "Login to the package manager (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        ("NAME", "Sets token for login to the package manager", false, 1u64),
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
            None => {
                Ok((None, None, None))
            }
        }
    }

    fn output(options: Self::Options) -> Result<Self::Output, crate::errors::CLIError> {
        let token = match options {
            // Login using existing token
            (Some(token), _, _) => token,

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
                            log::error!("Wrong login or password");
                            return Err(LoginError(WrongLoginOrPassword("Wrong login or password".into())));
                        }
                    },
                    //Cannot connect to the server
                    Err(_error) => {
                        return Err(LoginError(ConnectionUnavailable(
                            "Could not connect to the package manager".into(),
                        )));
                    }
                };

                match response.get("token") {
                    Some(token) => token.clone(),
                    None => return Err(LoginError(CannotGetToken("There is no token".into()))),
                }
            }

            // Login using stored JWT credentials.
            // TODO (raychu86) Package manager re-authentication from token
            (_, _, _) => {
                read_token()?
            }
        };

        write_token(token.as_str())?;
        log::info!("Successfully logged in");
        Ok(())
    }
}
