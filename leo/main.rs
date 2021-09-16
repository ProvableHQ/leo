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
    package::{Clone, Fetch, Login, Logout, Publish},
    Build, Clean, Command, Deploy, Init, Lint, New, Prove, Run, Setup, Test, Update, Watch,
};
use leo_errors::Result;

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

    // #[structopt(about = "Import a package from the Aleo Package Manager")]
    // Add {
    //     #[structopt(flatten)]
    //     command: Add,
    // },
    #[structopt(about = "Pull dependencies from Aleo Package Manager")]
    Fetch {
        #[structopt(flatten)]
        command: Fetch,
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

    // #[structopt(about = "Uninstall a package from the current package")]
    // Remove {
    //     #[structopt(flatten)]
    //     command: Remove,
    // },
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
    handle_error(run_with_args(Opt::from_args()))
}

/// Run command with custom build arguments.
fn run_with_args(opt: Opt) -> Result<()> {
    if !opt.quiet {
        // Init logger with optional debug flag.
        logger::init_logger(
            "leo",
            match opt.debug {
                false => 1,
                true => 2,
            },
        )?;
    }

    // Get custom root folder and create context for it.
    // If not specified, default context will be created in cwd.
    let context = handle_error(match opt.path {
        Some(path) => context::create_context(path, opt.api),
        None => context::get_context(opt.api),
    });

    match opt.command {
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

        // CommandOpts::Add { command } => command.try_execute(context),
        CommandOpts::Fetch { command } => command.try_execute(context),
        CommandOpts::Clone { command } => command.try_execute(context),
        CommandOpts::Login { command } => command.try_execute(context),
        CommandOpts::Logout { command } => command.try_execute(context),
        CommandOpts::Publish { command } => command.try_execute(context),
        // CommandOpts::Remove { command } => command.try_execute(context),
        CommandOpts::Lint { command } => command.try_execute(context),
        CommandOpts::Deploy { command } => command.try_execute(context),
    }
}

fn handle_error<T>(res: Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{}", err);
            exit(err.exit_code());
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use crate::{run_with_args, Opt};
    use leo_errors::{CliError, Result};

    use snarkvm_utilities::Write;
    use std::path::PathBuf;
    use structopt::StructOpt;
    use test_dir::{DirBuilder, FileType, TestDir};

    // Runs Command from cmd-like argument "leo run --arg1 --arg2".
    fn run_cmd(args: &str, path: &Option<PathBuf>) -> Result<()> {
        let args = args.split(' ').collect::<Vec<&str>>();
        let mut opts = Opt::from_iter_safe(args).map_err(CliError::opt_args_error)?;

        if path.is_some() {
            opts.path = path.clone();
        }

        if !opts.debug {
            // turn off tracing for all tests
            opts.quiet = true;
        }

        run_with_args(opts)
    }

    // Create a test directory with name.
    fn testdir(name: &str) -> TestDir {
        TestDir::temp().create(name, FileType::Dir)
    }

    #[test]
    fn global_options() {
        let path = Some(PathBuf::from("examples/pedersen-hash"));

        assert!(run_cmd("leo build", &path).is_ok());
        assert!(run_cmd("leo -q build", &path).is_ok());
    }

    #[test]
    fn global_options_fail() {
        assert!(run_cmd("leo --path ../../examples/no-directory-there build", &None).is_err());
        assert!(run_cmd("leo -v build", &None).is_err());
    }

    #[test]
    fn init() {
        let dir = testdir("test");
        let path = Some(dir.path("test"));

        assert!(run_cmd("leo init", &path).is_ok());
        assert!(run_cmd("leo init", &path).is_err()); // 2nd time
    }

    #[test]
    fn init_fail() {
        let dir = testdir("incorrect_name");
        let path = Some(dir.path("incorrect_name"));
        let fake = Some(PathBuf::from("no_such_directory"));

        assert!(run_cmd("leo init", &fake).is_err());
        assert!(run_cmd("leo init", &path).is_err());
    }

    #[test]
    fn new() {
        let dir = testdir("new");
        let path = Some(dir.path("new"));

        assert!(run_cmd("leo new test", &path).is_ok());
        assert!(run_cmd("leo new test", &path).is_err()); // 2nd time
        assert!(run_cmd("leo new wrong_name", &path).is_err());
    }

    #[test]
    #[should_panic]
    fn unimplemented() {
        assert!(run_cmd("leo lint", &None).is_err());
        assert!(run_cmd("leo deploy", &None).is_err());
    }

    #[test]
    fn clean() {
        let path = &Some(PathBuf::from("examples/pedersen-hash"));

        assert!(run_cmd("leo build", path).is_ok());
        assert!(run_cmd("leo clean", path).is_ok());
    }

    #[test]
    fn build_optimizations() {
        let dir = testdir("build-test");
        let path = dir.path("build-test");

        assert!(run_cmd("leo new setup-test", &Some(path.clone())).is_ok());

        let build_path = &Some(path.join("setup-test"));

        assert!(run_cmd("leo build --disable-all-optimizations", build_path).is_ok());
        assert!(run_cmd("leo build --disable-code-elimination", build_path).is_ok());
        assert!(run_cmd("leo build --disable-constant-folding", build_path).is_ok());
    }

    #[test]
    fn setup_prove_run_clean() {
        let dir = testdir("test");
        let path = dir.path("test");

        assert!(run_cmd("leo new setup-test", &Some(path.clone())).is_ok());

        let setup_path = &Some(path.join("setup-test"));

        assert!(run_cmd("leo setup", setup_path).is_ok());
        assert!(run_cmd("leo setup", setup_path).is_ok());
        assert!(run_cmd("leo setup --skip-key-check", setup_path).is_ok());
        assert!(run_cmd("leo prove --skip-key-check", setup_path).is_ok());
        assert!(run_cmd("leo run --skip-key-check", setup_path).is_ok());
        assert!(run_cmd("leo clean", setup_path).is_ok());
    }

    #[test]
    #[ignore]
    fn test_import() {
        let dir = testdir("test");
        let path = dir.path("test");

        assert!(run_cmd("leo new import", &Some(path.clone())).is_ok());

        let import_path = &Some(path.join("import"));

        assert!(run_cmd("leo add no-package/definitely-no", import_path).is_err());
        assert!(run_cmd("leo add justice-league/u8u32", import_path).is_ok());
        assert!(run_cmd("leo remove u8u32", import_path).is_ok());
        assert!(run_cmd("leo add --author justice-league --package u8u32", import_path).is_ok());
        assert!(run_cmd("leo remove u8u32", import_path).is_ok());
        assert!(run_cmd("leo remove u8u32", import_path).is_err());
    }

    #[test]
    fn test_missing_file() {
        let dir = testdir("test");
        let path = dir.path("test");

        assert!(run_cmd("leo new test-file-missing", &Some(path.clone())).is_ok());

        let path = path.join("test-file-missing");
        let file = path.join("src/main.leo");
        let path = Some(path);

        assert!(run_cmd("leo test", &path).is_ok());
        std::fs::remove_file(&file).unwrap();
        assert!(run_cmd("leo test", &path).is_err());
    }

    #[test]
    fn test_sudoku() {
        let path = &Some(PathBuf::from("examples/silly-sudoku"));

        assert!(run_cmd("leo build", path).is_ok());
        assert!(run_cmd("leo test", path).is_ok());
        assert!(run_cmd("leo test -f examples/silly-sudoku/src/lib.leo", path).is_ok());
        assert!(run_cmd("leo test -f examples/silly-sudoku/src/main.leo", path).is_ok());
    }

    #[test]
    fn test_install() {
        let dir = testdir("test");
        let path = dir.path("test");

        assert!(run_cmd("leo new install", &Some(path.clone())).is_ok());

        let install_path = &Some(path.join("install"));

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path.join("install/Leo.toml"))
            .unwrap();

        assert!(run_cmd("leo fetch", install_path).is_ok());
        assert!(file
            .write_all(
                br#"
            sudoku = {author = "justice-league", package = "u8u32", version = "0.1.0"}
        "#
            )
            .is_ok());

        assert!(run_cmd("leo fetch", install_path).is_ok());
        assert!(run_cmd("leo build", install_path).is_ok());
    }
}
