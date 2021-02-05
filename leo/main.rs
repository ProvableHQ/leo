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
pub mod command;
pub mod config;
pub mod context;
pub mod logger;
pub mod synthesizer;
pub mod updater;

use anyhow::Error;
use std::process::exit;

use command::{
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
        cmd: Init,
    },

    #[structopt(about = "Create new Leo project in new directory")]
    New {
        #[structopt(flatten)]
        cmd: New,
    },

    #[structopt(about = "Compile current package as a program")]
    Build {
        #[structopt(flatten)]
        cmd: Build,
    },

    #[structopt(about = "Run a program setup")]
    Setup {
        #[structopt(flatten)]
        cmd: Setup,
    },

    #[structopt(about = "Run the program and produce a proof")]
    Prove {
        #[structopt(flatten)]
        cmd: Prove,
    },

    #[structopt(about = "Run a program with input variables")]
    Run {
        #[structopt(flatten)]
        cmd: Run,
    },

    #[structopt(about = "Clean current package: remove proof and circuits")]
    Clean {
        #[structopt(flatten)]
        cmd: Clean,
    },

    #[structopt(about = "Watch for changes of Leo source files and run build")]
    Watch {
        #[structopt(flatten)]
        cmd: Watch,
    },

    #[structopt(about = "Watch for changes of Leo source files and run build")]
    Update {
        #[structopt(flatten)]
        cmd: Update,
    },

    #[structopt(about = "Compile and run all tests in the current package")]
    Test {
        #[structopt(flatten)]
        cmd: Test,
    },

    #[structopt(about = "Import package from Aleo PM")]
    Add {
        #[structopt(flatten)]
        cmd: Add,
    },

    #[structopt(about = "Login to the package manager and store credentials")]
    Login {
        #[structopt(flatten)]
        cmd: Login,
    },

    #[structopt(about = "Logout - remove local credentials")]
    Logout {
        #[structopt(flatten)]
        cmd: Logout,
    },

    #[structopt(about = "Publish package")]
    Publish {
        #[structopt(flatten)]
        cmd: Publish,
    },

    #[structopt(about = "Remove imported package")]
    Remove {
        #[structopt(flatten)]
        cmd: Remove,
    },

    #[structopt(about = "Lint package code (not implemented)")]
    Lint {
        #[structopt(flatten)]
        cmd: Lint,
    },

    #[structopt(about = "Deploy the current package as a program to the network (*)")]
    Deploy {
        #[structopt(flatten)]
        cmd: Deploy,
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
        CommandOpts::Init { cmd } => cmd.try_execute(),
        CommandOpts::New { cmd } => cmd.try_execute(),
        CommandOpts::Build { cmd } => cmd.try_execute(),
        CommandOpts::Setup { cmd } => cmd.try_execute(),
        CommandOpts::Prove { cmd } => cmd.try_execute(),
        CommandOpts::Test { cmd } => cmd.try_execute(),
        CommandOpts::Run { cmd } => cmd.try_execute(),
        CommandOpts::Clean { cmd } => cmd.try_execute(),
        CommandOpts::Watch { cmd } => cmd.try_execute(),
        CommandOpts::Update { cmd } => cmd.try_execute(),

        CommandOpts::Add { cmd } => cmd.try_execute(),
        CommandOpts::Login { cmd } => cmd.try_execute(),
        CommandOpts::Logout { cmd } => cmd.try_execute(),
        CommandOpts::Publish { cmd } => cmd.try_execute(),
        CommandOpts::Remove { cmd } => cmd.try_execute(),

        CommandOpts::Lint { cmd } => cmd.try_execute(),
        CommandOpts::Deploy { cmd } => cmd.try_execute(),
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
