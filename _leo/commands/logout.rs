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
//  leo logout
//

#[derive(Debug)]
pub struct LogoutCommand;

use crate::{cli::CLI, cli_types::*, config::remove_token, errors::CLIError};
use std::io::ErrorKind;

impl CLI for LogoutCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Logout from Aleo Package Manager";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "logout";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    /// no options and no arguments for this buddy
    fn parse(_: &clap::ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    /// as simple as it could be - remove credentials file
    fn output(_: Self::Options) -> Result<Self::Output, CLIError> {
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
