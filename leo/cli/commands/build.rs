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

use super::*;

use leo_ast::{NetworkName, NodeBuilder, Program, Stub};
use leo_compiler::{AstSnapshots, Compiled, Compiler, CompilerOptions};
use leo_errors::{CliError, UtilError};
use leo_package::{ABI_FILENAME, BUILD_DIRECTORY, Manifest, Package};
use leo_span::Symbol;

use snarkvm::prelude::{CanaryV0, MainnetV0, Program as SvmProgram, TestnetV0};

use indexmap::IndexMap;
use itertools::Itertools;
use std::{path::Path, rc::Rc};

impl From<BuildOptions> for CompilerOptions {
    fn from(options: BuildOptions) -> Self {
        Self {
            ast_spans_enabled: options.enable_ast_spans,
            ast_snapshots: if options.enable_all_ast_snapshots {
                AstSnapshots::All
            } else {
                AstSnapshots::Some(options.ast_snapshots.into_iter().collect())
            },
            initial_ast: options.enable_all_ast_snapshots | options.enable_initial_ast_snapshot,
        }
    }
}

/// Compile and build program command.
#[derive(Parser, Debug)]
pub struct LeoBuild {
    #[clap(flatten)]
    pub(crate) options: BuildOptions,
    #[clap(flatten)]
    pub(crate) env_override: EnvOptions,
}

impl Command for LeoBuild {
    type Input = ();
    type Output = Package;

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Build the program.
        handle_build(&self, context)
    }
}

// A helper function to handle the build command.
fn handle_build(command: &LeoBuild, context: Context) -> Result<<LeoBuild as Command>::Output> {
    // Get the package path and home directory.
    let package_path = context.dir()?;
    let home_path = context.home()?;

    // Get the network, defaulting to `TestnetV0` if none is specified.
    let network = match get_network(&command.env_override.network) {
        Ok(network) => network,
        Err(_) => {
            println!("⚠️ No network specified, defaulting to 'testnet'.");
            NetworkName::TestnetV0
        }
    };

    // Get the endpoint, if it is provided.
    let endpoint = match get_endpoint(&command.env_override.endpoint) {
        Ok(endpoint) => endpoint,
        Err(_) => {
            println!("⚠️ No endpoint specified, defaulting to '{}'.", DEFAULT_ENDPOINT);
            DEFAULT_ENDPOINT.to_string()
        }
    };

    let package = if command.options.build_tests {
        Package::from_directory_with_tests(
            &package_path,
            &home_path,
            command.options.no_cache,
            command.options.no_local,
            Some(network),
            Some(&endpoint),
        )?
    } else {
        Package::from_directory(
            &package_path,
            &home_path,
            command.options.no_cache,
            command.options.no_local,
            Some(network),
            Some(&endpoint),
        )?
    };

    // Check the manifest for the compiler version.
    // If it does not match, warn the user and continue.
    if package.manifest.leo != env!("CARGO_PKG_VERSION") {
        tracing::warn!(
            "The Leo compiler version in the manifest ({}) does not match the current version ({}).",
            package.manifest.leo,
            env!("CARGO_PKG_VERSION")
        );
    }

    let outputs_directory = package.outputs_directory();
    let build_directory = package.build_directory();
    let imports_directory = package.imports_directory();
    let source_directory = package.source_directory();
    let main_source_path = source_directory.join("main.leo");

    for dir in [&outputs_directory, &build_directory, &imports_directory] {
        std::fs::create_dir_all(dir).map_err(|err| {
            UtilError::util_file_io_error(format_args!("Couldn't create directory {}", dir.display()), err)
        })?;
    }

    // Initialize error handler.
    let handler = Handler::default();
    let node_builder = Rc::new(NodeBuilder::default());

    let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();

    for program in &package.programs {
        match &program.data {
            leo_package::ProgramData::Bytecode(bytecode) => {
                // This was a network dependency or local .aleo dependency, and we have its bytecode.
                let build_path = imports_directory.join(format!("{}.aleo", program.name));

                // Write the .aleo file.
                std::fs::write(&build_path, bytecode).map_err(CliError::failed_to_load_instructions)?;

                // Track the Stub.
                let stub = match network {
                    NetworkName::MainnetV0 => {
                        leo_disassembler::disassemble_from_str::<MainnetV0>(program.name, bytecode)
                    }
                    NetworkName::TestnetV0 => {
                        leo_disassembler::disassemble_from_str::<TestnetV0>(program.name, bytecode)
                    }
                    NetworkName::CanaryV0 => leo_disassembler::disassemble_from_str::<CanaryV0>(program.name, bytecode),
                }?;

                stubs.insert(program.name, stub.into());
            }

            leo_package::ProgramData::SourcePath { directory, source } => {
                // This is a local dependency, so we must compile or parse it.
                let source_dir = directory.join("src");

                if source == &main_source_path || program.is_test {
                    // Compile the program (main or test).
                    let compiled = compile_leo_source_directory(
                        source, // entry file
                        &source_dir,
                        program.name,
                        program.is_test,
                        &outputs_directory,
                        &handler,
                        &node_builder,
                        command.options.clone(),
                        stubs.clone(),
                        network,
                    )?;

                    // Where to write the primary bytecode?
                    let primary_path = if source == &main_source_path {
                        build_directory.join("main.aleo")
                    } else {
                        imports_directory.join(format!("{}.aleo", program.name))
                    };

                    // Write the primary program bytecode.
                    std::fs::write(&primary_path, &compiled.primary.bytecode)
                        .map_err(CliError::failed_to_load_instructions)?;

                    // Write imports (bytecode and ABI).
                    for import in &compiled.imports {
                        let import_path = imports_directory.join(format!("{}.aleo", import.name));
                        std::fs::write(&import_path, &import.bytecode)
                            .map_err(CliError::failed_to_load_instructions)?;

                        let import_abi_path = imports_directory.join(format!("{}.abi.json", import.name));
                        let import_abi_json = serde_json::to_string_pretty(&import.abi)
                            .map_err(|e| CliError::failed_to_serialize_abi(e.to_string()))?;
                        std::fs::write(&import_abi_path, import_abi_json).map_err(CliError::failed_to_write_abi)?;
                    }

                    // Write the ABI file for the main program.
                    if source == &main_source_path {
                        let abi_path = build_directory.join(ABI_FILENAME);
                        let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
                            .map_err(|e| CliError::failed_to_serialize_abi(e.to_string()))?;
                        std::fs::write(&abi_path, abi_json).map_err(CliError::failed_to_write_abi)?;
                        tracing::info!("✅ Generated ABI at '{BUILD_DIRECTORY}/{ABI_FILENAME}'.");
                    }
                }

                // Parse intermediate dependencies only.
                let leo_program = parse_leo_source_directory(
                    source,
                    &source_dir,
                    program.name,
                    &handler,
                    &node_builder,
                    command.options.clone(),
                    network,
                )?;

                stubs.insert(program.name, leo_program.into());
            }
        }
    }

    // SnarkVM expects to find a `program.json` file in the build directory, so make
    // a bogus one.
    let build_manifest_path = build_directory.join(leo_package::MANIFEST_FILENAME);
    let fake_manifest = Manifest {
        program: package.manifest.program.clone(),
        version: "0.1.0".to_string(),
        description: String::new(),
        license: String::new(),
        leo: env!("CARGO_PKG_VERSION").to_string(),
        dependencies: None,
        dev_dependencies: None,
    };
    fake_manifest.write_to_file(build_manifest_path)?;

    Ok(package)
}

/// Compiles a Leo file. Writes and returns the compiled bytecode and ABI.
#[allow(clippy::too_many_arguments)]
fn compile_leo_source_directory(
    entry_file_path: &Path,
    source_directory: &Path,
    program_name: Symbol,
    is_test: bool,
    output_path: &Path,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
    network: NetworkName,
) -> Result<Compiled> {
    // Create a new instance of the Leo compiler.
    let mut compiler = Compiler::new(
        Some(program_name.to_string()),
        is_test,
        handler.clone(),
        Rc::clone(node_builder),
        output_path.to_path_buf(),
        Some(options.into()),
        stubs,
        network,
    );

    // Compile the Leo program into Aleo instructions.
    let compiled = compiler.compile_from_directory(entry_file_path, source_directory)?;
    let primary_bytecode = &compiled.primary.bytecode;

    // Check the program size limit for each bytecode.
    use leo_package::MAX_PROGRAM_SIZE;
    let program_size = primary_bytecode.len();

    if program_size > MAX_PROGRAM_SIZE {
        return Err(leo_errors::LeoError::UtilError(UtilError::program_size_limit_exceeded(
            program_name,
            program_size,
            MAX_PROGRAM_SIZE,
        )));
    }

    // Get the AVM bytecode.
    let checksum: String = match network {
        NetworkName::MainnetV0 => SvmProgram::<MainnetV0>::from_str(primary_bytecode)?.to_checksum().iter().join(", "),
        NetworkName::TestnetV0 => SvmProgram::<TestnetV0>::from_str(primary_bytecode)?.to_checksum().iter().join(", "),
        NetworkName::CanaryV0 => SvmProgram::<CanaryV0>::from_str(primary_bytecode)?.to_checksum().iter().join(", "),
    };

    tracing::info!("    {} statements before dead code elimination.", compiler.statements_before_dce);
    tracing::info!("    {} statements after dead code elimination.", compiler.statements_after_dce);
    tracing::info!("    The program checksum is: '[{checksum}]'.");

    let (size_kb, max_kb, warning) = format_program_size(program_size, MAX_PROGRAM_SIZE);
    tracing::info!("    Program size: {size_kb:.2} KB / {max_kb:.2} KB");
    if let Some(msg) = warning {
        tracing::warn!("⚠️  Program '{program_name}' is {msg}.");
    }

    tracing::info!("✅ Compiled '{program_name}.aleo' into Aleo instructions.");

    // Print checksums for all additional bytecodes (imports).
    for import in &compiled.imports {
        // Compute checksum depending on network.
        let dep_checksum: String = match network {
            NetworkName::MainnetV0 => {
                SvmProgram::<MainnetV0>::from_str(&import.bytecode)?.to_checksum().iter().join(", ")
            }
            NetworkName::TestnetV0 => {
                SvmProgram::<TestnetV0>::from_str(&import.bytecode)?.to_checksum().iter().join(", ")
            }
            NetworkName::CanaryV0 => {
                SvmProgram::<CanaryV0>::from_str(&import.bytecode)?.to_checksum().iter().join(", ")
            }
        };

        tracing::info!("    Import '{}.aleo': checksum = '[{dep_checksum}]'", import.name);
    }

    Ok(compiled)
}

/// Parses a Leo file into an AST without generating bytecode.
fn parse_leo_source_directory(
    entry_file_path: &Path,
    source_directory: &Path,
    program_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    network: NetworkName,
) -> Result<Program> {
    // Create a new instance of the Leo compiler.
    let mut compiler = Compiler::new(
        Some(program_name.to_string()),
        false,
        handler.clone(),
        Rc::clone(node_builder),
        std::path::PathBuf::default(),
        Some(options.into()),
        IndexMap::new(),
        network,
    );

    // Parse the Leo program into an AST.
    compiler.parse_from_directory(entry_file_path, source_directory)
}
