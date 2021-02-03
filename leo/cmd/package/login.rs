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

use crate::{
    cmd::Cmd,
    context::{Context, PACKAGE_MANAGER_URL},
};

use crate::config::*;

use anyhow::{anyhow, Error};
use structopt::StructOpt;

use std::collections::HashMap;

pub const LOGIN_URL: &str = "v1/account/authenticate";
pub const PROFILE_URL: &str = "v1/account/my_profile";

/// Login to Aleo PM and store credentials locally
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Login {
    #[structopt(name = "TOKEN")]
    token: Option<String>,

    #[structopt(short = "u", long = "user", about = "Username for Aleo PM")]
    user: Option<String>,

    #[structopt(short = "p", long = "password", about = "Password for Aleo PM")]
    pass: Option<String>,
}

impl Login {
    pub fn new(token: Option<String>, user: Option<String>, pass: Option<String>) -> Login {
        Login { token, user, pass }
    }
}

impl Cmd for Login {
    type Output = String;

    fn apply(self, _ctx: Context) -> Result<Self::Output, Error> {
        // Begin "Login" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Login");
        let _enter = span.enter();

        let token = match (self.token, self.user, self.pass) {
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
                            return Err(anyhow!("Wrong login or password"));
                        }
                    },
                    //Cannot connect to the server
                    Err(_error) => {
                        return Err(anyhow!("No connection found"));
                    }
                };

                match response.get("token") {
                    Some(token) => Some(token.clone()),
                    None => {
                        return Err(anyhow!("Unable to get token"));
                    }
                }
            }

            // Login using stored JWT credentials.
            // TODO (raychu86) Package manager re-authentication from token
            (_, _, _) => {
                let token = read_token().map_err(|_| -> Error { anyhow!("No credentials provided") })?;

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

                Err(anyhow!("No credentials provided. Please read --help"))
            }
        }
    }
}
