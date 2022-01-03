// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{commands::Command, config::remove_token_and_username, context::Context};
use leo_errors::Result;

use structopt::StructOpt;
use tracing::Span;

/// Remove credentials for Aleo PM from .leo directory
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Logout {}

impl<'a> Command<'a> for Logout {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Logout")
    }

    fn prelude(&self, _: Context<'a>) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _context: Context<'a>, _: Self::Input) -> Result<Self::Output> {
        // the only error we're interested here is NotFound
        // however err in this case can also be of kind PermissionDenied or other
        remove_token_and_username()?;
        tracing::info!("success");
        Ok(())
    }
}
