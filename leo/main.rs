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

use leo::{cli::*, commands::*, errors::CLIError};

use clap::{App, AppSettings, Arg};

#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), CLIError> {
    let arguments = App::new("leo")
        .version("v0.1.0")
        .about("Leo compiler and package manager")
        .author("The Aleo Team <hello@aleo.org>")
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableVersion,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .args(&[Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Enables debugging mode")
            .global(true)])
        .subcommands(vec![
            NewCommand::new().display_order(0),
            InitCommand::new().display_order(1),
            BuildCommand::new().display_order(2),
            WatchCommand::new().display_order(3),
            TestCommand::new().display_order(4),
            SetupCommand::new().display_order(5),
            ProveCommand::new().display_order(6),
            RunCommand::new().display_order(7),
            LoginCommand::new().display_order(8),
            AddCommand::new().display_order(9),
            RemoveCommand::new().display_order(10),
            PublishCommand::new().display_order(11),
            DeployCommand::new().display_order(12),
            CleanCommand::new().display_order(13),
            LintCommand::new().display_order(14),
        ])
        .set_term_width(0)
        .get_matches();

    match arguments.subcommand() {
        ("new", Some(arguments)) => NewCommand::process(arguments),
        ("init", Some(arguments)) => InitCommand::process(arguments),
        ("build", Some(arguments)) => BuildCommand::process(arguments),
        ("watch", Some(arguments)) => WatchCommand::process(arguments),
        ("test", Some(arguments)) => TestCommand::process(arguments),
        ("setup", Some(arguments)) => SetupCommand::process(arguments),
        ("prove", Some(arguments)) => ProveCommand::process(arguments),
        ("run", Some(arguments)) => RunCommand::process(arguments),
        ("login", Some(arguments)) => LoginCommand::process(arguments),
        ("add", Some(arguments)) => AddCommand::process(arguments),
        ("remove", Some(arguments)) => RemoveCommand::process(arguments),
        ("publish", Some(arguments)) => PublishCommand::process(arguments),
        ("deploy", Some(arguments)) => DeployCommand::process(arguments),
        ("clean", Some(arguments)) => CleanCommand::process(arguments),
        ("lint", Some(arguments)) => LintCommand::process(arguments),
        _ => unreachable!(),
    }
}
