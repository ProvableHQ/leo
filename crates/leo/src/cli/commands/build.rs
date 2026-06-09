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
use leo_package::{ABI_FILENAME, Package};
use leo_span::Symbol;

use snarkvm::prelude::{CanaryV0, MainnetV0, Process as SvmProcess, Program as SvmProgram, TestnetV0};

use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    rc::Rc,
};

/// Network-typed `Process` used during the disassemble loop. The variants let
/// a single loop body reuse a process across iterations without triplicating
/// the body for each network. Held for the duration of the build's stub
/// resolution, then dropped (a separate `Process` is loaded later for the
/// post-build bulk validation in `validate_compiled_programs_inner`).
enum DisassembleProcess {
    Mainnet(SvmProcess<MainnetV0>),
    Testnet(SvmProcess<TestnetV0>),
    Canary(SvmProcess<CanaryV0>),
}

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
            no_std: options.no_std,
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
    /// Recompile the primary program under a different on-chain name. Set internally
    /// by `leo deploy --rename`; not exposed as a build flag.
    #[clap(skip)]
    pub(crate) rename: Option<String>,
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
        match context.resolve_targets()? {
            Some((_, targets)) => {
                let mut last_package = None;
                for target in &targets {
                    let member_name = target.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    if targets.len() > 1 {
                        println!("\n--- workspace member '{member_name}' ---");
                    }
                    let member_ctx = context.with_path(target.clone());
                    last_package = Some(handle_build(&self, member_ctx)?);
                }
                last_package.ok_or_else(|| crate::errors::custom("No workspace members found.").into())
            }
            None => handle_build(&self, context),
        }
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

    let mut package = if command.options.build_tests {
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

    let build_directory = package.build_directory();
    let source_directory = package.source_directory();
    let main_source_path = source_directory.join("main.leo");

    // Resolve via the manifest so this isn't a test unit under `--build-tests`.
    let primary_name = package.primary_unit().map(|p| p.name);

    // `leo deploy --rename`: recompile the primary program under a different on-chain name.
    let rename_target = apply_rename(command, &mut package, primary_name)?;

    std::fs::create_dir_all(&build_directory).map_err(|err| {
        crate::errors::util_file_io_error(format_args!("Couldn't create directory {}", build_directory.display()), err)
    })?;
    // Clear artifacts from the pre-flat-layout build directory so they don't
    // linger beside the new per-program directories after an upgrade.
    remove_legacy_build_artifacts(&build_directory);

    // Initialize error handler.
    let handler = Handler::default();
    let node_builder = Rc::new(NodeBuilder::default());

    // Manifest opt-out for the implicit `std` library. Propagated to every
    // unit's `Compiler` via `CompilerOptions::no_std`.
    let mut build_options = command.options.clone();
    build_options.no_std = package.manifest.no_std;

    let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();

    // Prebuild the implicit `std` library once. Every per-unit `Compiler` clones `stubs`,
    // so the prebuilt stub is shared across the build instead of recompiled per unit.
    // `inject_std_library` short-circuits when it finds this entry.
    if !build_options.no_std {
        let std_stub = Compiler::build_std_stub(handler.clone(), Rc::clone(&node_builder), network)?;
        stubs.insert(Symbol::intern(leo_std::library_name()), std_stub);
    }

    // All programs to validate through snarkVM's bytecode validator, in dependency order
    // (imports must be loaded before the programs that depend on them).
    let mut compiled_programs: IndexMap<String, ProgramForValidation> = IndexMap::new();

    // Tracks compilation units whose build artifacts have already been written this
    // build, so each unit is written exactly once - by its first (most authoritative)
    // compilation. The package's own program is compiled before its tests, so its
    // primary build is kept rather than a test's re-derived import copy.
    let mut written: HashSet<String> = HashSet::new();

    // Hoist a typed `Process` for the active network so bytecode dependencies are
    // validated through `Process::add_program` *before* `disassemble` runs on them
    // (issue #29399 — `from_function_core` panics on shapes the grammar accepts).
    // `add_program` is contextual, so the same process must be reused across the
    // loop in topological order; the enum lets the network-typed process live
    // across iterations without triplicating the loop body.
    let mut disassemble_process = match network {
        NetworkName::MainnetV0 => DisassembleProcess::Mainnet(SvmProcess::<MainnetV0>::load().map_err(|e| {
            crate::errors::custom(format!("Failed to initialize snarkVM process for disassembler validation: {e}"))
        })?),
        NetworkName::TestnetV0 => DisassembleProcess::Testnet(SvmProcess::<TestnetV0>::load().map_err(|e| {
            crate::errors::custom(format!("Failed to initialize snarkVM process for disassembler validation: {e}"))
        })?),
        NetworkName::CanaryV0 => DisassembleProcess::Canary(SvmProcess::<CanaryV0>::load().map_err(|e| {
            crate::errors::custom(format!("Failed to initialize snarkVM process for disassembler validation: {e}"))
        })?),
    };

    for unit in &package.compilation_units {
        let unit_name = unit.name.to_string();
        // Normalize key so `"foo.aleo"` and `"foo"` don't collide on the same dir.
        let unit_key = leo_package::bare_unit_name(&unit_name).to_string();
        match &unit.data {
            leo_package::ProgramData::Bytecode(bytecode) => {
                // This was a network dependency or local .aleo dependency, and we have its bytecode.
                let build_path = package.unit_bytecode_path(&unit_name);

                // Write the .aleo file into the program's own build directory.
                if written.insert(unit_key.clone()) {
                    ensure_parent_dir(&build_path)?;
                    std::fs::write(&build_path, bytecode).map_err(crate::errors::failed_to_load_instructions)?;
                }

                // Track the stub. Validates via `Process::add_program` and disassembles in
                // one step; the hoisted process accumulates dependencies across iterations.
                let stub = match &mut disassemble_process {
                    DisassembleProcess::Mainnet(p) => {
                        leo_disassembler::disassemble_from_str::<MainnetV0>(unit.name, bytecode, p)
                    }
                    DisassembleProcess::Testnet(p) => {
                        leo_disassembler::disassemble_from_str::<TestnetV0>(unit.name, bytecode, p)
                    }
                    DisassembleProcess::Canary(p) => {
                        leo_disassembler::disassemble_from_str::<CanaryV0>(unit.name, bytecode, p)
                    }
                }?;

                stubs.insert(unit.name, stub.into());

                compiled_programs.entry(unit_key.clone()).or_insert(ProgramForValidation {
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
                            crate::errors::failed_to_open_file(format_args!(
                                "Failed to find directory for test {}",
                                source.display()
                            ))
                        })?
                        .to_path_buf()
                } else {
                    directory.join("src")
                };

                let is_main = source == &main_source_path;
                if is_main || unit.kind.is_test() {
                    // Compile the program (main or test). AST snapshots, if enabled, land
                    // under this unit's own `build/<name>/snapshots/` directory.
                    let snapshots_directory = package.unit_snapshots_directory(&unit_name);
                    let compiled = compile_leo_source_directory(
                        source, // entry file
                        &source_dir,
                        unit.name,
                        unit.kind.is_test(),
                        &snapshots_directory,
                        &handler,
                        &node_builder,
                        build_options.clone(),
                        stubs.clone(),
                        network,
                        if is_main { rename_target.clone() } else { None },
                    )?;

                    // Write this unit's compiled bytecode. ABI and interface ABIs are
                    // emitted only for the main program; tests deliberately skip them.
                    let primary_path = package.unit_bytecode_path(&unit_name);
                    if written.insert(unit_key.clone()) {
                        ensure_parent_dir(&primary_path)?;
                        std::fs::write(&primary_path, &compiled.primary.bytecode)
                            .map_err(crate::errors::failed_to_load_instructions)?;
                        if is_main {
                            let abi_path = package.unit_abi_path(&unit_name);
                            let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
                                .map_err(|e| crate::errors::failed_to_serialize_abi(e.to_string()))?;
                            std::fs::write(&abi_path, abi_json).map_err(crate::errors::failed_to_write_abi)?;
                            tracing::info!("✅ Generated ABI for program '{unit_name}'.");
                            let interfaces_directory = package.unit_interfaces_directory(&unit_name);
                            write_interface_abis(&interfaces_directory, &compiled.interfaces)?;
                        }
                    }

                    // Write each import's bytecode and ABI into its own build directory.
                    for import in &compiled.imports {
                        let import_path = package.unit_bytecode_path(&import.name);
                        let import_key = leo_package::bare_unit_name(&import.name).to_string();
                        if written.insert(import_key.clone()) {
                            ensure_parent_dir(&import_path)?;
                            std::fs::write(&import_path, &import.bytecode)
                                .map_err(crate::errors::failed_to_load_instructions)?;

                            let import_abi_path = package.unit_abi_path(&import.name);
                            let import_abi_json = serde_json::to_string_pretty(&import.abi)
                                .map_err(|e| crate::errors::failed_to_serialize_abi(e.to_string()))?;
                            std::fs::write(&import_abi_path, import_abi_json)
                                .map_err(crate::errors::failed_to_write_abi)?;
                        }

                        // Queue import for validation.
                        compiled_programs.entry(import_key).or_insert(ProgramForValidation {
                            bytecode: import.bytecode.clone(),
                            path: import_path,
                            is_leo_compiled: true,
                        });
                    }
                    // Queue the primary program.
                    compiled_programs.entry(unit_key.clone()).or_insert(ProgramForValidation {
                        bytecode: compiled.primary.bytecode.clone(),
                        path: primary_path,
                        is_leo_compiled: true,
                    });
                }

                if unit.kind.is_library() {
                    // The primary library runs the full frontend (name validation through static
                    // analysis) so type errors, undefined names, and interface cycles are caught
                    // even when no downstream program consumes the library. Non-primary library
                    // dependencies are parsed only; their semantics are validated when their own
                    // `leo build` is run.
                    let library = if primary_name == Some(unit.name) {
                        let snapshots_directory = package.unit_snapshots_directory(&unit_name);
                        let (lib, interfaces) = build_leo_source_directory_library(
                            source,
                            &source_dir,
                            unit.name,
                            &snapshots_directory,
                            &handler,
                            &node_builder,
                            build_options.clone(),
                            stubs.clone(),
                            network,
                        )?;

                        // Write interface ABIs for the primary library.
                        let interfaces_directory = package.unit_interfaces_directory(&unit_name);
                        write_interface_abis(&interfaces_directory, &interfaces)?;

                        lib
                    } else {
                        parse_leo_source_directory_library(
                            source,
                            &source_dir,
                            unit.name,
                            &handler,
                            &node_builder,
                            build_options.clone(),
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
                    // Parse the primary program (for its stub) and intermediate dependencies; the
                    // primary's stub adopts the rename too, or its parse would fail the name check.
                    let leo_program = parse_leo_source_directory(
                        source,
                        &source_dir,
                        unit.name,
                        &handler,
                        &node_builder,
                        build_options.clone(),
                        network,
                        if is_main { rename_target.clone() } else { None },
                    )?;

                    stubs.insert(unit.name, leo_program.into());
                }
            }
        }
    }

    // Ensure every program unit has on-disk bytecode for downstream commands; the
    // parse-only path above skips manifest deps the main source doesn't `import`.
    for unit in &package.compilation_units {
        if !unit.kind.is_program() || unit.kind.is_test() {
            continue;
        }
        let leo_package::ProgramData::SourcePath { directory, source } = &unit.data else { continue };
        let unit_name = unit.name.to_string();
        let unit_key = leo_package::bare_unit_name(&unit_name).to_string();
        if !written.insert(unit_key.clone()) {
            continue;
        }
        let source_dir = directory.join("src");
        let snapshots_directory = package.unit_snapshots_directory(&unit_name);
        let compiled = compile_leo_source_directory(
            source,
            &source_dir,
            unit.name,
            false,
            &snapshots_directory,
            &handler,
            &node_builder,
            build_options.clone(),
            stubs.clone(),
            network,
            // Dependencies are never renamed; only the primary deploy target is.
            None,
        )?;
        let primary_path = package.unit_bytecode_path(&unit_name);
        ensure_parent_dir(&primary_path)?;
        std::fs::write(&primary_path, &compiled.primary.bytecode)
            .map_err(crate::errors::failed_to_load_instructions)?;
        let abi_path = package.unit_abi_path(&unit_name);
        let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
            .map_err(|e| crate::errors::failed_to_serialize_abi(e.to_string()))?;
        std::fs::write(&abi_path, abi_json).map_err(crate::errors::failed_to_write_abi)?;
        let interfaces_directory = package.unit_interfaces_directory(&unit_name);
        write_interface_abis(&interfaces_directory, &compiled.interfaces)?;
        compiled_programs.entry(unit_key).or_insert(ProgramForValidation {
            bytecode: compiled.primary.bytecode.clone(),
            path: primary_path,
            is_leo_compiled: true,
        });
    }

    // Validate generated bytecode through snarkVM's type checker.
    validate_compiled_programs(&compiled_programs, network)?;

    Ok(package)
}

/// Resolves `leo deploy --rename` and applies it to `package`.
///
/// Returns `Ok(None)` when no rename was requested. Otherwise validates the
/// requested name, rejects conflicts, and rewrites the primary unit's name in
/// `package` so the build artifacts, the deploy read path, and the compiled
/// bytecode all share the renamed identity; the canonical `.aleo`-suffixed name
/// to compile under is returned. Programs that import the original name are
/// intentionally not redirected to the renamed copy.
fn apply_rename(command: &LeoBuild, package: &mut Package, primary_name: Option<Symbol>) -> Result<Option<String>> {
    let Some(requested) = &command.rename else {
        return Ok(None);
    };

    // `--rename` rewrites a single primary program, so it cannot apply to a test build:
    // tests keep their original names and would dangle against the renamed primary.
    if command.options.build_tests {
        return Err(crate::errors::custom("`--rename` cannot be combined with `--build-tests`.").into());
    }

    let renamed = leo_package::canonicalize_program_name(requested);
    if !leo_package::is_valid_program_name(&renamed) {
        return Err(crate::errors::custom(format!(
            "Invalid program name '{requested}' for `--rename`; expected a valid Aleo program name."
        ))
        .into());
    }

    let Some(original) = primary_name else {
        return Err(crate::errors::custom("`--rename` requires a primary program to rename.").into());
    };

    // Compare on the bare name: a local primary's name is bare while the target is
    // `.aleo`-suffixed, so a direct symbol comparison would never flag a no-op rename.
    let renamed_bare = leo_package::bare_unit_name(&renamed);
    if renamed_bare == leo_package::bare_unit_name(&original.to_string()) {
        return Err(crate::errors::custom(format!(
            "`--rename` target '{renamed}' is identical to the program's current name."
        ))
        .into());
    }

    // Reject renaming onto a name already used by another unit or dependency. Build
    // artifacts are keyed by bare unit name, so a collision would silently discard the
    // renamed program and deploy the colliding unit's bytecode instead. Compare on the
    // bare name to match that keying.
    if package
        .compilation_units
        .iter()
        .any(|unit| unit.name != original && leo_package::bare_unit_name(&unit.name.to_string()) == renamed_bare)
    {
        return Err(crate::errors::custom(format!(
            "`--rename` target '{renamed}' conflicts with an existing program or dependency in this package; choose a different name."
        ))
        .into());
    }

    let renamed_symbol = Symbol::intern(&renamed);
    for unit in package.compilation_units.iter_mut() {
        if !unit.kind.is_test() && unit.name == original {
            unit.name = renamed_symbol;
        }
    }

    Ok(Some(renamed))
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
    rename: Option<String>,
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
    // When set, recompile the program scope under this on-chain name (`leo deploy --rename`).
    compiler.rename = rename;

    // Compile the Leo program into Aleo instructions.
    let compiled = compiler.compile_from_directory(entry_file_path, source_directory)?;
    let primary_bytecode = &compiled.primary.bytecode;

    // Check the program size limit for each bytecode.
    use leo_package::MAX_PROGRAM_SIZE;
    let program_size = primary_bytecode.len();

    if program_size > MAX_PROGRAM_SIZE {
        return Err(crate::errors::program_size_limit_exceeded(program_name, program_size, MAX_PROGRAM_SIZE).into());
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
#[allow(clippy::too_many_arguments)]
fn parse_leo_source_directory(
    entry_file_path: &Path,
    source_directory: &Path,
    program_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    network: NetworkName,
    rename: Option<String>,
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
    // When set, rewrite the parsed program scope under this on-chain name so the stub
    // matches the renamed identity (`leo deploy --rename`).
    compiler.rename = rename;

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
    let process = SvmProcess::<N>::load().map_err(|e| {
        crate::errors::custom(format!("Failed to initialize snarkVM process for bytecode validation: {e}"))
    })?;

    for (name, ProgramForValidation { bytecode, path, is_leo_compiled }) in programs {
        let program =
            SvmProgram::<N>::from_str(bytecode).map_err(|e| crate::errors::failed_to_parse_aleo_file(name, e))?;

        let checksum = program.to_checksum().iter().join(", ");

        process.lock().add_program_with_edition(&program, LOCAL_PROGRAM_DEFAULT_EDITION).map_err(|e| {
            if *is_leo_compiled {
                crate::errors::generated_invalid_bytecode(name, path.display(), &checksum, e)
            } else {
                crate::errors::custom(format!(
                    "snarkVM rejected external program '{name}' during build validation: {e}"
                ))
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
/// Returns the validated library and any interface ABIs defined in it.
#[allow(clippy::too_many_arguments)]
fn build_leo_source_directory_library(
    entry_file_path: &Path,
    source_directory: &Path,
    library_name: Symbol,
    snapshots_directory: &Path,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
    network: NetworkName,
) -> Result<(leo_ast::Library, Vec<leo_abi::interfaces::CompiledInterface>)> {
    // Print a newline for better formatting.
    println!();
    tracing::info!("🔨 Building library '{library_name}'");

    let mut compiler = Compiler::new(
        Some(library_name.to_string()),
        false,
        handler.clone(),
        Rc::clone(node_builder),
        snapshots_directory.to_path_buf(),
        Some(options.into()),
        stubs,
        network,
    );

    let library = compiler.build_library_from_directory(library_name, entry_file_path, source_directory)?;
    let interfaces = compiler.generate_library_interface_abis();

    tracing::info!("✅ Validated '{library_name}'.");

    Ok((library, interfaces))
}

/// Writes interface ABI JSON files to the interfaces directory.
fn write_interface_abis(interfaces_dir: &Path, interfaces: &[leo_abi::interfaces::CompiledInterface]) -> Result<()> {
    // Remove stale files from a previous build (renamed/deleted interfaces).
    if interfaces_dir.exists() {
        std::fs::remove_dir_all(interfaces_dir).map_err(crate::errors::failed_to_write_abi)?;
    }
    if interfaces.is_empty() {
        return Ok(());
    }
    for ci in interfaces {
        let mut file_path = match &ci.owner {
            leo_abi::interfaces::InterfaceOwner::Local => interfaces_dir.to_path_buf(),
            leo_abi::interfaces::InterfaceOwner::External { owner_program } => interfaces_dir.join(owner_program),
        };
        // Module path segments (everything except the final name).
        for seg in &ci.abi.path[..ci.abi.path.len().saturating_sub(1)] {
            file_path.push(seg);
        }
        std::fs::create_dir_all(&file_path).map_err(crate::errors::failed_to_write_abi)?;
        file_path.push(format!("{}.json", ci.abi.name));
        let json =
            serde_json::to_string_pretty(&ci.abi).map_err(|e| crate::errors::failed_to_serialize_abi(e.to_string()))?;
        std::fs::write(&file_path, json).map_err(crate::errors::failed_to_write_abi)?;
    }
    tracing::info!("✅ Generated {} interface ABI(s).", interfaces.len());
    Ok(())
}

/// Ensure the parent directory of `path` exists, creating it if necessary.
fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            crate::errors::util_file_io_error(format_args!("Couldn't create directory {}", parent.display()), err)
        })?;
    }
    Ok(())
}

/// Remove artifacts left by the pre-flat-layout build directory (`build/main.aleo`,
/// `build/imports/`, etc.) so they don't linger beside the new per-program
/// directories after an upgrade. Best-effort: missing entries are ignored.
///
/// Gated on a top-level legacy artifact so we don't wipe a perfectly valid new-layout
/// per-unit directory for a user program literally named `imports` or `interfaces`.
fn remove_legacy_build_artifacts(build_directory: &Path) {
    let is_legacy =
        build_directory.join("main.aleo").exists() || build_directory.join(leo_package::MANIFEST_FILENAME).exists();
    if !is_legacy {
        return;
    }
    for file in ["main.aleo", ABI_FILENAME, leo_package::MANIFEST_FILENAME] {
        let _ = std::fs::remove_file(build_directory.join(file));
    }
    for dir in ["imports", "interfaces"] {
        let _ = std::fs::remove_dir_all(build_directory.join(dir));
    }
}
