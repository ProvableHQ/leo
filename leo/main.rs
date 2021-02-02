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
    add::Add,
    build::Build,
    clean::Clean,
    deploy::Deploy,
    init::Init,
    lint::Lint,
    new::New,
    prove::Prove,
    run::Run,
    setup::Setup,
    watch::Watch,
    Cmd,
};

use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::ColoredHelp)]
enum Opt {
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

    #[structopt(about = "Import package from Aleo PM")]
    Add {
        #[structopt(flatten)]
        cmd: Add,
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
    // before even starting to match we init logger...
    // TODO: add debug option here!
    logger::init_logger("leo", 1);

    let matches = Opt::from_args();

    handle_error(match matches {
        Opt::Init { cmd } => cmd.execute(),
        Opt::New { cmd } => cmd.execute(),
        Opt::Add { cmd } => cmd.execute(),
        Opt::Build { cmd } => cmd.execute(),
        Opt::Setup { cmd } => cmd.execute(),
        Opt::Prove { cmd } => cmd.execute(),
        Opt::Run { cmd } => cmd.execute(),
        Opt::Clean { cmd } => cmd.execute(),
        Opt::Watch { cmd } => cmd.execute(),
        Opt::Lint { cmd } => cmd.execute(),
        Opt::Deploy { cmd } => cmd.execute(),
    });
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("error: {:?}", err);
            exit(1);
        }
    }
}
