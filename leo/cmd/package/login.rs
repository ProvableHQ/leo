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

use crate::{cmd::Cmd, context::Context};

use crate::api::Login as LoginRoute;

use crate::config::*;

use anyhow::{anyhow, Error};
use structopt::StructOpt;
use tracing::Span;

use std::collections::HashMap;

pub const LOGIN_URL: &str = "v1/account/authenticate";
pub const PROFILE_URL: &str = "v1/account/my_profile";

/// Login to Aleo PM and store credentials locally
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Login {
    #[structopt(short = "u", long = "user", about = "Username for Aleo PM")]
    user: Option<String>,

    #[structopt(short = "p", long = "password", about = "Password for Aleo PM")]
    pass: Option<String>,
}

impl Login {
    pub fn new(user: Option<String>, pass: Option<String>) -> Login {
        Login { user, pass }
    }
}

impl Cmd for Login {
    type Input = ();
    type Output = String;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Login")
    }

    fn prelude(&self) -> Result<Self::Input, Error> {
        Ok(())
    }

    fn apply(self, ctx: Context, _: Self::Input) -> Result<Self::Output, Error> {
        let token = match (self.user, self.pass) {
            // Login using existing token, use get_profile route for that
            // (Some(token), _, _) => Some(token),
            // unimplemented!

            // Login using username and password
            (Some(email_username), Some(password)) => {
                let login = LoginRoute {
                    email_username,
                    password,
                };

                let res = ctx.api.run_route(login)?;
                let mut res: HashMap<String, String> = res.json()?;

                let token = match res.remove("token") {
                    Some(token) => token,
                    None => {
                        return Err(anyhow!("Unable to get token"));
                    }
                };

                write_token(token.as_str())?;

                tracing::info!("Success! You are now logged in!");

                token
            }

            // Login using stored JWT credentials.
            // TODO (raychu86) Package manager re-authentication from token
            (_, _) => read_token().map_err(|_| -> Error { anyhow!("No credentials provided") })?,
        };

        Ok(token)
    }
}
