// Copyright (C) 2019-2026 Provable Inc.
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
use serde::Serialize;
use std::{ffi::OsString, path::PathBuf, process::exit};

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(Parser, Debug)]
#[clap(name = "leo", author = "The Leo Team <leo@provable.com>", version, long_version = env!("LEO_VERSION_STRING"))]
pub struct CLI {
    #[clap(short, global = true, help = "Print additional information for debugging")]
    debug: bool,

    #[clap(short, global = true, help = "Suppress CLI output")]
    quiet: bool,

    #[clap(long, global = true, help = "Write results as JSON to a file. Defaults to build/json-outputs/<command>.json if no path specified.", num_args = 0..=1, require_equals = true, default_missing_value = "")]
    json_output: Option<String>,

    #[clap(long, global = true, help = "Disable Leo's daily check for version updates")]
    disable_update_check: bool,

    #[clap(subcommand)]
    command: Commands,

    #[clap(long, global = true, help = "Path to Leo program root folder")]
    path: Option<PathBuf>,

    #[clap(long, global = true, help = "Path to aleo program registry")]
    pub home: Option<PathBuf>,

    #[clap(short = 'p', long = "package", global = true, help = "Target a specific workspace member by name")]
    pub package: Option<String>,
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
        command: LeoNew,
    },
    #[clap(about = "Run a program with input variables", visible_alias = "r")]
    Run {
        #[clap(flatten)]
        command: LeoRun,
    },
    #[clap(about = "Test a Leo program", visible_alias = "t")]
    Test {
        #[clap(flatten)]
        command: LeoTest,
    },
    #[clap(about = "Execute a program with input variables")]
    Execute {
        #[clap(flatten)]
        command: LeoExecute,
    },
    #[clap(about = "Deploy a program")]
    Deploy {
        #[clap(flatten)]
        command: LeoDeploy,
    },
    #[clap(about = "Run a local devnet")]
    Devnet {
        #[clap(flatten)]
        command: LeoDevnet,
    },
    #[clap(about = "Run a local devnode")]
    Devnode {
        #[clap(flatten)]
        command: LeoDevnode,
    },
    #[clap(about = "Query live data from the Aleo network")]
    Query {
        #[clap(flatten)]
        command: LeoQuery,
    },
    #[clap(about = "Compile the current package as a program", visible_alias = "b")]
    Build {
        #[clap(flatten)]
        command: LeoBuild,
    },
    #[clap(about = "Generate ABI from an Aleo bytecode file")]
    Abi {
        #[clap(flatten)]
        command: LeoAbi,
    },
    #[clap(about = "Add a new on-chain or local dependency to the current package.")]
    Add {
        #[clap(flatten)]
        command: LeoAdd,
    },
    #[clap(about = "Remove a dependency from the current package.")]
    Remove {
        #[clap(flatten)]
        command: LeoRemove,
    },
    #[clap(about = "Clean the output directory")]
    Clean {
        #[clap(flatten)]
        command: LeoClean,
    },
    #[clap(about = "Synthesize individual keys")]
    Synthesize {
        #[clap(flatten)]
        command: LeoSynthesize,
    },
    #[clap(about = "List installed leo plugins")]
    Plugins,
    #[clap(about = "Update the Leo CLI")]
    Update {
        #[clap(flatten)]
        command: LeoUpdate,
    },
    #[clap(about = "Upgrade the program on a network")]
    Upgrade {
        #[clap(flatten)]
        command: LeoUpgrade,
    },
    /// Dispatch to an external `leo-<name>` plugin.
    #[clap(external_subcommand)]
    External(Vec<OsString>),
}

impl Commands {
    fn name(&self) -> &'static str {
        match self {
            Commands::Account { .. } => "account",
            Commands::New { .. } => "new",
            Commands::Run { .. } => "run",
            Commands::Test { .. } => "test",
            Commands::Execute { .. } => "execute",
            Commands::Deploy { .. } => "deploy",
            Commands::Devnet { .. } => "devnet",
            Commands::Devnode { .. } => "devnode",
            Commands::Query { .. } => "query",
            Commands::Build { .. } => "build",
            Commands::Abi { .. } => "abi",
            Commands::Add { .. } => "add",
            Commands::Remove { .. } => "remove",
            Commands::Clean { .. } => "clean",
            Commands::Synthesize { .. } => "synthesize",
            Commands::Plugins => "plugins",
            Commands::Update { .. } => "update",
            Commands::Upgrade { .. } => "upgrade",
            Commands::External(_) => "external",
        }
    }
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

/// JSON output types for commands that support `--json`.
#[derive(Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
enum Output {
    Deploy(DeployOutput),
    Run(RunOutput),
    Execute(ExecuteOutput),
    Test(TestOutput),
    Query(serde_json::Value),
    Synthesize(SynthesizeOutput),
}

/// Run command with custom build arguments.
pub fn run_with_args(cli: CLI) -> Result<()> {
    // JSON output mode implies quiet mode.
    let quiet = cli.quiet || cli.json_output.is_some();

    // Print the variables found in the `.env` files.
    if !quiet && let Ok(vars) = dotenvy::dotenv_iter().map(|v| v.flatten().collect::<Vec<_>>()) {
        if !vars.is_empty() {
            println!("📢 Loading environment variables from a `.env` file in the directory tree.");
        }
        for (k, v) in vars {
            println!("  - {k}={v}");
        }
    }
    // Initialize the `.env` file.
    dotenvy::dotenv().ok();

    // Skip logger initialization for devnode -- it uses it's own logger.
    let is_devnode = matches!(&cli.command, Commands::Devnode { .. });

    if !quiet && !is_devnode {
        // Init logger with optional debug flag.
        logger::init_logger("leo", match cli.debug {
            false => 1,
            true => 2,
        })?;
    }

    // Check for updates. If not forced, it checks once per day.
    if !quiet
        && !cli.disable_update_check
        && let Ok(true) = updater::Updater::check_for_updates(false)
    {
        let _ = updater::Updater::print_cli();
    }

    // Get custom root folder and create context for it.
    // If not specified, default context will be created in cwd.
    let context = handle_error(Context::new(cli.path.clone(), cli.home, false, cli.package.clone()));

    let command_name = cli.command.name();
    let mut command_output: Option<Output> = None;

    match cli.command {
        Commands::Add { command } => command.try_execute(context)?,
        Commands::Account { command } => command.try_execute(context)?,
        Commands::New { command } => command.try_execute(context)?,
        Commands::Build { command } => command.try_execute(context)?,
        Commands::Abi { command } => command.try_execute(context)?,
        Commands::Query { command } => {
            let result = command.execute(context)?;
            let data = serde_json::from_str(&result).unwrap_or_else(|_| serde_json::Value::String(result));
            command_output = Some(Output::Query(data));
        }
        Commands::Clean { command } => command.try_execute(context)?,
        Commands::Deploy { command } => command_output = Some(Output::Deploy(command.execute(context)?)),
        Commands::Devnet { command } => command.try_execute(context)?,
        Commands::Devnode { command } => command.try_execute(context)?,
        Commands::Run { command } => command_output = Some(Output::Run(command.execute(context)?)),
        Commands::Test { command } => command_output = Some(Output::Test(command.execute(context)?)),
        Commands::Execute { command } => command_output = Some(Output::Execute(command.execute(context)?)),
        Commands::Plugins => crate::cli::plugin::print_all(),
        Commands::External(args) => {
            let (name, plugin_args) = args.split_first().expect("external subcommand requires a name");
            let name = format!("leo-{}", name.to_string_lossy());
            crate::cli::plugin::exec(&name, plugin_args, Some(&context.dir()?))?;
        }
        Commands::Remove { command } => command.try_execute(context)?,
        Commands::Synthesize { command } => command_output = Some(Output::Synthesize(command.execute(context)?)),
        Commands::Update { command } => command.try_execute(context)?,
        Commands::Upgrade { command } => command_output = Some(Output::Deploy(command.execute(context)?)),
    }

    if let Some(json_output_arg) = cli.json_output
        && let Some(output) = &command_output
    {
        let json = serde_json::to_string_pretty(output).expect("JSON serialization failed");

        // Use provided path or default to build/json-outputs/<command>.json
        let path = if json_output_arg.is_empty() {
            cli.path
                .unwrap_or_else(|| PathBuf::from("."))
                .join("build")
                .join("json-outputs")
                .join(format!("{command_name}.json"))
        } else {
            PathBuf::from(json_output_arg)
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| crate::errors::custom(format!("Failed to create directory: {e}")))?;
        }
        std::fs::write(&path, json)
            .map_err(|e| crate::errors::custom(format!("Failed to write JSON output to {}: {e}", path.display())))?;
    }

    if let Some(Output::Test(output)) = &command_output
        && output.failed > 0
    {
        return Err(crate::errors::tests_failed(output.failed, output.tests.len()).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cli::{
        CLI,
        cli::{Commands, test_helpers},
        run_with_args,
    };
    use clap::Parser;
    use leo_ast::NetworkName;
    use leo_span::create_session_if_not_set_then;
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

        // Set the env options.
        let env_override = crate::cli::commands::EnvOptions {
            network: Some(NetworkName::TestnetV0),
            endpoint: Some("http://localhost:3030".to_string()),
            ..Default::default()
        };

        // Run program
        let run = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Run {
                command: crate::cli::commands::LeoRun {
                    name: "example".to_string(),
                    inputs: vec!["1u32".to_string(), "2u32".to_string()],
                    env_override,
                    build_options: Default::default(),
                    with: vec![],
                },
            },
            path: Some(project_directory.clone()),
            home: Some(temp_dir.join(".aleo")),
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(run).expect("Failed to execute `leo run`");
        });

        // TODO: Clear tmp directory
        // let registry = temp_dir.join(".aleo").join("registry").join("mainnet");
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
            json_output: None,
            disable_update_check: false,
            command: Commands::Run {
                command: crate::cli::commands::LeoRun {
                    name: "double_wrapper_mint".to_string(),
                    inputs: vec![
                        "aleo13tngrq7506zwdxj0cxjtvp28pk937jejhne0rt4zp0z370uezuysjz2prs".to_string(),
                        "2u32".to_string(),
                    ],
                    env_override: Default::default(),
                    build_options: Default::default(),
                    with: vec![],
                },
            },
            path: Some(project_directory.clone()),
            home: None,
            package: None,
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
            json_output: None,
            disable_update_check: false,
            command: Commands::Run {
                command: crate::cli::commands::LeoRun {
                    name: "inner_1_main".to_string(),
                    inputs: vec!["1u32".to_string(), "2u32".to_string()],
                    build_options: Default::default(),
                    env_override: Default::default(),
                    with: vec![],
                },
            },
            path: Some(project_directory.clone()),
            home: None,
            package: None,
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
            json_output: None,
            disable_update_check: false,
            command: Commands::Run {
                command: crate::cli::commands::LeoRun {
                    name: "main".to_string(),
                    inputs: vec!["1u32".to_string(), "2u32".to_string()],
                    env_override: Default::default(),
                    build_options: Default::default(),
                    with: vec![],
                },
            },
            path: Some(project_directory.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(run).expect("Failed to execute `leo run`");
        });
    }

    #[test]
    #[serial]
    fn new_library_test() {
        let temp_dir = temp_dir();
        let lib_name = "my_test_lib";
        let lib_directory = temp_dir.join(lib_name);

        // Clean up from any previous run.
        if lib_directory.exists() {
            std::fs::remove_dir_all(&lib_directory).unwrap();
        }

        // Run `leo new --library my_test_lib`.
        let new_cmd = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: crate::cli::commands::LeoNew { name: lib_name.to_string(), library: true, workspace: false },
            },
            path: Some(lib_directory.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(new_cmd).expect("Failed to execute `leo new --library`");
        });

        // Verify the directory was created.
        assert!(lib_directory.exists(), "Library directory should exist");

        // Verify src/lib.leo exists (not main.leo).
        let src_dir = lib_directory.join("src");
        assert!(src_dir.join("lib.leo").exists(), "src/lib.leo should exist");
        assert!(!src_dir.join("main.leo").exists(), "src/main.leo should NOT exist for a library");

        // Verify the manifest has the library name (no .aleo suffix).
        let manifest_path = lib_directory.join(leo_package::MANIFEST_FILENAME);
        let manifest = leo_package::Manifest::read_from_file(&manifest_path).unwrap();
        assert_eq!(manifest.program, lib_name, "Manifest program name should be the bare library name");

        // Best-effort cleanup. On Windows the directory may still be held open briefly
        // by the OS, so we ignore errors here rather than failing an otherwise-passing test.
        let _ = std::fs::remove_dir_all(&lib_directory);
    }

    #[test]
    #[serial]
    fn new_workspace_test() {
        let temp_dir = temp_dir();
        let ws_name = "my_test_workspace";
        let ws_directory = temp_dir.join(ws_name);

        if ws_directory.exists() {
            std::fs::remove_dir_all(&ws_directory).unwrap();
        }

        let new_cmd = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: crate::cli::commands::LeoNew { name: ws_name.to_string(), library: false, workspace: true },
            },
            path: Some(ws_directory.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(new_cmd).expect("Failed to execute `leo new --workspace`");
        });

        assert!(ws_directory.is_dir(), "Workspace directory should exist");
        let manifest_path = ws_directory.join(leo_package::WORKSPACE_MANIFEST_FILENAME);
        assert!(manifest_path.exists(), "workspace.json should exist");
        let manifest = leo_package::WorkspaceManifest::read_from_file(&manifest_path).unwrap();
        assert!(manifest.members.is_empty(), "newly created workspace should have an empty members list");
        assert!(!ws_directory.join("src").exists(), "workspace skeleton should not have a src directory");
        assert!(
            !ws_directory.join(leo_package::MANIFEST_FILENAME).exists(),
            "workspace skeleton should not have a program.json"
        );

        let _ = std::fs::remove_dir_all(&ws_directory);
    }

    #[test]
    #[serial]
    fn new_workspace_with_library_flag_rejected() {
        // The clap `conflicts_with` constraint between `--workspace` and `--library` should
        // surface as a parse error rather than running either path.
        let result = CLI::try_parse_from(["leo", "new", "--workspace", "--library", "anything"]);
        assert!(result.is_err(), "leo new --workspace --library should fail parsing");
    }

    #[test]
    #[serial]
    fn deploy_rename_flag_parses() {
        // `leo deploy --rename <name>` should populate the deploy command's `rename` field.
        let cli = CLI::try_parse_from(["leo", "deploy", "--rename", "renamed"])
            .expect("`leo deploy --rename renamed` should parse");
        match cli.command {
            Commands::Deploy { command } => assert_eq!(command.rename.as_deref(), Some("renamed")),
            _ => panic!("expected a deploy command"),
        }
        // The flag is optional: deploying without it leaves `rename` unset.
        let cli = CLI::try_parse_from(["leo", "deploy"]).expect("`leo deploy` should parse");
        match cli.command {
            Commands::Deploy { command } => assert_eq!(command.rename, None),
            _ => panic!("expected a deploy command"),
        }
    }

    #[test]
    #[serial]
    fn new_inside_workspace_auto_registers() {
        let temp_dir = temp_dir();
        let ws_root = temp_dir.join("ws_new_inside_test");
        if ws_root.exists() {
            std::fs::remove_dir_all(&ws_root).unwrap();
        }
        std::fs::create_dir_all(&ws_root).unwrap();

        // Seed the workspace with a single literal member that actually exists.
        test_helpers::scaffold_minimal_member(&ws_root, "existing");
        let manifest = leo_package::WorkspaceManifest { members: vec!["existing".to_string()] };
        manifest.write_to_file(ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME)).unwrap();

        let new_pkg = "my_pkg";
        let pkg_dir = ws_root.join(new_pkg);
        let new_cmd = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: crate::cli::commands::LeoNew { name: new_pkg.to_string(), library: false, workspace: false },
            },
            path: Some(pkg_dir.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(new_cmd).expect("Failed to execute `leo new` inside a workspace");
        });

        assert!(pkg_dir.join(leo_package::MANIFEST_FILENAME).exists(), "package should be scaffolded");
        let manifest =
            leo_package::WorkspaceManifest::read_from_file(ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME))
                .unwrap();
        assert_eq!(manifest.members, vec!["existing".to_string(), new_pkg.to_string()]);

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn new_inside_workspace_with_glob_does_not_modify_manifest() {
        let temp_dir = temp_dir();
        let ws_root = temp_dir.join("ws_new_inside_glob_test");
        if ws_root.exists() {
            std::fs::remove_dir_all(&ws_root).unwrap();
        }
        std::fs::create_dir_all(&ws_root).unwrap();

        // A `*` pattern in `members` covers anything created at the workspace root.
        let manifest = leo_package::WorkspaceManifest { members: vec!["*".to_string()] };
        manifest.write_to_file(ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME)).unwrap();

        let new_pkg = "my_pkg";
        let pkg_dir = ws_root.join(new_pkg);
        let new_cmd = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: crate::cli::commands::LeoNew { name: new_pkg.to_string(), library: false, workspace: false },
            },
            path: Some(pkg_dir.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(new_cmd).expect("Failed to execute `leo new` inside a globbed workspace");
        });

        assert!(pkg_dir.join(leo_package::MANIFEST_FILENAME).exists(), "package should be scaffolded");
        let manifest =
            leo_package::WorkspaceManifest::read_from_file(ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME))
                .unwrap();
        assert_eq!(manifest.members, vec!["*".to_string()], "glob-covered workspace manifest should be untouched");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_build_from_root_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace(&temp_dir, "build_root");

        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("workspace build should succeed");
        });

        assert!(ws_root.join("build/token/token.aleo").exists(), "token should be built");
        assert!(ws_root.join("build/swap/swap.aleo").exists(), "swap should be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_build_single_member_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace(&temp_dir, "build_single");

        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(ws_root.join("token")),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("single member build should succeed");
        });

        assert!(ws_root.join("build/token/token.aleo").exists(), "token should be built");
        assert!(!ws_root.join("build/swap/swap.aleo").exists(), "swap should NOT be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_package_flag_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace(&temp_dir, "pkg_flag");

        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: Some("token".to_string()),
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("--package build should succeed");
        });

        assert!(ws_root.join("build/token/token.aleo").exists(), "token should be built");
        assert!(!ws_root.join("build/swap/swap.aleo").exists(), "swap should NOT be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_package_flag_not_found_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace(&temp_dir, "pkg_not_found");

        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: Default::default(),
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: Some("nonexistent".to_string()),
        };

        create_session_if_not_set_then(|_| {
            let result = run_with_args(build);
            assert!(result.is_err(), "--package with unknown member should error");
        });

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_clean_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace(&temp_dir, "clean");

        // Build first.
        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("build should succeed");
        });

        assert!(ws_root.join("build/token").exists(), "token build dir should exist under shared build/");
        assert!(ws_root.join("build/swap").exists(), "swap build dir should exist under shared build/");

        // Clean.
        let clean = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Clean { command: crate::cli::commands::LeoClean {} },
            path: Some(ws_root.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(clean).expect("workspace clean should succeed");
        });

        assert!(!ws_root.join("build").exists(), "shared workspace build dir should be cleaned");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_build_workspace_dep_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace_with_workspace_deps(&temp_dir, "build_ws_dep");

        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("workspace build with workspace deps should succeed");
        });

        assert!(ws_root.join("build/token/token.aleo").exists(), "token should be built");
        assert!(ws_root.join("build/swap/swap.aleo").exists(), "swap should be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_build_single_member_workspace_dep_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace_with_workspace_deps(&temp_dir, "build_single_ws_dep");

        // Build from the swap member directory (which depends on token via workspace).
        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(ws_root.join("swap")),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("single member build with workspace dep should succeed");
        });

        assert!(ws_root.join("build/swap/swap.aleo").exists(), "swap should be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_build_workspace_dev_dep_test() {
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace_with_workspace_dev_deps(&temp_dir, "build_ws_dev_dep");

        // Build from the swap member directory (which depends on token via dev_dependencies workspace).
        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(ws_root.join("swap")),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("build with workspace dev dep should succeed");
        });

        assert!(ws_root.join("build/swap/swap.aleo").exists(), "swap should be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_deploy_builds_all_members_test() {
        // Workspace deploy should build all members before attempting deployment.
        // Without a network endpoint, deploy will fail after the build phase, but
        // we can verify that build artifacts were created for all members.
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace_with_workspace_deps(&temp_dir, "deploy_builds");

        let deploy = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Deploy {
                command: crate::cli::commands::LeoDeploy {
                    fee_options: Default::default(),
                    action: crate::cli::commands::TransactionAction { print: false, broadcast: false, save: None },
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        private_key: Some("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH".to_string()),
                        endpoint: Some("http://localhost:1".to_string()),
                        consensus_heights: Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
                        ..Default::default()
                    },
                    extra: crate::cli::commands::ExtraOptions { yes: true, ..Default::default() },
                    skip: vec![],
                    rename: None,
                    build_options: Default::default(),
                    skip_deploy_certificate: true,
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            // Deploy will fail because there is no reachable endpoint, but it
            // should have already built both workspace members before that.
            let _ = run_with_args(deploy);
        });

        // Verify both members were built.
        assert!(ws_root.join("build/token/token.aleo").exists(), "token should be built");
        assert!(ws_root.join("build/swap/swap.aleo").exists(), "swap should be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_deploy_package_flag_test() {
        // With --package, deploy should only build and deploy that member.
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace_with_workspace_deps(&temp_dir, "deploy_pkg_flag");

        let deploy = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Deploy {
                command: crate::cli::commands::LeoDeploy {
                    fee_options: Default::default(),
                    action: crate::cli::commands::TransactionAction { print: false, broadcast: false, save: None },
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        private_key: Some("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH".to_string()),
                        endpoint: Some("http://localhost:1".to_string()),
                        consensus_heights: Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
                        ..Default::default()
                    },
                    extra: crate::cli::commands::ExtraOptions { yes: true, ..Default::default() },
                    skip: vec![],
                    rename: None,
                    build_options: Default::default(),
                    skip_deploy_certificate: true,
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            // Filter to just token.
            package: Some("token".to_string()),
        };

        create_session_if_not_set_then(|_| {
            let _ = run_with_args(deploy);
        });

        // --package=token targets a single member, so resolve_targets returns 1 target.
        // This falls through to the single-package deploy path, building only token.
        assert!(ws_root.join("build/token/token.aleo").exists(), "token should be built");
        assert!(!ws_root.join("build/swap/swap.aleo").exists(), "swap should NOT be built");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_deploy_all_libraries_error_test() {
        // Deploying a workspace where every member is a library should error.
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace_all_libraries(&temp_dir, "deploy_all_libs");

        let deploy = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Deploy {
                command: crate::cli::commands::LeoDeploy {
                    fee_options: Default::default(),
                    action: crate::cli::commands::TransactionAction { print: false, broadcast: false, save: None },
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        private_key: Some("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH".to_string()),
                        endpoint: Some("http://localhost:1".to_string()),
                        consensus_heights: Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
                        ..Default::default()
                    },
                    extra: crate::cli::commands::ExtraOptions { yes: true, ..Default::default() },
                    skip: vec![],
                    rename: None,
                    build_options: Default::default(),
                    skip_deploy_certificate: true,
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            let result = run_with_args(deploy);
            assert!(result.is_err(), "deploy of all-library workspace should fail");
            let err = result.unwrap_err().to_string();
            assert!(
                err.contains("No deployable workspace members found"),
                "expected 'No deployable workspace members found', got: {err}"
            );
        });

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_deploy_mixed_library_program_test() {
        // Workspace with one library and one program member. Only the program
        // member should be deployed (library is skipped).
        let temp_dir = temp_dir();
        let ws_root = test_helpers::sample_workspace_mixed(&temp_dir, "deploy_mixed");

        let deploy = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Deploy {
                command: crate::cli::commands::LeoDeploy {
                    fee_options: Default::default(),
                    action: crate::cli::commands::TransactionAction { print: false, broadcast: false, save: None },
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        private_key: Some("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH".to_string()),
                        endpoint: Some("http://localhost:1".to_string()),
                        consensus_heights: Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]),
                        ..Default::default()
                    },
                    extra: crate::cli::commands::ExtraOptions { yes: true, ..Default::default() },
                    skip: vec![],
                    rename: None,
                    build_options: Default::default(),
                    skip_deploy_certificate: true,
                },
            },
            path: Some(ws_root.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            // Deploy will fail at network, but build phase should succeed.
            let _ = run_with_args(deploy);
        });

        // The app program should be built.
        assert!(ws_root.join("build/app/app.aleo").exists(), "app should be built");
        // utils is a library - no bytecode output.
        assert!(!ws_root.join("build/utils/utils.aleo").exists(), "utils library should not produce bytecode");

        let _ = std::fs::remove_dir_all(&ws_root);
    }

    #[test]
    #[serial]
    fn workspace_backward_compat_test() {
        let temp_dir = temp_dir();
        let pkg_name = "standalone_pkg";
        let pkg_dir = temp_dir.join(pkg_name);

        if pkg_dir.exists() {
            std::fs::remove_dir_all(&pkg_dir).unwrap();
        }

        let new_cmd = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: crate::cli::commands::LeoNew { name: pkg_name.to_string(), library: false, workspace: false },
            },
            path: Some(pkg_dir.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(new_cmd).expect("leo new should succeed");
        });

        let build = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Build {
                command: crate::cli::commands::LeoBuild {
                    options: Default::default(),
                    rename: None,
                    env_override: crate::cli::commands::EnvOptions {
                        network: Some(NetworkName::TestnetV0),
                        ..Default::default()
                    },
                },
            },
            path: Some(pkg_dir.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(build).expect("standalone build should succeed");
        });

        assert!(pkg_dir.join("build/standalone_pkg/standalone_pkg.aleo").exists(), "build artifact should exist");

        let _ = std::fs::remove_dir_all(&pkg_dir);
    }
}

#[cfg(test)]
mod test_helpers {
    use crate::cli::{CLI, DependencySource, GitRef, LeoAdd, LeoNew, cli::Commands, run_with_args};
    use leo_span::create_session_if_not_set_then;
    use std::path::{Path, PathBuf};

    /// Scaffold a minimal program package named `name` at `<parent>/<name>`.
    ///
    /// Writes a no-deps `program.json` and a trivial `src/main.leo`. Useful as
    /// a lightweight workspace member when a test only needs the directory to
    /// pass validation.
    pub(crate) fn scaffold_minimal_member(parent: &Path, name: &str) {
        let member_dir = parent.join(name);
        std::fs::create_dir_all(member_dir.join("src")).unwrap();
        std::fs::write(
            member_dir.join("src/main.leo"),
            format!("program {name}.aleo {{\n    @noupgrade\n    constructor() {{}}\n}}\n"),
        )
        .unwrap();
        std::fs::write(
            member_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": format!("{name}.aleo"),
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();
    }

    /// Create a workspace with two members: `token` (no deps) and `swap` (depends on token).
    ///
    /// Returns the workspace root directory. Each caller should pass a unique
    /// `name` to avoid temp-directory collisions under parallel test runners.
    pub(crate) fn sample_workspace(temp_dir: &Path, name: &str) -> PathBuf {
        let ws_root = temp_dir.join(format!("ws_{name}"));

        if ws_root.exists() {
            std::fs::remove_dir_all(&ws_root).unwrap();
        }
        std::fs::create_dir_all(&ws_root).unwrap();

        let token_dir = ws_root.join("token");
        let swap_dir = ws_root.join("swap");

        // Create token package.
        std::fs::create_dir_all(token_dir.join("src")).unwrap();
        std::fs::write(
            token_dir.join("src/main.leo"),
            "\
program token.aleo {
    fn mint(owner: address, amount: u32) -> u32 {
        return amount;
    }

    @noupgrade
    constructor() {}
}
",
        )
        .unwrap();
        std::fs::write(
            token_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "token.aleo",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Create swap package (depends on token via local path).
        std::fs::create_dir_all(swap_dir.join("src")).unwrap();
        std::fs::write(
            swap_dir.join("src/main.leo"),
            "\
import token.aleo;
program swap.aleo {
    fn do_swap(owner: address, amount: u32) -> u32 {
        return token.aleo::mint(owner, amount);
    }

    @noupgrade
    constructor() {}
}
",
        )
        .unwrap();
        std::fs::write(
            swap_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "swap.aleo",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": [{
                    "name": "token.aleo",
                    "location": "local",
                    "path": "../token",
                    "edition": null
                }],
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Write workspace.json.
        std::fs::write(
            ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "members": ["token", "swap"]
            }))
            .unwrap(),
        )
        .unwrap();

        ws_root
    }

    /// Like `sample_workspace`, but `swap` depends on `token` via `"location": "workspace"`
    /// instead of `"location": "local"` with an explicit path.
    pub(crate) fn sample_workspace_with_workspace_deps(temp_dir: &Path, name: &str) -> PathBuf {
        let ws_root = temp_dir.join(format!("ws_{name}"));

        if ws_root.exists() {
            std::fs::remove_dir_all(&ws_root).unwrap();
        }
        std::fs::create_dir_all(&ws_root).unwrap();

        let token_dir = ws_root.join("token");
        let swap_dir = ws_root.join("swap");

        // Create token package (no deps).
        std::fs::create_dir_all(token_dir.join("src")).unwrap();
        std::fs::write(
            token_dir.join("src/main.leo"),
            "\
program token.aleo {
    fn mint(owner: address, amount: u32) -> u32 {
        return amount;
    }

    @noupgrade
    constructor() {}
}
",
        )
        .unwrap();
        std::fs::write(
            token_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "token.aleo",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Create swap package (depends on token via workspace location).
        std::fs::create_dir_all(swap_dir.join("src")).unwrap();
        std::fs::write(
            swap_dir.join("src/main.leo"),
            "\
import token.aleo;
program swap.aleo {
    fn do_swap(owner: address, amount: u32) -> u32 {
        return token.aleo::mint(owner, amount);
    }

    @noupgrade
    constructor() {}
}
",
        )
        .unwrap();
        std::fs::write(
            swap_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "swap.aleo",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": [{
                    "name": "token.aleo",
                    "location": "workspace"
                }],
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Write workspace.json.
        std::fs::write(
            ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "members": ["token", "swap"]
            }))
            .unwrap(),
        )
        .unwrap();

        ws_root
    }

    /// Like `sample_workspace_with_workspace_deps`, but `swap` depends on `token` via
    /// `dev_dependencies` with `Location::Workspace` instead of `dependencies`.
    pub(crate) fn sample_workspace_with_workspace_dev_deps(temp_dir: &Path, name: &str) -> PathBuf {
        let ws_root = temp_dir.join(format!("ws_{name}"));

        if ws_root.exists() {
            std::fs::remove_dir_all(&ws_root).unwrap();
        }
        std::fs::create_dir_all(&ws_root).unwrap();

        let token_dir = ws_root.join("token");
        let swap_dir = ws_root.join("swap");

        // Create token package (no deps).
        std::fs::create_dir_all(token_dir.join("src")).unwrap();
        std::fs::write(
            token_dir.join("src/main.leo"),
            "\
program token.aleo {
    fn mint(owner: address, amount: u32) -> u32 {
        return amount;
    }

    @noupgrade
    constructor() {}
}
",
        )
        .unwrap();
        std::fs::write(
            token_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "token.aleo",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Create swap package (token is a workspace dev_dependency - no import in main source).
        std::fs::create_dir_all(swap_dir.join("src")).unwrap();
        std::fs::write(
            swap_dir.join("src/main.leo"),
            "\
program swap.aleo {
    fn do_swap(amount: u32) -> u32 {
        return amount;
    }

    @noupgrade
    constructor() {}
}
",
        )
        .unwrap();
        std::fs::write(
            swap_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "swap.aleo",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": [{
                    "name": "token.aleo",
                    "location": "workspace"
                }]
            }))
            .unwrap(),
        )
        .unwrap();

        // Write workspace.json.
        std::fs::write(
            ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "members": ["token", "swap"]
            }))
            .unwrap(),
        )
        .unwrap();

        ws_root
    }

    /// Workspace where every member is a library (no `.aleo` programs).
    pub(crate) fn sample_workspace_all_libraries(temp_dir: &Path, name: &str) -> PathBuf {
        let ws_root = temp_dir.join(format!("ws_{name}"));

        if ws_root.exists() {
            std::fs::remove_dir_all(&ws_root).unwrap();
        }
        std::fs::create_dir_all(&ws_root).unwrap();

        let lib_a = ws_root.join("lib_a");
        let lib_b = ws_root.join("lib_b");

        // Create lib_a.
        std::fs::create_dir_all(lib_a.join("src")).unwrap();
        std::fs::write(lib_a.join("src/lib.leo"), "").unwrap();
        std::fs::write(
            lib_a.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "lib_a",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Create lib_b.
        std::fs::create_dir_all(lib_b.join("src")).unwrap();
        std::fs::write(lib_b.join("src/lib.leo"), "").unwrap();
        std::fs::write(
            lib_b.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "lib_b",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Write workspace.json.
        std::fs::write(
            ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "members": ["lib_a", "lib_b"]
            }))
            .unwrap(),
        )
        .unwrap();

        ws_root
    }

    /// Workspace with one library member (`utils`) and one program member
    /// (`app`) that imports it.
    pub(crate) fn sample_workspace_mixed(temp_dir: &Path, name: &str) -> PathBuf {
        let ws_root = temp_dir.join(format!("ws_{name}"));

        if ws_root.exists() {
            std::fs::remove_dir_all(&ws_root).unwrap();
        }
        std::fs::create_dir_all(&ws_root).unwrap();

        let utils_dir = ws_root.join("utils");
        let app_dir = ws_root.join("app");

        // Create utils library.
        std::fs::create_dir_all(utils_dir.join("src")).unwrap();
        std::fs::write(
            utils_dir.join("src/lib.leo"),
            "\
const FACTOR: u32 = 2u32;
",
        )
        .unwrap();
        std::fs::write(
            utils_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "utils",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": null,
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Create app program (depends on utils via workspace).
        std::fs::create_dir_all(app_dir.join("src")).unwrap();
        std::fs::write(
            app_dir.join("src/main.leo"),
            "\
program app.aleo {
    fn run(x: u32) -> u32 {
        return x * utils::FACTOR;
    }

    @noupgrade
    constructor() {}
}
",
        )
        .unwrap();
        std::fs::write(
            app_dir.join(leo_package::MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "program": "app.aleo",
                "version": "0.1.0",
                "description": "",
                "license": "MIT",
                "leo": env!("CARGO_PKG_VERSION"),
                "dependencies": [{
                    "name": "utils",
                    "location": "workspace"
                }],
                "dev_dependencies": null
            }))
            .unwrap(),
        )
        .unwrap();

        // Write workspace.json (utils first so it builds before app).
        std::fs::write(
            ws_root.join(leo_package::WORKSPACE_MANIFEST_FILENAME),
            serde_json::to_string_pretty(&serde_json::json!({
                "members": ["utils", "app"]
            }))
            .unwrap(),
        )
        .unwrap();

        ws_root
    }

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
            json_output: None,
            disable_update_check: false,
            command: Commands::New { command: LeoNew { name: name.to_string(), library: false, workspace: false } },
            path: Some(project_directory.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(new).expect("Failed to execute `leo run`");
        });

        // `nested.aleo` program
        let program_str = "
import nested_example_layer_0.aleo;
program nested.aleo {
    fn example(public a: u32, b: u32) -> u32 {
        let c: u32 = nested_example_layer_0.aleo::main(a, b);
        return c;
    }

    @noupgrade
    constructor() {}
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
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "nested_example_layer_0".to_string(),
                    source: DependencySource {
                        local: None,
                        network: true,
                        edition: Some(0),
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(project_directory.clone()),
            home: None,
            package: None,
        };

        create_session_if_not_set_then(|_| {
            run_with_args(add).expect("Failed to execute `leo add`");
        });

        // Add custom `.aleo` directory with the appropriate cache entries.
        let registry = temp_dir.join(".aleo").join("registry").join("testnet");
        std::fs::create_dir_all(&registry).unwrap();

        let dir = registry.join("nested_example_layer_0").join("0");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("nested_example_layer_0.aleo"), nested_example_layer_0).unwrap();

        let dir = registry.join("nested_example_layer_1").join("0");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("nested_example_layer_1.aleo"), nested_example_layer_1).unwrap();

        let dir = registry.join("nested_example_layer_2").join("0");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("nested_example_layer_2.aleo"), nested_example_layer_2).unwrap();
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
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: LeoNew { name: "grandparent".to_string(), library: false, workspace: false },
            },
            path: Some(grandparent_directory.clone()),
            home: None,
            package: None,
        };

        let create_parent_project = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New { command: LeoNew { name: "parent".to_string(), library: false, workspace: false } },
            path: Some(parent_directory.clone()),
            home: None,
            package: None,
        };

        let create_child_project = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New { command: LeoNew { name: "child".to_string(), library: false, workspace: false } },
            path: Some(child_directory.clone()),
            home: None,
            package: None,
        };

        // Add source files `grandparent/src/main.leo`, `grandparent/parent/src/main.leo`, and `grandparent/parent/child/src/main.leo`
        let grandparent_program = "
import child.aleo;
import parent.aleo;
program grandparent.aleo {
    fn double_wrapper_mint(owner: address, val: u32) -> child.aleo::A {
        return parent.aleo::wrapper_mint(owner, val);
    }

    @noupgrade
    constructor() {}
}
";
        let parent_program = "
import child.aleo;
program parent.aleo {
    fn wrapper_mint(owner: address, val: u32) ->  child.aleo::A {
        return child.aleo::mint(owner, val);
    }

    @noupgrade
    constructor() {}
}
";

        let child_program = "
// The 'a' program.
program child.aleo {
    record A {
        owner: address,
        val: u32,
    }
    fn mint(owner: address, val: u32) -> A {
        return A {owner: owner, val: val};
    }

    @noupgrade
    constructor() {}
}
";

        // Add dependencies `grandparent/program.json` and `grandparent/parent/program.json`
        let add_grandparent_dependency_1 = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "parent".to_string(),
                    source: DependencySource {
                        local: Some(parent_directory.clone()),
                        network: false,
                        edition: None,
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(grandparent_directory.clone()),
            home: None,
            package: None,
        };

        let add_grandparent_dependency_2 = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "child".to_string(),
                    source: DependencySource {
                        local: Some(child_directory.clone()),
                        network: false,
                        edition: None,
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(grandparent_directory.clone()),
            home: None,
            package: None,
        };

        let add_parent_dependency = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "child".to_string(),
                    source: DependencySource {
                        local: Some(child_directory.clone()),
                        network: false,
                        edition: None,
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(parent_directory.clone()),
            home: None,
            package: None,
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
            json_output: None,
            disable_update_check: false,
            command: Commands::New { command: LeoNew { name: "outer".to_string(), library: false, workspace: false } },
            path: Some(outer_directory.clone()),
            home: None,
            package: None,
        };

        let create_inner_1_project = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: LeoNew { name: "inner_1".to_string(), library: false, workspace: false },
            },
            path: Some(inner_1_directory.clone()),
            home: None,
            package: None,
        };

        let create_inner_2_project = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: LeoNew { name: "inner_2".to_string(), library: false, workspace: false },
            },
            path: Some(inner_2_directory.clone()),
            home: None,
            package: None,
        };

        // Add source files `outer/src/main.leo` and `outer/inner/src/main.leo`
        let outer_program = "
import inner_1.aleo;
import inner_2.aleo;
program outer.aleo {
    record inner_1_record {
        owner: address,
        arg1: u32,
        arg2: u32,
        arg3: u32,
    }

    fn inner_1_main(public a: u32, b: u32) -> (inner_1.aleo::inner_1_record, inner_2.aleo::inner_2_record, inner_1_record) {
        let c: inner_1.aleo::ex_struct = inner_1.aleo::ex_struct {arg1: 1u32, arg2: 1u32};
        let rec_1:inner_1.aleo::inner_1_record = inner_1.aleo::inner_1_main(1u32,1u32, c);
        let rec_2:inner_2.aleo::inner_2_record = inner_2.aleo::inner_2_main(1u32,1u32);
        return (rec_1, rec_2, inner_1_record {owner: aleo14tnetva3xfvemqyg5ujzvr0qfcaxdanmgjx2wsuh2xrpvc03uc9s623ps7, arg1: 1u32, arg2: 1u32, arg3: 1u32});
    }

    @noupgrade
    constructor() {}
}
            ";
        let inner_1_program = "
struct ex_struct {
    arg1: u32,
    arg2: u32,
}
program inner_1.aleo {
    mapping inner_1_mapping: u32 => u32;
    record inner_1_record {
        owner: address,
        val: u32,
    }
    fn inner_1_main(public a: u32, b: u32, c: ex_struct) -> inner_1_record {
        return inner_1_record {
            owner: self.caller,
            val: c.arg1,
        };
    }

    @noupgrade
    constructor() {}
}
";
        let inner_2_program = "
program inner_2.aleo {
    mapping inner_2_mapping: u32 => u32;
    record inner_2_record {
        owner: address,
        val: u32,
    }
    fn inner_2_main(public a: u32, b: u32) -> inner_2_record {
        let c: u32 = a + b;
        return inner_2_record {
            owner: self.caller,
            val: a,
        };
    }

    @noupgrade
    constructor() {}
}
        ";
        // Add dependencies `outer/program.json`
        let add_outer_dependency_1 = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "inner_1".to_string(),
                    source: DependencySource {
                        local: Some(inner_1_directory.clone()),
                        network: false,
                        edition: None,
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
            package: None,
        };

        let add_outer_dependency_2 = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "inner_2".to_string(),
                    source: DependencySource {
                        local: Some(inner_2_directory.clone()),
                        network: false,
                        edition: None,
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
            package: None,
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
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: LeoNew { name: "outer_2".to_string(), library: false, workspace: false },
            },
            path: Some(outer_directory.clone()),
            home: None,
            package: None,
        };

        let create_inner_1_project = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: LeoNew { name: "inner_1".to_string(), library: false, workspace: false },
            },
            path: Some(inner_1_directory.clone()),
            home: None,
            package: None,
        };

        let create_inner_2_project = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::New {
                command: LeoNew { name: "inner_2".to_string(), library: false, workspace: false },
            },
            path: Some(inner_2_directory.clone()),
            home: None,
            package: None,
        };

        // Add source files `outer_2/src/main.leo` and `outer_2/inner/src/main.leo`
        let outer_program = "
import inner_1.aleo;
import inner_2.aleo;
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
program outer_2.aleo {
    record Hello {
        owner: address,
        a: u32,
    }

    fn main(public a: u32, b: u32) -> (inner_2.aleo::Yoo, Hello) {
        let d: inner_1.aleo::Foo = inner_1.aleo::main(1u32,1u32);
        let e: u32 = inner_1.aleo::main_2(inner_1.aleo::Foo {a: a, b: b, c: inner_1.aleo::Boo {a:1u32, b:1u32}});
        let f: Boo = Boo {a:1u32, b:1u32};
        let g: inner_2.aleo::Foo = inner_2.aleo::main(1u32, 1u32);
        inner_2.aleo::Yo_Consumer(inner_2.aleo::Yo());
        let h: inner_2.aleo::Yoo = inner_2.aleo::Yo();
        let i: inner_2.aleo::Goo = inner_2.aleo::Goo_creator();
        let j: Hello = Hello {owner: self.signer, a:1u32};

        return (h, j);
    }

    @noupgrade
    constructor() {}
}
";
        let inner_1_program = "
struct Foo {
    a: u32,
    b: u32,
    c: Boo,
}
struct Boo {
    a: u32,
    b: u32,
}
program inner_1.aleo {
    fn main(public a: u32, b: u32) -> Foo {
        return Foo {a: a, b: b, c: Boo {a:1u32, b:1u32}};
    }
    fn main_2(a:Foo)->u32{
        return a.a;
    }

    @noupgrade
    constructor() {}
}";
        let inner_2_program = "
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
program inner_2.aleo {
    record Yoo {
        owner: address,
        a: u32,
    }
    fn main(public a: u32, b: u32) -> Foo {
        return Foo {a: a, b: b, c: Boo {a:1u32, b:1u32}};
    }
    fn Yo()-> Yoo {
        return Yoo {owner: self.signer, a:1u32};
    }
    fn Yo_Consumer(a: Yoo)->u32 {
        return a.a;
    }
    fn Goo_creator() -> Goo {
        return Goo {a:100u32, b:1u32, c:1u32};
    }

    @noupgrade
    constructor() {}
}";
        // Add dependencies `outer_2/program.json`
        let add_outer_dependency_1 = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "inner_1".to_string(),
                    source: DependencySource {
                        local: Some(inner_1_directory.clone()),
                        network: false,
                        edition: None,
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
            package: None,
        };

        let add_outer_dependency_2 = CLI {
            debug: false,
            quiet: false,
            json_output: None,
            disable_update_check: false,
            command: Commands::Add {
                command: LeoAdd {
                    name: "inner_2".to_string(),
                    source: DependencySource {
                        local: Some(inner_2_directory.clone()),
                        network: false,
                        edition: None,
                        workspace: false,
                        git: None,
                    },
                    git_ref: GitRef { branch: None, tag: None, rev: None },
                    clear: false,
                    dev: false,
                },
            },
            path: Some(outer_directory.clone()),
            home: None,
            package: None,
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
