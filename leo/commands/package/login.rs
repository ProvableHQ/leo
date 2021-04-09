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

use anyhow::{anyhow, Result};
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
        // quick hack to check if user is already logged in. ;)
        if context.api.auth_token().is_some() {
            tracing::info!("You are already logged in");
            return Ok(context.api.auth_token().unwrap());
        };

        let mut api = context.api;

        // ...or trying to use arguments to either get token or user-pass
        let token = match (self.token, self.user, self.pass) {
            // Login using existing token, use get_profile route for that
            (Some(token), _, _) => {
                tracing::info!("Token passed, checking...");

                api.set_auth_token(token.clone());

                let is_ok = api.run_route(ProfileRoute {})?;
                if !is_ok {
                    return Err(anyhow!("Supplied token is incorrect"));
                };

                token
            }

            // Login using username and password
            (None, Some(email_username), Some(password)) => {
                let login = LoginRoute {
                    email_username,
                    password,
                };

                let res = api.run_route(login)?;
                let mut res: HashMap<String, String> = res.json()?;

                let tok_opt = res.remove("token");
                if tok_opt.is_none() {
                    return Err(anyhow!("Unable to get token"));
                };

                tok_opt.unwrap()
            }

            // In case token or login/pass were not passed as arguments
            (_, _, _) => return Err(anyhow!("No credentials provided")),
        };

        // write token either after logging or if it was passed
        write_token(token.as_str())?;

        tracing::info!("Success! You are now logged in!");

        Ok(token)
    }
}
