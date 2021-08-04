// Copyright (C) 2019-2021 Aleo Systems Inc.
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
    api::{Login as LoginRoute, Profile as ProfileRoute},
    commands::Command,
    config::*,
    context::Context,
};
use leo_errors::{CliError, Result};

use std::collections::HashMap;
use structopt::StructOpt;
use tracing::Span;

/// Login to Aleo PM and store credentials locally
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Login {
    #[structopt(name = "AUTH_TOKEN", about = "Pass authorization token")]
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

impl Command for Login {
    type Input = ();
    type Output = String;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Login")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        let mut api = context.clone().api;

        // ...or trying to use arguments to either get token or user-pass
        let (token, username) = match (self.token, self.user, self.pass) {
            // Login using username and password if they were passed. Even if token already
            // exists login procedure will be done first (we need that for expired credentials).
            (None, Some(email_username), Some(password)) => {
                let login = LoginRoute {
                    email_username: email_username.clone(),
                    password,
                };

                let res = api.run_route(login)?;
                let mut res: HashMap<String, String> = res.json().map_err(|e| CliError::reqwest_json_error(e))?;

                let tok_opt = res.remove("token");
                if tok_opt.is_none() {
                    return Err(CliError::unable_to_get_token().into());
                };

                (tok_opt.unwrap(), email_username)
            }

            // Login with token, use get_profile route to verify that.
            (Some(token), _, _) => {
                tracing::info!("Token passed, checking...");

                api.set_auth_token(token.clone());

                match api.run_route(ProfileRoute {})? {
                    Some(username) => (token, username),
                    None => return Err(CliError::supplied_token_is_incorrect().into()),
                }
            }

            // In case token or login/pass were not passed as arguments
            (_, _, _) => {
                // Check locally stored token if there is.
                let token = context.api.auth_token();

                match token {
                    Some(token) => {
                        tracing::info!("Found locally stored credentials, verifying...");

                        if let Some(username) = api.run_route(ProfileRoute {})? {
                            (token, username)
                        } else {
                            remove_token_and_username()?;
                            return Err(CliError::stored_credentials_expired().into());
                        }
                    }
                    None => return Err(CliError::no_credentials_provided().into()),
                }
            }
        };

        // write token either after logging or if it was passed
        write_token_and_username(token.as_str(), username.as_str())?;

        tracing::info!("Success! You are logged in!");

        Ok(token)
    }
}
