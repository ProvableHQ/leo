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

pub mod commands;
pub mod context;
pub mod logger;
pub mod updater;

use crate::commands::*;
use crate::context::*;
use leo_errors::Result;
use leo_span::symbol::create_session_if_not_set_then;

use clap::StructOpt;
use std::path::PathBuf;
use std::process::exit;

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(name = "leo", author = "The Aleo Team <hello@aleo.org>")]
pub struct CLI {
    #[structopt(short, global = true, help = "Print additional information for debugging")]
    debug: bool,

    #[structopt(short, global = true, help = "Suppress CLI output")]
    quiet: bool,

    #[structopt(subcommand)]
    command: Commands,

    #[structopt(help = "Custom Aleo PM backend URL", env = "APM_URL")]
    api: Option<String>,

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
enum Commands {
    // #[structopt(about = "Create a new Leo package in an existing directory")]
    // Init {
    //     #[structopt(flatten)]
    //     command: Init,
    // },
    //
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
    #[structopt(about = "Clean the output directory")]
    Clean {
        #[structopt(flatten)]
        command: Clean,
    },
    #[structopt(about = "Run a program with input variables")]
    Run {
        #[structopt(flatten)]
        command: Run,
    },
    // #[structopt(subcommand)]
    // Node(Node),
    #[structopt(about = "Deploy a program")]
    Deploy {
        #[structopt(flatten)]
        command: Deploy,
    },
}

fn set_panic_hook() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook({
        Box::new(move |e| {
            eprintln!(
                "thread `{}` {}",
                std::thread::current().name().unwrap_or("<unnamed>"),
                e
            );
            eprintln!("stack backtrace: \n{:?}", backtrace::Backtrace::new());
            eprintln!("error: internal compiler error: unexpected panic\n");
            eprintln!("note: the compiler unexpectedly panicked. this is a bug.\n");
            eprintln!("note: we would appreciate a bug report: https://github.com/AleoHQ/leo/issues/new?labels=bug,panic&template=bug.md&title=[Bug]\n");
            eprintln!(
                "note: {} {} running on {} {}\n",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                sys_info::os_type().unwrap_or_else(|e| e.to_string()),
                sys_info::os_release().unwrap_or_else(|e| e.to_string()),
            );
            eprintln!(
                "note: compiler args: {}\n",
                std::env::args().collect::<Vec<_>>().join(" ")
            );
            eprintln!("note: compiler flags: {:?}\n", CLI::parse());
        })
    });
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
        logger::init_logger(
            "leo",
            match cli.debug {
                false => 1,
                true => 2,
            },
        )?;
    }

    // Get custom root folder and create context for it.
    // If not specified, default context will be created in cwd.
    let context = handle_error(Context::new(cli.path));

    match cli.command {
        Commands::New { command } => command.try_execute(context),
        Commands::Build { command } => command.try_execute(context),
        Commands::Clean { command } => command.try_execute(context),
        Commands::Run { command } => command.try_execute(context),
        // Commands::Node(command) => command.try_execute(context),
        Commands::Deploy { command } => command.try_execute(context),
    }
}

fn main() {
    set_panic_hook();
    create_session_if_not_set_then(|_| handle_error(run_with_args(CLI::parse())));
}
