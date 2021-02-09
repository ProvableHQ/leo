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

use std::process::exit;

use anyhow::Error;
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
    #[structopt(about = "Init new Leo project command in current directory")]
    Init {
        #[structopt(flatten)]
        command: Init,
    },

    #[structopt(about = "Create new Leo project in new directory")]
    New {
        #[structopt(flatten)]
        command: New,
    },

    #[structopt(about = "Compile current package as a program")]
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

    #[structopt(about = "Clean current package: remove proof and circuits")]
    Clean {
        #[structopt(flatten)]
        command: Clean,
    },

    #[structopt(about = "Watch for changes of Leo source files and run build")]
    Watch {
        #[structopt(flatten)]
        command: Watch,
    },

    #[structopt(about = "Watch for changes of Leo source files and run build")]
    Update {
        #[structopt(flatten)]
        command: Update,
    },

    #[structopt(about = "Compile and run all tests in the current package")]
    Test {
        #[structopt(flatten)]
        command: Test,
    },

    #[structopt(about = "Import package from Aleo PM")]
    Add {
        #[structopt(flatten)]
        command: Add,
    },

    #[structopt(about = "Login to the package manager and store credentials")]
    Login {
        #[structopt(flatten)]
        command: Login,
    },

    #[structopt(about = "Logout - remove local credentials")]
    Logout {
        #[structopt(flatten)]
        command: Logout,
    },

    #[structopt(about = "Publish package")]
    Publish {
        #[structopt(flatten)]
        command: Publish,
    },

    #[structopt(about = "Remove imported package")]
    Remove {
        #[structopt(flatten)]
        command: Remove,
    },

    #[structopt(about = "Lint package code (not implemented)")]
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
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}
