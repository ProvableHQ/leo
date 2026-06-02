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

//! Native-only `leo build` core. Migrated out of
//! `crates/leo/src/cli/commands/build.rs`. The `LeoBuild` clap struct in
//! `crates/leo` now just collects flags and calls [`handle_build`].

#![cfg(not(target_arch = "wasm32"))]

use crate::{
    commands::{LOCAL_PROGRAM_DEFAULT_EDITION, format_program_size},
    errors,
    options::BuildOptions,
};

use leo_ast::{NetworkName, NodeBuilder, Program, Stub};
use leo_compiler::{Compiled, Compiler};
use leo_errors::{Handler, Result};
use leo_package::{ABI_FILENAME, MANIFEST_FILENAME, MAX_PROGRAM_SIZE, Package};
use leo_span::Symbol;

use snarkvm::prelude::{CanaryV0, MainnetV0, Process as SvmProcess, Program as SvmProgram, TestnetV0};

use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr as _,
};

/// Network-typed `Process` used during the disassemble loop.
enum DisassembleProcess {
    Mainnet(SvmProcess<MainnetV0>),
    Testnet(SvmProcess<TestnetV0>),
    Canary(SvmProcess<CanaryV0>),
}

/// A program queued for bytecode validation after the build.
struct ProgramForValidation {
    bytecode: String,
    path: PathBuf,
    is_leo_compiled: bool,
}

/// Drive a `leo build`: load (or re-resolve) the project's `Package`,
/// compile every source unit, write bytecode / ABI artifacts to disk,
/// disassemble bytecode deps, and validate the result through snarkVM.
///
/// Returns the loaded `Package` so workspace-mode callers can iterate
/// members and report the last-built one.
pub fn handle_build(
    options: &BuildOptions,
    network: NetworkName,
    endpoint: &str,
    network_retries: u32,
    package_path: &Path,
    home_path: &Path,
) -> Result<Package> {
    let package = if options.build_tests {
        Package::from_directory_with_tests(
            package_path,
            home_path,
            options.no_cache,
            options.no_local,
            Some(network),
            Some(endpoint),
            network_retries,
            crate::package_fetch::fetch_compilation_unit,
        )?
    } else {
        Package::from_directory(
            package_path,
            home_path,
            options.no_cache,
            options.no_local,
            Some(network),
            Some(endpoint),
            network_retries,
            crate::package_fetch::fetch_compilation_unit,
        )?
    };

    // Check the manifest for the compiler version.
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

    let primary_name = package.primary_unit().map(|p| p.name);
    std::fs::create_dir_all(&build_directory).map_err(|err| {
        errors::util_file_io_error(format_args!("Couldn't create directory {}", build_directory.display()), err)
    })?;
    remove_legacy_build_artifacts(&build_directory);

    let handler = Handler::default();
    let node_builder = Rc::new(NodeBuilder::default());

    let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();
    let mut compiled_programs: IndexMap<String, ProgramForValidation> = IndexMap::new();
    let mut written: HashSet<String> = HashSet::new();

    let mut disassemble_process = match network {
        NetworkName::MainnetV0 => DisassembleProcess::Mainnet(SvmProcess::<MainnetV0>::load().map_err(|e| {
            errors::custom(format!("Failed to initialize snarkVM process for disassembler validation: {e}"))
        })?),
        NetworkName::TestnetV0 => DisassembleProcess::Testnet(SvmProcess::<TestnetV0>::load().map_err(|e| {
            errors::custom(format!("Failed to initialize snarkVM process for disassembler validation: {e}"))
        })?),
        NetworkName::CanaryV0 => DisassembleProcess::Canary(SvmProcess::<CanaryV0>::load().map_err(|e| {
            errors::custom(format!("Failed to initialize snarkVM process for disassembler validation: {e}"))
        })?),
    };

    for unit in &package.compilation_units {
        let unit_name = unit.name.to_string();
        let unit_key = leo_package::bare_unit_name(&unit_name).to_string();
        match &unit.data {
            leo_package::ProgramData::Bytecode(bytecode) => {
                let build_path = package.unit_bytecode_path(&unit_name);

                if written.insert(unit_key.clone()) {
                    ensure_parent_dir(&build_path)?;
                    std::fs::write(&build_path, bytecode).map_err(errors::failed_to_load_instructions)?;
                }

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
                let source_dir = if unit.kind.is_test() {
                    source
                        .parent()
                        .ok_or_else(|| {
                            errors::failed_to_open_file(format_args!(
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
                    let snapshots_directory = package.unit_snapshots_directory(&unit_name);
                    let compiled = compile_leo_source_directory(
                        source,
                        &source_dir,
                        unit.name,
                        unit.kind.is_test(),
                        &snapshots_directory,
                        &handler,
                        &node_builder,
                        options.clone(),
                        stubs.clone(),
                        network,
                    )?;

                    let primary_path = package.unit_bytecode_path(&unit_name);
                    if written.insert(unit_key.clone()) {
                        ensure_parent_dir(&primary_path)?;
                        std::fs::write(&primary_path, &compiled.primary.bytecode)
                            .map_err(errors::failed_to_load_instructions)?;
                        if is_main {
                            let abi_path = package.unit_abi_path(&unit_name);
                            let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
                                .map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
                            std::fs::write(&abi_path, abi_json).map_err(errors::failed_to_write_abi)?;
                            tracing::info!("✅ Generated ABI for program '{unit_name}'.");
                            let interfaces_directory = package.unit_interfaces_directory(&unit_name);
                            write_interface_abis(&interfaces_directory, &compiled.interfaces)?;
                        }
                    }

                    for import in &compiled.imports {
                        let import_path = package.unit_bytecode_path(&import.name);
                        let import_key = leo_package::bare_unit_name(&import.name).to_string();
                        if written.insert(import_key.clone()) {
                            ensure_parent_dir(&import_path)?;
                            std::fs::write(&import_path, &import.bytecode)
                                .map_err(errors::failed_to_load_instructions)?;

                            let import_abi_path = package.unit_abi_path(&import.name);
                            let import_abi_json = serde_json::to_string_pretty(&import.abi)
                                .map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
                            std::fs::write(&import_abi_path, import_abi_json).map_err(errors::failed_to_write_abi)?;
                        }

                        compiled_programs.entry(import_key).or_insert(ProgramForValidation {
                            bytecode: import.bytecode.clone(),
                            path: import_path,
                            is_leo_compiled: true,
                        });
                    }
                    compiled_programs.entry(unit_key.clone()).or_insert(ProgramForValidation {
                        bytecode: compiled.primary.bytecode.clone(),
                        path: primary_path,
                        is_leo_compiled: true,
                    });
                }

                if unit.kind.is_library() {
                    let library = if primary_name == Some(unit.name) {
                        let snapshots_directory = package.unit_snapshots_directory(&unit_name);
                        let (lib, interfaces) = build_leo_source_directory_library(
                            source,
                            &source_dir,
                            unit.name,
                            &snapshots_directory,
                            &handler,
                            &node_builder,
                            options.clone(),
                            stubs.clone(),
                            network,
                        )?;

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
                            options.clone(),
                            network,
                        )?
                    };
                    handler.last_err()?;
                    let mut library_stub: Stub = library.into();
                    for node in package.dep_graph.nodes() {
                        if package.dep_graph.neighbors(node).any(|dep| dep == &unit.name) {
                            library_stub.add_parent(*node);
                        }
                    }
                    stubs.insert(unit.name, library_stub);
                } else {
                    let leo_program = parse_leo_source_directory(
                        source,
                        &source_dir,
                        unit.name,
                        &handler,
                        &node_builder,
                        options.clone(),
                        network,
                    )?;

                    stubs.insert(unit.name, leo_program.into());
                }
            }
        }
    }

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
            options.clone(),
            stubs.clone(),
            network,
        )?;
        let primary_path = package.unit_bytecode_path(&unit_name);
        ensure_parent_dir(&primary_path)?;
        std::fs::write(&primary_path, &compiled.primary.bytecode).map_err(errors::failed_to_load_instructions)?;
        let abi_path = package.unit_abi_path(&unit_name);
        let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
            .map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
        std::fs::write(&abi_path, abi_json).map_err(errors::failed_to_write_abi)?;
        let interfaces_directory = package.unit_interfaces_directory(&unit_name);
        write_interface_abis(&interfaces_directory, &compiled.interfaces)?;
        compiled_programs.entry(unit_key).or_insert(ProgramForValidation {
            bytecode: compiled.primary.bytecode.clone(),
            path: primary_path,
            is_leo_compiled: true,
        });
    }

    validate_compiled_programs(&compiled_programs, network)?;

    Ok(package)
}

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
    println!();
    tracing::info!("🔨 Compiling '{program_name}'");
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

    let compiled = compiler.compile_from_directory(entry_file_path, source_directory)?;
    let primary_bytecode = &compiled.primary.bytecode;

    let program_size = primary_bytecode.len();

    if program_size > MAX_PROGRAM_SIZE {
        return Err(errors::program_size_limit_exceeded(program_name, program_size, MAX_PROGRAM_SIZE).into());
    }

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

    for import in &compiled.imports {
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

fn parse_leo_source_directory(
    entry_file_path: &Path,
    source_directory: &Path,
    program_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    network: NetworkName,
) -> Result<Program> {
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
    compiler.parse_program_from_directory(entry_file_path, source_directory)
}

fn validate_compiled_programs(programs: &IndexMap<String, ProgramForValidation>, network: NetworkName) -> Result<()> {
    match network {
        NetworkName::MainnetV0 => validate_compiled_programs_inner::<MainnetV0>(programs),
        NetworkName::TestnetV0 => validate_compiled_programs_inner::<TestnetV0>(programs),
        NetworkName::CanaryV0 => validate_compiled_programs_inner::<CanaryV0>(programs),
    }
}

fn validate_compiled_programs_inner<N: snarkvm::prelude::Network>(
    programs: &IndexMap<String, ProgramForValidation>,
) -> Result<()> {
    let process = SvmProcess::<N>::load()
        .map_err(|e| errors::custom(format!("Failed to initialize snarkVM process for bytecode validation: {e}")))?;

    for (name, ProgramForValidation { bytecode, path, is_leo_compiled }) in programs {
        let program = SvmProgram::<N>::from_str(bytecode).map_err(|e| errors::failed_to_parse_aleo_file(name, e))?;

        let checksum = program.to_checksum().iter().join(", ");

        process.lock().add_program_with_edition(&program, LOCAL_PROGRAM_DEFAULT_EDITION).map_err(|e| {
            if *is_leo_compiled {
                errors::generated_invalid_bytecode(name, path.display(), &checksum, e)
            } else {
                errors::custom(format!("snarkVM rejected external program '{name}' during build validation: {e}"))
            }
        })?;
    }

    Ok(())
}

fn parse_leo_source_directory_library(
    entry_file_path: &Path,
    source_directory: &Path,
    library_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    network: NetworkName,
) -> Result<leo_ast::Library> {
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
    compiler.parse_library_from_directory(library_name, entry_file_path, source_directory)
}

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

fn write_interface_abis(interfaces_dir: &Path, interfaces: &[leo_abi::interfaces::CompiledInterface]) -> Result<()> {
    if interfaces_dir.exists() {
        std::fs::remove_dir_all(interfaces_dir).map_err(errors::failed_to_write_abi)?;
    }
    if interfaces.is_empty() {
        return Ok(());
    }
    for ci in interfaces {
        let mut file_path = match &ci.owner {
            leo_abi::interfaces::InterfaceOwner::Local => interfaces_dir.to_path_buf(),
            leo_abi::interfaces::InterfaceOwner::External { owner_program } => interfaces_dir.join(owner_program),
        };
        for seg in &ci.abi.path[..ci.abi.path.len().saturating_sub(1)] {
            file_path.push(seg);
        }
        std::fs::create_dir_all(&file_path).map_err(errors::failed_to_write_abi)?;
        file_path.push(format!("{}.json", ci.abi.name));
        let json = serde_json::to_string_pretty(&ci.abi).map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
        std::fs::write(&file_path, json).map_err(errors::failed_to_write_abi)?;
    }
    tracing::info!("✅ Generated {} interface ABI(s).", interfaces.len());
    Ok(())
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            errors::util_file_io_error(format_args!("Couldn't create directory {}", parent.display()), err)
        })?;
    }
    Ok(())
}

fn remove_legacy_build_artifacts(build_directory: &Path) {
    let is_legacy = build_directory.join("main.aleo").exists() || build_directory.join(MANIFEST_FILENAME).exists();
    if !is_legacy {
        return;
    }
    for file in ["main.aleo", ABI_FILENAME, MANIFEST_FILENAME] {
        let _ = std::fs::remove_file(build_directory.join(file));
    }
    for dir in ["imports", "interfaces"] {
        let _ = std::fs::remove_dir_all(build_directory.join(dir));
    }
}
