//
// Usage:
//
//    leo login <token>
//    leo login -u username -p password
//    leo login // not yet implemented
//

use crate::{cli::CLI, cli_types::*, errors::LoginError};
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fs::{create_dir, File},
    io,
    io::prelude::*,
    path::Path,
};

const PACKAGE_MANAGER_URL: &str = "https://apm-backend-dev.herokuapp.com/";
const LOGIN_URL: &str = "api/account/login";

const LEO_CREDENTIALS_DIR: &str = ".leo";
const LEO_CREDENTIALS_FILE: &str = "credentials";

lazy_static! {
    static ref LEO_CREDENTIALS_PATH: String = format!("{}/{}", LEO_CREDENTIALS_DIR, LEO_CREDENTIALS_FILE);
}

#[derive(Debug)]
pub struct LoginCommand;

impl LoginCommand {
    // Write token to the leo credentials file
    fn write_token(token: &str) -> Result<(), io::Error> {
        let mut credentials = File::create(LEO_CREDENTIALS_PATH.as_str())?;
        credentials.write_all(&token.as_bytes())?;
        Ok(())
    }

    // Read token from the leo credentials file
    pub fn read_token() -> Result<String, io::Error> {
        let mut credentials = File::open(LEO_CREDENTIALS_PATH.as_str())?;
        let mut buf = String::new();
        credentials.read_to_string(&mut buf)?;
        Ok(buf)
    }

    // Get token to make authorized requests
    pub fn get_token() -> String {
        // Get token to make an authorized request
        let token = match LoginCommand::read_token() {
            // Already logged in
            Ok(token) => token,

            // If not logged then try to login using JWT
            Err(_errorr) => {
                log::warn!("You should be logged before publish the package");
                log::info!("Trying to log in using JWT...");
                let options = (None, None, None);
                LoginCommand::output(options).unwrap()
            }
        };
        token
    }
}

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
            None => {
                // TODO implement JWT
                Ok((None, None, None))
            }
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
                            log::error!("Wrong login or password");
                            return Err(LoginError::WrongLoginOrPassword("Wrong login or password".into()).into());
                        }
                    },
                    //Cannot connect to the server
                    Err(_error) => {
                        return Err(
                            LoginError::NoConnectionFound("Could not connect to the package manager".into()).into(),
                        );
                    }
                };

                match response.get("token") {
                    Some(token) => Some(token.clone()),
                    None => {
                        return Err(LoginError::CannotGetToken("No token was provided in the response".into()).into());
                    }
                }
            }

            // Login using JWT
            (_, _, _) => {
                // TODO JWT
                None
            }
        };

        match token {
            Some(token) => {
                // Create Leo credentials directory if it not exists
                if !Path::new(LEO_CREDENTIALS_DIR).exists() {
                    create_dir(LEO_CREDENTIALS_DIR)?;
                }

                LoginCommand::write_token(token.as_str())?;

                log::info!("Login successful.");

                Ok(token)
            }
            _ => {
                log::error!("Failed to login. Please run `leo login -h` for help.");

                Err(LoginError::NoCredentialsProvided.into())
            }
        }
    }
}
