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

    #[clap(long, global = true, help = "Path to Leo program root folder")]
    path: Option<PathBuf>,

    #[clap(long, global = true, help = "Path to aleo program registry")]
    pub home: Option<PathBuf>,
}

///Leo compiler and package manager
#[derive(Parser, Debug)]
enum Commands {
    #[clap(about = "Create a new Aleo account, sign and verify messages")]
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
        #[clap(flatten)]
        command: Example,
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
    #[clap(about = "Deploy a program")]
    Deploy {
        #[clap(flatten)]
        command: Deploy,
    },
    #[clap(about = "Query live data from the Aleo network")]
    Query {
        #[clap(flatten)]
        command: Query,
    },
    #[clap(about = "Compile the current package as a program")]
    Build {
        #[clap(flatten)]
        command: Build,
    },
    #[clap(about = "Add a new on-chain or local dependency to the current package.")]
    Add {
        #[clap(flatten)]
        command: Add,
    },
    #[clap(about = "Remove a dependency from the current package.")]
    Remove {
        #[clap(flatten)]
        command: Remove,
    },
    #[clap(about = "Clean the output directory")]
    Clean {
        #[clap(flatten)]
        command: Clean,
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
    let context = handle_error(Context::new(cli.path, cli.home, false));

    match cli.command {
        Commands::Add { command } => command.try_execute(context),
        Commands::Account { command } => command.try_execute(context),
        Commands::New { command } => command.try_execute(context),
        Commands::Build { command } => command.try_execute(context),
        Commands::Query { command } => command.try_execute(context),
        Commands::Clean { command } => command.try_execute(context),
        Commands::Deploy { command } => command.try_execute(context),
        Commands::Example { command } => command.try_execute(context),
        Commands::Run { command } => command.try_execute(context),
        Commands::Execute { command } => command.try_execute(context),
        Commands::Remove { command } => command.try_execute(context),
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
        // let registry = temp_dir.join(".aleo").join("registry").join("testnet");
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

    #[test]
    #[serial]
    fn relaxed_shadowing_run_test() {
        // Set current directory to temporary directory
        let temp_dir = temp_dir();
        let project_name = "outer";
        let project_directory = temp_dir.join(project_name);

        // Remove it if it already exists
        if project_directory.exists() {
            std::fs::remove_dir_all(project_directory.clone()).unwrap();
        }

        // Create file structure
        test_helpers::sample_shadowing_package(&temp_dir);

        // Run program
        let run = CLI {
            debug: false,
            quiet: false,
            command: Commands::Run {
                command: crate::cli::commands::Run {
                    name: "inner_1_main".to_string(),
                    inputs: vec!["1u32".to_string(), "2u32".to_string()],
                    compiler_options: Default::default(),
                    file: None,
                },
            },
            path: Some(project_directory.clone()),
            home: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(run).expect("Failed to execute `leo run`");
        });
    }

    #[test]
    #[serial]
    fn relaxed_struct_shadowing_run_test() {
        // Set current directory to temporary directory
        let temp_dir = temp_dir();
        let project_name = "outer_2";
        let project_directory = temp_dir.join(project_name);

        // Remove it if it already exists
        if project_directory.exists() {
            std::fs::remove_dir_all(project_directory.clone()).unwrap();
        }

        // Create file structure
        test_helpers::sample_struct_shadowing_package(&temp_dir);

        // Run program
        let run = CLI {
            debug: false,
            quiet: false,
            command: Commands::Run {
                command: crate::cli::commands::Run {
                    name: "main".to_string(),
                    inputs: vec!["1u32".to_string(), "2u32".to_string()],
                    compiler_options: Default::default(),
                    file: None,
                },
            },
            path: Some(project_directory.clone()),
            home: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(run).expect("Failed to execute `leo run`");
        });
    }
}

#[cfg(test)]
mod test_helpers {
    use crate::cli::{cli::Commands, run_with_args, Add, New, CLI};
    use leo_span::symbol::create_session_if_not_set_then;
    use std::path::Path;

    const NETWORK: &str = "testnet";
    const ENDPOINT: &str = "https://api.explorer.aleo.org/v1";

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
            command: Commands::New {
                command: New { name: name.to_string(), network: NETWORK.to_string(), endpoint: ENDPOINT.to_string() },
            },
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
                    network: NETWORK.to_string(),
                    clear: false,
                },
            },
            path: Some(project_directory.clone()),
            home: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(add).expect("Failed to execute `leo add`");
        });

        // Add custom `.aleo` directory
        let registry = temp_dir.join(".aleo").join("registry").join("testnet");
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
            command: Commands::New {
                command: New {
                    name: "grandparent".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(grandparent_directory.clone()),
            home: None,
        };

        let create_parent_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "parent".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(parent_directory.clone()),
            home: None,
        };

        let create_child_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "child".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
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
                    network: NETWORK.to_string(),
                    clear: false,
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
                    network: NETWORK.to_string(),
                    clear: false,
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
                    network: NETWORK.to_string(),
                    clear: false,
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

    pub(crate) fn sample_shadowing_package(temp_dir: &Path) {
        let outer_directory = temp_dir.join("outer");
        let inner_1_directory = outer_directory.join("inner_1");
        let inner_2_directory = outer_directory.join("inner_2");

        if outer_directory.exists() {
            std::fs::remove_dir_all(outer_directory.clone()).unwrap();
        }

        // Create project file structure `outer/inner_1` and `outer/inner_2`
        let create_outer_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "outer".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
        };

        let create_inner_1_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "inner_1".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(inner_1_directory.clone()),
            home: None,
        };

        let create_inner_2_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "inner_2".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(inner_2_directory.clone()),
            home: None,
        };

        // Add source files `outer/src/main.leo` and `outer/inner/src/main.leo`
        let outer_program = "import inner_1.aleo;
import inner_2.aleo;
program outer.aleo {

    struct ex_struct {
        arg1: u32,
        arg2: u32,
    }

    record inner_1_record {
        owner: address,
        arg1: u32,
        arg2: u32,
        arg3: u32,
    }

    transition inner_1_main(public a: u32, b: u32) -> (inner_1.aleo/inner_1_record, inner_2.aleo/inner_1_record, inner_1_record) {
        let c: ex_struct = ex_struct {arg1: 1u32, arg2: 1u32};
        let rec_1:inner_1.aleo/inner_1_record = inner_1.aleo/inner_1_main(1u32,1u32, c);
        let rec_2:inner_2.aleo/inner_1_record = inner_2.aleo/inner_1_main(1u32,1u32);
        return (rec_1, rec_2, inner_1_record {owner: aleo14tnetva3xfvemqyg5ujzvr0qfcaxdanmgjx2wsuh2xrpvc03uc9s623ps7, arg1: 1u32, arg2: 1u32, arg3: 1u32});
    }
}";
        let inner_1_program = "program inner_1.aleo {
    mapping inner_1_mapping: u32 => u32;
    record inner_1_record {
        owner: address,
        val: u32,
    }
    struct ex_struct {
        arg1: u32,
        arg2: u32,
    }
    transition inner_1_main(public a: u32, b: u32, c: ex_struct) -> inner_1_record {
        return inner_1_record {
            owner: self.caller,
            val: c.arg1,
        };
    }
}";
        let inner_2_program = "program inner_2.aleo {
    mapping inner_2_mapping: u32 => u32;
    record inner_1_record {
        owner: address,
        val: u32,
    }
    transition inner_1_main(public a: u32, b: u32) -> inner_1_record {
        let c: u32 = a + b;
        return inner_1_record {
            owner: self.caller,
            val: a,
        };
    }
}";
        // Add dependencies `outer/program.json`
        let add_outer_dependency_1 = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "inner_1".to_string(),
                    local: Some(inner_1_directory.clone()),
                    network: NETWORK.to_string(),
                    clear: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
        };

        let add_outer_dependency_2 = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "inner_2".to_string(),
                    local: Some(inner_2_directory.clone()),
                    network: NETWORK.to_string(),
                    clear: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
        };

        // Execute all commands
        create_session_if_not_set_then(|_| {
            // Create projects
            run_with_args(create_outer_project).unwrap();
            run_with_args(create_inner_1_project).unwrap();
            run_with_args(create_inner_2_project).unwrap();

            // Write files
            std::fs::write(outer_directory.join("src").join("main.leo"), outer_program).unwrap();
            std::fs::write(inner_1_directory.join("src").join("main.leo"), inner_1_program).unwrap();
            std::fs::write(inner_2_directory.join("src").join("main.leo"), inner_2_program).unwrap();

            // Add dependencies
            run_with_args(add_outer_dependency_1).unwrap();
            run_with_args(add_outer_dependency_2).unwrap();
        });
    }

    pub(crate) fn sample_struct_shadowing_package(temp_dir: &Path) {
        let outer_directory = temp_dir.join("outer_2");
        let inner_1_directory = outer_directory.join("inner_1");
        let inner_2_directory = outer_directory.join("inner_2");

        if outer_directory.exists() {
            std::fs::remove_dir_all(outer_directory.clone()).unwrap();
        }

        // Create project file structure `outer_2/inner_1` and `outer_2/inner_2`
        let create_outer_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "outer_2".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
        };

        let create_inner_1_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "inner_1".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(inner_1_directory.clone()),
            home: None,
        };

        let create_inner_2_project = CLI {
            debug: false,
            quiet: false,
            command: Commands::New {
                command: New {
                    name: "inner_2".to_string(),
                    network: NETWORK.to_string(),
                    endpoint: ENDPOINT.to_string(),
                },
            },
            path: Some(inner_2_directory.clone()),
            home: None,
        };

        // Add source files `outer_2/src/main.leo` and `outer_2/inner/src/main.leo`
        let outer_program = "
import inner_1.aleo;
import inner_2.aleo;
program outer_2.aleo {
    struct Foo {
        a: u32,
        b: u32,
        c: Boo,
    }
    struct Boo {
        a: u32,
        b: u32,
    }
    struct Goo {
        a: u32,
        b: u32,
        c: u32,
    }
    record Hello {
        owner: address,
        a: u32,
    }
    transition main(public a: u32, b: u32) -> (inner_2.aleo/Yoo, Hello) {
        let d: Foo = inner_1.aleo/main(1u32,1u32);
        let e: u32 = inner_1.aleo/main_2(Foo {a: a, b: b, c: Boo {a:1u32, b:1u32}});
        let f: Boo = Boo {a:1u32, b:1u32};
        let g: Foo = inner_2.aleo/main(1u32, 1u32);
        inner_2.aleo/Yo_Consumer(inner_2.aleo/Yo());
        let h: inner_2.aleo/Yoo = inner_2.aleo/Yo();
        let i: Goo = inner_2.aleo/Goo_creator();
        let j: Hello = Hello {owner: self.signer, a:1u32};

        return (h, j);
    }
}
";
        let inner_1_program = "program inner_1.aleo {
    struct Foo {
        a: u32,
        b: u32,
        c: Boo,
    }
    struct Boo {
        a: u32,
        b: u32,
    }
    transition main(public a: u32, b: u32) -> Foo {
        return Foo {a: a, b: b, c: Boo {a:1u32, b:1u32}};
    }
    transition main_2(a:Foo)->u32{
        return a.a;
    }
}";
        let inner_2_program = "program inner_2.aleo {
    struct Foo {
        a: u32,
        b: u32,
        c: Boo,
    }
    struct Boo {
        a: u32,
        b: u32,
    }
    record Yoo {
        owner: address,
        a: u32,
    }
    struct Goo {
        a: u32,
        b: u32,
        c: u32,
    }
    transition main(public a: u32, b: u32) -> Foo {
        return Foo {a: a, b: b, c: Boo {a:1u32, b:1u32}};
    }
    transition Yo()-> Yoo {
        return Yoo {owner: self.signer, a:1u32};
    }
    transition Yo_Consumer(a: Yoo)->u32 {
        return a.a;
    }
    transition Goo_creator() -> Goo {
        return Goo {a:100u32, b:1u32, c:1u32};
    }
}";
        // Add dependencies `outer_2/program.json`
        let add_outer_dependency_1 = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "inner_1".to_string(),
                    local: Some(inner_1_directory.clone()),
                    network: NETWORK.to_string(),
                    clear: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
        };

        let add_outer_dependency_2 = CLI {
            debug: false,
            quiet: false,
            command: Commands::Add {
                command: Add {
                    name: "inner_2".to_string(),
                    local: Some(inner_2_directory.clone()),
                    network: NETWORK.to_string(),
                    clear: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
        };

        // Execute all commands
        create_session_if_not_set_then(|_| {
            // Create projects
            run_with_args(create_outer_project).unwrap();
            run_with_args(create_inner_1_project).unwrap();
            run_with_args(create_inner_2_project).unwrap();

            // Write files
            std::fs::write(outer_directory.join("src").join("main.leo"), outer_program).unwrap();
            std::fs::write(inner_1_directory.join("src").join("main.leo"), inner_1_program).unwrap();
            std::fs::write(inner_2_directory.join("src").join("main.leo"), inner_2_program).unwrap();

            // Add dependencies
            run_with_args(add_outer_dependency_1).unwrap();
            run_with_args(add_outer_dependency_2).unwrap();
        });
    }
}
