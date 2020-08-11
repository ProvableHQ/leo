//
// Usege:
//
//    leo login <token>
//    leo login -u username -p password
//    leo login // not yet implemented
//

use crate::{
    cli::CLI,
    cli_types::*,
    errors::{
        CLIError::LoginError,
        LoginError::{CannotGetToken, ConnectionUnavalaible, WrongLoginOrPassword},
    },
};
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
    fn write_token(token: &str) -> Result<(), io::Error> {
        let mut credentials = File::create(LEO_CREDENTIALS_PATH.as_str())?;
        credentials.write_all(&token.as_bytes())?;
        Ok(())
    }

    pub fn read_token() -> Result<String, io::Error> {
        let mut credentials = File::open(LEO_CREDENTIALS_PATH.as_str())?;
        let mut buf = String::new();
        credentials.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

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
                // TODO implement JWT
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
                        return Err(LoginError(ConnectionUnavalaible(
                            "Could not connect to the package manager".into(),
                        )));
                    }
                };

                match response.get("token") {
                    Some(token) => token.clone(),
                    None => return Err(LoginError(CannotGetToken("There is no token".into()))),
                }
            }

            // Login using JWT
            (_, _, _) => {
                // TODO JWT
                unimplemented!()
            }
        };

        // Create Leo credentials directory if it not exists
        if !Path::new(LEO_CREDENTIALS_DIR).exists() {
            create_dir(LEO_CREDENTIALS_DIR)?;
        }

        LoginCommand::write_token(token.as_str())?;
        log::info!("Successfully logged in");
        Ok(())
    }
}
