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
pub mod synthesizer;
pub mod updater;

use commands::{
    package::{Add, Login, Logout, Publish, Remove},
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
use std::process::exit;
use structopt::{clap::AppSettings, StructOpt};

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(name = "leo", author = "The Aleo Team <hello@aleo.org>", setting = AppSettings::ColoredHelp)]
struct Opt {
    #[structopt(short, long, help = "Print additional information for debugging")]
    debug: bool,

    #[structopt(short, long, help = "Suppress CLI output")]
    quiet: bool,

    #[structopt(subcommand)]
    command: CommandOpts,
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

    #[structopt(about = "Install a package from the Aleo Package Manager")]
    Add {
        #[structopt(flatten)]
        command: Add,
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
    // read command line arguments
    let opt = Opt::from_args();

    if !opt.quiet {
        // init logger with optional debug flag
        logger::init_logger("leo", match opt.debug {
            false => 1,
            true => 2,
        });
    }

    handle_error(match opt.command {
        CommandOpts::Init { command } => command.try_execute(),
        CommandOpts::New { command } => command.try_execute(),
        CommandOpts::Build { command } => command.try_execute(),
        CommandOpts::Setup { command } => command.try_execute(),
        CommandOpts::Prove { command } => command.try_execute(),
        CommandOpts::Test { command } => command.try_execute(),
        CommandOpts::Run { command } => command.try_execute(),
        CommandOpts::Clean { command } => command.try_execute(),
        CommandOpts::Watch { command } => command.try_execute(),
        CommandOpts::Update { command } => command.try_execute(),

        CommandOpts::Add { command } => command.try_execute(),
        CommandOpts::Login { command } => command.try_execute(),
        CommandOpts::Logout { command } => command.try_execute(),
        CommandOpts::Publish { command } => command.try_execute(),
        CommandOpts::Remove { command } => command.try_execute(),

        CommandOpts::Lint { command } => command.try_execute(),
        CommandOpts::Deploy { command } => command.try_execute(),
    });
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            println!("Error: {}", err);
            exit(1);
        }
    }
}
