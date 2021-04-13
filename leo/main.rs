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

pub mod api;
pub mod commands;
pub mod config;
pub mod context;
pub mod logger;
pub mod updater;

use commands::{
    package::{Add, Clone, Login, Logout, Publish, Remove},
    Build,
    Clean,
    Command,
    Deploy,
    Init,
    Lint,
    New,
    Prove,
    Run,
    Setup,
    Test,
    Update,
    Watch,
};

use anyhow::Error;
use std::{path::PathBuf, process::exit};
use structopt::{clap::AppSettings, StructOpt};

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(name = "leo", author = "The Aleo Team <hello@aleo.org>", setting = AppSettings::ColoredHelp)]
struct Opt {
    #[structopt(short, global = true, help = "Print additional information for debugging")]
    debug: bool,

    #[structopt(short, global = true, help = "Suppress CLI output")]
    quiet: bool,

    #[structopt(subcommand)]
    command: CommandOpts,

    #[structopt(
        long,
        global = true,
        help = "Optional path to Leo program root folder",
        parse(from_os_str)
    )]
    path: Option<PathBuf>,
}

///Leo compiler and package manager
#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::ColoredHelp)]
enum CommandOpts {
    #[structopt(about = "Create a new Leo package in an existing directory")]
    Init {
        #[structopt(flatten)]
        command: Init,
    },

    #[structopt(about = "Create a new Leo package in a new directory")]
    New {
        #[structopt(flatten)]
        command: New,
    },

    #[structopt(about = "Compile the current package as a program")]
    Build {
        #[structopt(flatten)]
        command: Build,
    },

    #[structopt(about = "Run a program setup")]
    Setup {
        #[structopt(flatten)]
        command: Setup,
    },

    #[structopt(about = "Run the program and produce a proof")]
    Prove {
        #[structopt(flatten)]
        command: Prove,
    },

    #[structopt(about = "Run a program with input variables")]
    Run {
        #[structopt(flatten)]
        command: Run,
    },

    #[structopt(about = "Clean the output directory")]
    Clean {
        #[structopt(flatten)]
        command: Clean,
    },

    #[structopt(about = "Watch for changes of Leo source files")]
    Watch {
        #[structopt(flatten)]
        command: Watch,
    },

    #[structopt(about = "Update Leo to the latest version")]
    Update {
        #[structopt(flatten)]
        command: Update,
    },

    #[structopt(about = "Compile and run all tests in the current package")]
    Test {
        #[structopt(flatten)]
        command: Test,
    },

    #[structopt(about = "Import a package from the Aleo Package Manager")]
    Add {
        #[structopt(flatten)]
        command: Add,
    },

    #[structopt(about = "Clone a package from the Aleo Package Manager")]
    Clone {
        #[structopt(flatten)]
        command: Clone,
    },

    #[structopt(about = "Login to the Aleo Package Manager")]
    Login {
        #[structopt(flatten)]
        command: Login,
    },

    #[structopt(about = "Logout of the Aleo Package Manager")]
    Logout {
        #[structopt(flatten)]
        command: Logout,
    },

    #[structopt(about = "Publish the current package to the Aleo Package Manager")]
    Publish {
        #[structopt(flatten)]
        command: Publish,
    },

    #[structopt(about = "Uninstall a package from the current package")]
    Remove {
        #[structopt(flatten)]
        command: Remove,
    },

    #[structopt(about = "Lints the Leo files in the package (*)")]
    Lint {
        #[structopt(flatten)]
        command: Lint,
    },

    #[structopt(about = "Deploy the current package as a program to the network (*)")]
    Deploy {
        #[structopt(flatten)]
        command: Deploy,
    },
}

fn main() {
    // Read command line arguments.
    let opt = Opt::from_args();

    if !opt.quiet {
        // Init logger with optional debug flag.
        logger::init_logger("leo", match opt.debug {
            false => 1,
            true => 2,
        });
    }

    // Get custom root folder and create context for it.
    // If not specified, default context will be created in cwd.
    let context = handle_error(match opt.path {
        Some(path) => context::create_context(path),
        None => context::get_context(),
    });

    handle_error(match opt.command {
        CommandOpts::Init { command } => command.try_execute(context),
        CommandOpts::New { command } => command.try_execute(context),
        CommandOpts::Build { command } => command.try_execute(context),
        CommandOpts::Setup { command } => command.try_execute(context),
        CommandOpts::Prove { command } => command.try_execute(context),
        CommandOpts::Test { command } => command.try_execute(context),
        CommandOpts::Run { command } => command.try_execute(context),
        CommandOpts::Clean { command } => command.try_execute(context),
        CommandOpts::Watch { command } => command.try_execute(context),
        CommandOpts::Update { command } => command.try_execute(context),

        CommandOpts::Add { command } => command.try_execute(context),
        CommandOpts::Clone { command } => command.try_execute(context),
        CommandOpts::Login { command } => command.try_execute(context),
        CommandOpts::Logout { command } => command.try_execute(context),
        CommandOpts::Publish { command } => command.try_execute(context),
        CommandOpts::Remove { command } => command.try_execute(context),

        CommandOpts::Lint { command } => command.try_execute(context),
        CommandOpts::Deploy { command } => command.try_execute(context),
    });
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}
