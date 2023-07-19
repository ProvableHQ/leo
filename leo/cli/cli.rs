// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::cli::{commands::*, context::*, helpers::*};
use leo_errors::Result;

use clap::Parser;
use std::{path::PathBuf, process::exit};

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Aleo Team <hello@aleo.org>", version)]
pub struct CLI {
    #[clap(short, global = true, help = "Print additional information for debugging")]
    debug: bool,

    #[clap(short, global = true, help = "Suppress CLI output")]
    quiet: bool,

    #[clap(subcommand)]
    command: Commands,

    #[clap(help = "Custom Aleo PM backend URL", env = "APM_URL")]
    api: Option<String>,

    #[clap(long, global = true, help = "Optional path to Leo program root folder")]
    path: Option<PathBuf>,
}

///Leo compiler and package manager
#[derive(Parser, Debug)]
enum Commands {
    // #[clap(about = "Create a new Leo package in an existing directory")]
    // Init {
    //     #[clap(flatten)]
    //     command: Init,
    // },
    //
    #[clap(about = "Create a new Leo package in a new directory")]
    New {
        #[clap(flatten)]
        command: New,
    },
    #[clap(about = "Compile the current package as a program")]
    Build {
        #[clap(flatten)]
        command: Build,
    },
    #[clap(about = "Clean the output directory")]
    Clean {
        #[clap(flatten)]
        command: Clean,
    },
    #[clap(about = "Run a program with input variables")]
    Run {
        #[clap(flatten)]
        command: Run,
    },
    #[clap(about = "Execute a program with input variables")]
    Execute {
        #[clap(flatten)]
        command: Execute,
    },
    #[clap(about = "Update the Leo CLI")]
    Update {
        #[clap(flatten)]
        command: Update,
    },
    // #[clap(subcommand)]
    // Node(Node),
    // #[clap(about = "Deploy a program")]
    // Deploy {
    //     #[clap(flatten)]
    //     command: Deploy,
    // },
}

pub fn handle_error<T>(res: Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{err}");
            exit(err.exit_code());
        }
    }
}

/// Run command with custom build arguments.
pub fn run_with_args(cli: CLI) -> Result<()> {
    if !cli.quiet {
        // Init logger with optional debug flag.
        logger::init_logger("leo", match cli.debug {
            false => 1,
            true => 2,
        })?;
    }

    // Get custom root folder and create context for it.
    // If not specified, default context will be created in cwd.
    let context = handle_error(Context::new(cli.path));

    match cli.command {
        Commands::New { command } => command.try_execute(context),
        Commands::Build { command } => command.try_execute(context),
        Commands::Clean { command } => command.try_execute(context),
        Commands::Run { command } => command.try_execute(context),
        Commands::Execute { command } => command.try_execute(context),
        Commands::Update { command } => command.try_execute(context),
        // Commands::Node(command) => command.try_execute(context),
        // Commands::Deploy { command } => command.try_execute(context),
    }
}
