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

pub mod api;
pub mod cmd;
pub mod config;
pub mod context;
pub mod logger;
pub mod synthesizer;

use anyhow::Error;
use std::process::exit;

use cmd::{
    build::Build,
    clean::Clean,
    deploy::Deploy,
    init::Init,
    lint::Lint,
    new::New,
    package::{Add, Login, Logout, Publish, Remove},
    prove::Prove,
    run::Run,
    setup::Setup,
    test::Test,
    watch::Watch,
    Cmd,
};

use structopt::{clap::AppSettings, StructOpt};

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::ColoredHelp)]
struct Opt {
    #[structopt(short, long, help = "Print additional information for debugging")]
    debug: bool,

    #[structopt(flatten)]
    command: Command,
}

/// Leo commands (subcommands for Opt)
#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::ColoredHelp)]
enum Command {
    #[structopt(about = "Init Leo project command in current directory")]
    Init {
        #[structopt(flatten)]
        cmd: Init,
    },

    #[structopt(about = "Create Leo project in new directory")]
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

    #[structopt(about = "Login to the Aleo Package Manager")]
    Login {
        #[structopt(flatten)]
        cmd: Login,
    },

    #[structopt(about = "Logout from Aleo Package Manager")]
    Logout {
        #[structopt(flatten)]
        cmd: Logout,
    },

    #[structopt(about = "Publish package to Aleo PM")]
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

    logger::init_logger("leo", match opt.debug {
        false => 1,
        true => 2,
    });

    handle_error(match opt.command {
        Command::Init { cmd } => cmd.execute(),
        Command::New { cmd } => cmd.execute(),
        Command::Build { cmd } => cmd.execute(),
        Command::Setup { cmd } => cmd.execute(),
        Command::Prove { cmd } => cmd.execute(),
        Command::Test { cmd } => cmd.execute(),
        Command::Run { cmd } => cmd.execute(),
        Command::Clean { cmd } => cmd.execute(),
        Command::Watch { cmd } => cmd.execute(),

        Command::Add { cmd } => cmd.execute(),
        Command::Login { cmd } => cmd.execute(),
        Command::Logout { cmd } => cmd.execute(),
        Command::Publish { cmd } => cmd.execute(),
        Command::Remove { cmd } => cmd.execute(),

        Command::Lint { cmd } => cmd.execute(),
        Command::Deploy { cmd } => cmd.execute(),
    });
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            tracing::error!("Error: {:?}", err);
            exit(1);
        }
    }
}
