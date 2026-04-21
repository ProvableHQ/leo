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

use snarkvm::prelude::{CanaryV0, MainnetV0, Process as SvmProcess, Program as SvmProgram, TestnetV0};

use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

/// A program queued for bytecode validation after the build.
struct ProgramForValidation {
    /// The Aleo bytecode.
    bytecode: String,
    /// Path to the bytecode file on disk, used for error reporting.
    path: PathBuf,
    /// Whether the program was compiled from Leo source (`true`) or loaded as external bytecode (`false`).
    is_leo_compiled: bool,
}

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
            command.env_override.network_retries,
        )?
    } else {
        Package::from_directory(
            &package_path,
            &home_path,
            command.options.no_cache,
            command.options.no_local,
            Some(network),
            Some(&endpoint),
            command.env_override.network_retries,
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

    // Use the flag already computed by Package during dependency resolution rather than
    // re-checking the filesystem. The main program is always last in topological order.
    let primary_name = package.compilation_units.last().map(|p| p.name);
    let is_library = package.compilation_units.last().map(|p| p.kind.is_library()).unwrap_or(false);
    if !is_library {
        for dir in [&outputs_directory, &build_directory, &imports_directory] {
            std::fs::create_dir_all(dir).map_err(|err| {
                UtilError::util_file_io_error(format_args!("Couldn't create directory {}", dir.display()), err)
            })?;
        }
    }

    // Initialize error handler.
    let handler = Handler::default();
    let node_builder = Rc::new(NodeBuilder::default());

    let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();

    // All programs to validate through snarkVM's bytecode validator, in dependency order
    // (imports must be loaded before the programs that depend on them).
    let mut compiled_programs: IndexMap<String, ProgramForValidation> = IndexMap::new();

    for unit in &package.compilation_units {
        match &unit.data {
            leo_package::ProgramData::Bytecode(bytecode) => {
                // This was a network dependency or local .aleo dependency, and we have its bytecode.
                let build_path = imports_directory.join(format!("{}", unit.name));

                // Write the .aleo file.
                std::fs::write(&build_path, bytecode).map_err(CliError::failed_to_load_instructions)?;

                // Track the stub.
                let stub = match network {
                    NetworkName::MainnetV0 => leo_disassembler::disassemble_from_str::<MainnetV0>(unit.name, bytecode),
                    NetworkName::TestnetV0 => leo_disassembler::disassemble_from_str::<TestnetV0>(unit.name, bytecode),
                    NetworkName::CanaryV0 => leo_disassembler::disassemble_from_str::<CanaryV0>(unit.name, bytecode),
                }?;

                stubs.insert(unit.name, stub.into());

                compiled_programs.entry(unit.name.to_string()).or_insert(ProgramForValidation {
                    bytecode: bytecode.clone(),
                    path: build_path,
                    is_leo_compiled: false,
                });
            }

            leo_package::ProgramData::SourcePath { directory, source } => {
                // This is a local dependency, so we must compile or parse it.
                let source_dir = if unit.kind.is_test() {
                    source
                        .parent()
                        .ok_or_else(|| {
                            UtilError::failed_to_open_file(format_args!(
                                "Failed to find directory for test {}",
                                source.display()
                            ))
                        })?
                        .to_path_buf()
                } else {
                    directory.join("src")
                };

                if source == &main_source_path || unit.kind.is_test() {
                    // Compile the program (main or test).
                    let compiled = compile_leo_source_directory(
                        source, // entry file
                        &source_dir,
                        unit.name,
                        unit.kind.is_test(),
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
                        imports_directory.join(format!("{}", unit.name))
                    };

                    // Write the primary program bytecode.
                    std::fs::write(&primary_path, &compiled.primary.bytecode)
                        .map_err(CliError::failed_to_load_instructions)?;

                    // Write imports (bytecode and ABI) and queue for validation.
                    for import in &compiled.imports {
                        let import_path = imports_directory.join(&import.name);
                        std::fs::write(&import_path, &import.bytecode)
                            .map_err(CliError::failed_to_load_instructions)?;

                        let import_abi_path = imports_directory.join(format!("{}.abi.json", import.name));
                        let import_abi_json = serde_json::to_string_pretty(&import.abi)
                            .map_err(|e| CliError::failed_to_serialize_abi(e.to_string()))?;
                        std::fs::write(&import_abi_path, import_abi_json).map_err(CliError::failed_to_write_abi)?;

                        // Queue import for validation.
                        compiled_programs.entry(import.name.clone()).or_insert(ProgramForValidation {
                            bytecode: import.bytecode.clone(),
                            path: import_path,
                            is_leo_compiled: true,
                        });
                    }
                    // Queue the primary program.
                    compiled_programs.entry(unit.name.to_string()).or_insert(ProgramForValidation {
                        bytecode: compiled.primary.bytecode.clone(),
                        path: primary_path,
                        is_leo_compiled: true,
                    });

                    // Write the ABI file for the main program.
                    if source == &main_source_path {
                        let abi_path = build_directory.join(ABI_FILENAME);
                        let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
                            .map_err(|e| CliError::failed_to_serialize_abi(e.to_string()))?;
                        std::fs::write(&abi_path, abi_json).map_err(CliError::failed_to_write_abi)?;
                        tracing::info!("✅ Generated ABI at '{BUILD_DIRECTORY}/{ABI_FILENAME}'.");
                    }
                }

                if unit.kind.is_library() {
                    // The primary library runs the full frontend (name validation through static
                    // analysis) so type errors, undefined names, and interface cycles are caught
                    // even when no downstream program consumes the library. Non-primary library
                    // dependencies are parsed only; their semantics are validated when their own
                    // `leo build` is run.
                    let library = if primary_name == Some(unit.name) {
                        build_leo_source_directory_library(
                            source,
                            &source_dir,
                            unit.name,
                            &handler,
                            &node_builder,
                            command.options.clone(),
                            stubs.clone(),
                            network,
                        )?
                    } else {
                        parse_leo_source_directory_library(
                            source,
                            &source_dir,
                            unit.name,
                            &handler,
                            &node_builder,
                            command.options.clone(),
                            network,
                        )?
                    };
                    // Bail out if any errors were collected (parse errors produce ErrExpression
                    // nodes that would panic in later passes, and frontend-pass errors surface here).
                    handler.last_err()?;
                    // Compute parents from dep_graph
                    let mut library_stub: Stub = library.into();
                    for node in package.dep_graph.nodes() {
                        if package.dep_graph.neighbors(node).any(|dep| dep == &unit.name) {
                            library_stub.add_parent(*node);
                        }
                    }
                    stubs.insert(unit.name, library_stub);
                } else {
                    // Parse intermediate dependencies only.
                    let leo_program = parse_leo_source_directory(
                        source,
                        &source_dir,
                        unit.name,
                        &handler,
                        &node_builder,
                        command.options.clone(),
                        network,
                    )?;

                    stubs.insert(unit.name, leo_program.into());
                }
            }
        }
    }

    // Validate generated bytecode through snarkVM's type checker.
    validate_compiled_programs(&compiled_programs, network)?;

    if !is_library {
        // SnarkVM expects to find a `program.json` file in the build directory, so make a bogus one.
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
    }

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
    // Print a newline for better formatting.
    println!();
    tracing::info!("🔨 Compiling '{program_name}'");
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

    tracing::info!("✅ Compiled '{program_name}' into Aleo instructions.");

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

        tracing::info!("    Import '{}': checksum = '[{dep_checksum}]'", import.name);
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
    compiler.parse_program_from_directory(entry_file_path, source_directory)
}

/// Validates compiled Aleo bytecode by loading all programs into a snarkVM `Process`.
/// Note that the programs must be provided in dependency order.
fn validate_compiled_programs(programs: &IndexMap<String, ProgramForValidation>, network: NetworkName) -> Result<()> {
    match network {
        NetworkName::MainnetV0 => validate_compiled_programs_inner::<MainnetV0>(programs),
        NetworkName::TestnetV0 => validate_compiled_programs_inner::<TestnetV0>(programs),
        NetworkName::CanaryV0 => validate_compiled_programs_inner::<CanaryV0>(programs),
    }
}

/// Network-generic implementation of [`validate_compiled_programs`].
fn validate_compiled_programs_inner<N: snarkvm::prelude::Network>(
    programs: &IndexMap<String, ProgramForValidation>,
) -> Result<()> {
    let mut process = SvmProcess::<N>::load()
        .map_err(|e| CliError::custom(format!("Failed to initialize snarkVM process for bytecode validation: {e}")))?;

    for (name, ProgramForValidation { bytecode, path, is_leo_compiled }) in programs {
        let program = SvmProgram::<N>::from_str(bytecode).map_err(|e| CliError::failed_to_parse_aleo_file(name, e))?;

        let checksum = program.to_checksum().iter().join(", ");

        process.add_program_with_edition(&program, LOCAL_PROGRAM_DEFAULT_EDITION).map_err(|e| {
            if *is_leo_compiled {
                CliError::generated_invalid_bytecode(name, path.display(), &checksum, e)
            } else {
                CliError::custom(format!("snarkVM rejected external program '{name}' during build validation: {e}"))
            }
        })?;
    }

    Ok(())
}

/// Parses a Leo file into an AST without generating bytecode.
fn parse_leo_source_directory_library(
    entry_file_path: &Path,
    source_directory: &Path,
    library_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    network: NetworkName,
) -> Result<leo_ast::Library> {
    // Create a new instance of the Leo compiler.
    let mut compiler = Compiler::new(
        Some(library_name.to_string()),
        false,
        handler.clone(),
        Rc::clone(node_builder),
        std::path::PathBuf::default(),
        Some(options.into()),
        IndexMap::new(),
        network,
    );

    // Parse the Leo program into an AST.
    compiler.parse_library_from_directory(library_name, entry_file_path, source_directory)
}

/// Builds a library by running all frontend passes. Does not generate bytecode.
#[allow(clippy::too_many_arguments)]
fn build_leo_source_directory_library(
    entry_file_path: &Path,
    source_directory: &Path,
    library_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
    network: NetworkName,
) -> Result<leo_ast::Library> {
    // Print a newline for better formatting.
    println!();
    tracing::info!("🔨 Building library '{library_name}'");

    let mut compiler = Compiler::new(
        Some(library_name.to_string()),
        false,
        handler.clone(),
        Rc::clone(node_builder),
        std::path::PathBuf::default(),
        Some(options.into()),
        stubs,
        network,
    );

    let library = compiler.build_library_from_directory(library_name, entry_file_path, source_directory)?;

    tracing::info!("✅ Validated '{library_name}'.");

    Ok(library)
}
