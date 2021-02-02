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

use crate::config::remove_token;
use anyhow::Error;
use std::io::ErrorKind;
use structopt::StructOpt;

/// Remove credentials for Aleo PM from .leo directory
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Logout {}

impl Logout {
    pub fn new() -> Logout {
        Logout {}
    }
}

impl Cmd for Logout {
    type Output = ();

    fn apply(self, _ctx: Context) -> Result<Self::Output, Error> {
        // we gotta do something about this span issue :confused:
        let span = tracing::span!(tracing::Level::INFO, "Logout");
        let _ent = span.enter();

        // the only error we're interested here is NotFound
        // however err in this case can also be of kind PermissionDenied or other
        if let Err(err) = remove_token() {
            match err.kind() {
                ErrorKind::NotFound => {
                    tracing::info!("you are not logged in");
                    Ok(())
                }
                ErrorKind::PermissionDenied => {
                    tracing::error!("permission denied - check file permission in .leo folder");
                    Ok(())
                }
                _ => {
                    tracing::error!("something went wrong, can't access the file");
                    Ok(())
                }
            }
        } else {
            tracing::info!("success");
            Ok(())
        }
    }
}
