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
use clap::Parser;
use colored::Colorize;
use leo_errors::Result;
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

    #[clap(long, global = true, help = "Optional path to Leo program root folder")]
    path: Option<PathBuf>,

    #[clap(long, global = true, help = "Optional path to aleo program registry.")]
    pub home: Option<PathBuf>,
}

///Leo compiler and package manager
#[derive(Parser, Debug)]
enum Commands {
    #[clap(about = "Add a new dependency to the current package. Defaults to testnet3 network")]
    Add {
        #[clap(flatten)]
        command: Add,
    },
    #[clap(about = "Create a new Aleo account")]
    Account {
        #[clap(subcommand)]
        command: Account,
    },
    #[clap(about = "Create a new Leo package in a new directory")]
    New {
        #[clap(flatten)]
        command: New,
    },
    #[clap(about = "Create a new Leo example package in a new directory")]
    Example {
        #[clap(subcommand)]
        command: Example,
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
    let context = handle_error(Context::new(cli.path, cli.home));

    match cli.command {
        Commands::Add { command } => command.try_execute(context),
        Commands::Account { command } => command.try_execute(context),
        Commands::New { command } => command.try_execute(context),
        Commands::Build { command } => {
            // Enter tracing span
            let span = command.log_span();
            let span = span.enter();

            // Leo build is deprecated in version 1.9.0
            tracing::info!(
                "⚠️  Attention - This command is deprecated. Use the {} command.\n",
                "'run'".to_string().bold()
            );

            // Drop tracing span
            drop(span);

            command.try_execute(context)
        }
        Commands::Clean { command } => command.try_execute(context),
        Commands::Example { command } => command.try_execute(context),
        Commands::Run { command } => command.try_execute(context),
        Commands::Execute { command } => command.try_execute(context),
        Commands::Update { command } => command.try_execute(context),
    }
}
#[cfg(test)]
mod tests {
    use crate::cli::{
        cli::{test_helpers, Commands},
        run_with_args,
        CLI,
    };
    use leo_span::symbol::create_session_if_not_set_then;
    use serial_test::serial;
    use std::env::temp_dir;

    #[test]
    #[serial]
    fn nested_network_dependency_run_test() {
        // Set current directory to temporary directory
        let temp_dir = temp_dir();
        let project_directory = temp_dir.join("nested");

        // Create file structure
        test_helpers::sample_nested_package(&temp_dir);

        // Run program
        let run = CLI {
            debug: false,
            quiet: false,
            command: Commands::Run {
                command: crate::cli::commands::Run {
                    name: "example".to_string(),
                    inputs: vec!["1u32".to_string(), "2u32".to_string()],
                    file: None,
                    compiler_options: Default::default(),
                },
            },
            path: Some(project_directory.clone()),
            home: Some(temp_dir.join(".aleo")),
        };

        create_session_if_not_set_then(|_| {
            run_with_args(run).expect("Failed to execute `leo run`");
        });

        // TODO: Clear tmp directory
        // let registry = temp_dir.join(".aleo").join("registry").join("testnet3");
        // std::fs::remove_dir_all(registry).unwrap();
        // std::fs::remove_dir_all(project_directory).unwrap();
    }

    #[test]
    #[serial]
    fn nested_local_dependency_run_test() {
        // Set current directory to temporary directory
        let temp_dir = temp_dir();
        let project_name = "grandparent";
        let project_directory = temp_dir.join(project_name);

        // Remove it if it already exists
        if project_directory.exists() {
            std::fs::remove_dir_all(project_directory.clone()).unwrap();
        }

        // Create file structure
        test_helpers::sample_grandparent_package(&temp_dir);

        // Run program
        let run = CLI {
            debug: false,
            quiet: false,
            command: Commands::Run {
                command: crate::cli::commands::Run {
                    name: "double_wrapper_mint".to_string(),
                    inputs: vec![
                        "aleo13tngrq7506zwdxj0cxjtvp28pk937jejhne0rt4zp0z370uezuysjz2prs".to_string(),
                        "2u32".to_string(),
                    ],
                    file: None,
                    compiler_options: Default::default(),
                },
            },
            path: Some(project_directory.clone()),
            home: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(run).expect("Failed to execute `leo run`");
        });

        // TODO: Clear tmp directory
        // std::fs::remove_dir_all(project_directory).unwrap();
    }
}

#[cfg(test)]
mod test_helpers {
    use crate::cli::{cli::Commands, run_with_args, Add, New, CLI};
    use leo_span::symbol::create_session_if_not_set_then;
    use std::path::Path;

    pub(crate) fn sample_nested_package(temp_dir: &Path) {
        let name = "nested";

        // Remove it if it already exists
        let project_directory = temp_dir.join(name);
        if project_directory.exists() {
            std::fs::remove_dir_all(project_directory.clone()).unwrap();
        }

        // Create new Leo project
        let new = CLI {
            debug: false,
            quiet: false,
            command: Commands::New { command: New { name: name.to_string() } },
            path: Some(project_directory.clone()),
            home: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(new).expect("Failed to execute `leo run`");
        });

        // `nested.aleo` program
        let program_str = "
import nested_example_layer_0.aleo;
program nested.aleo {
    transition example(public a: u32, b: u32) -> u32 {
        let c: u32 = nested_example_layer_0.aleo/main(a, b);
        return c;
    }
}
";
        // `nested_example_layer_0.aleo` program
        let nested_example_layer_0 = "
import nested_example_layer_2.aleo;
import nested_example_layer_1.aleo;

program nested_example_layer_0.aleo;

function main:
    input r0 as u32.public;
    input r1 as u32.private;
    call nested_example_layer_1.aleo/external_function r0 r1 into r2;
    output r2 as u32.private;
";

        // `nested_example_layer_1.aleo` program
        let nested_example_layer_1 = "
import nested_example_layer_2.aleo;

program nested_example_layer_1.aleo;

function external_function:
    input r0 as u32.public;
    input r1 as u32.private;
    call nested_example_layer_2.aleo/external_nested_function r0 r1 into r2;
    output r2 as u32.private;
";

        // `nested_example_layer_2.aleo` program
        let nested_example_layer_2 = "
program nested_example_layer_2.aleo;

function external_nested_function:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

        // Overwrite `src/main.leo` file
        std::fs::write(project_directory.join("src").join("main.leo"), program_str).unwrap();

        // Add dependencies
        let add = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "nested_example_layer_0".to_string(),
                    local: None,
                    network: "testnet3".to_string(),
                },
            },
            path: Some(project_directory.clone()),
            home: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(add).expect("Failed to execute `leo add`");
        });

        // Add custom `.aleo` directory
        let registry = temp_dir.join(".aleo").join("registry").join("testnet3");
        std::fs::create_dir_all(&registry).unwrap();
        std::fs::write(registry.join("nested_example_layer_0.aleo"), nested_example_layer_0).unwrap();
        std::fs::write(registry.join("nested_example_layer_1.aleo"), nested_example_layer_1).unwrap();
        std::fs::write(registry.join("nested_example_layer_2.aleo"), nested_example_layer_2).unwrap();
    }

    pub(crate) fn sample_grandparent_package(temp_dir: &Path) {
        let grandparent_directory = temp_dir.join("grandparent");
        let parent_directory = grandparent_directory.join("parent");
        let child_directory = parent_directory.join("child");

        if grandparent_directory.exists() {
            std::fs::remove_dir_all(grandparent_directory.clone()).unwrap();
        }

        // Create project file structure `grandparent/parent/child`
        let create_grandparent_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New { command: New { name: "grandparent".to_string() } },
            path: Some(grandparent_directory.clone()),
            home: None,
        };

        let create_parent_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New { command: New { name: "parent".to_string() } },
            path: Some(parent_directory.clone()),
            home: None,
        };

        let create_child_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New { command: New { name: "child".to_string() } },
            path: Some(child_directory.clone()),
            home: None,
        };

        // Add source files `grandparent/src/main.leo`, `grandparent/parent/src/main.leo`, and `grandparent/parent/child/src/main.leo`
        let grandparent_program = "
import child.aleo;
import parent.aleo;
program grandparent.aleo {
    transition double_wrapper_mint(owner: address, val: u32) -> child.aleo/A {
        return parent.aleo/wrapper_mint(owner, val);
    }
}
";
        let parent_program = "
import child.aleo;
program parent.aleo {
    transition wrapper_mint(owner: address, val: u32) ->  child.aleo/A {
        return child.aleo/mint(owner, val);
    }
}
";

        let child_program = "
// The 'a' program.
program child.aleo {
    record A {
        owner: address,
        val: u32,
    }
    transition mint(owner: address, val: u32) -> A {
        return A {owner: owner, val: val};
    }
}
";

        // Add dependencies `grandparent/program.json` and `grandparent/parent/program.json`
        let add_grandparent_dependency_1 = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "parent".to_string(),
                    local: Some(parent_directory.clone()),
                    network: "testnet3".to_string(),
                },
            },
            path: Some(grandparent_directory.clone()),
            home: None,
        };

        let add_grandparent_dependency_2 = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "child".to_string(),
                    local: Some(child_directory.clone()),
                    network: "testnet3".to_string(),
                },
            },
            path: Some(grandparent_directory.clone()),
            home: None,
        };

        let add_parent_dependency = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "child".to_string(),
                    local: Some(child_directory.clone()),
                    network: "testnet3".to_string(),
                },
            },
            path: Some(parent_directory.clone()),
            home: None,
        };

        // Execute all commands
        create_session_if_not_set_then(|_| {
            // Create projects
            run_with_args(create_grandparent_project).unwrap();
            run_with_args(create_parent_project).unwrap();
            run_with_args(create_child_project).unwrap();

            // Write files
            std::fs::write(grandparent_directory.join("src").join("main.leo"), grandparent_program).unwrap();
            std::fs::write(parent_directory.join("src").join("main.leo"), parent_program).unwrap();
            std::fs::write(child_directory.join("src").join("main.leo"), child_program).unwrap();

            // Add dependencies
            run_with_args(add_grandparent_dependency_1).unwrap();
            run_with_args(add_grandparent_dependency_2).unwrap();
            run_with_args(add_parent_dependency).unwrap();
        });
    }
}
