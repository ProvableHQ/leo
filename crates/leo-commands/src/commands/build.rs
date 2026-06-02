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

//! `leo build` core. Both the native CLI's `LeoBuild` clap struct and
//! `leo-wasm`'s `build_impl` thin-wrap [`handle_build`] — the only
//! per-target difference is the `ArtifactSink` they hand in (disk vs
//! in-memory) and whether they ask for the post-build snarkVM validation
//! (native-only).

use crate::{errors, options::BuildOptions};

use leo_ast::{NetworkName, NodeBuilder, Program, Stub};
use leo_compiler::{Compiled, Compiler, FileSource};
#[cfg(target_arch = "wasm32")]
use leo_disassembler::disassemble_from_str_for_network;
use leo_errors::{Handler, Result};
#[cfg_attr(target_arch = "wasm32", allow(unused_imports))]
use leo_package::{
    ABI_FILENAME,
    LOCAL_PROGRAM_DEFAULT_EDITION,
    MANIFEST_FILENAME,
    Package,
    format_program_size,
    max_program_size,
};
use leo_span::Symbol;

use indexmap::IndexMap;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    rc::Rc,
};

/// Sink for the bytecode / ABI / interface artifacts a build emits.
///
/// The CLI passes a [`DiskSink`] that writes through to the real
/// filesystem; the `leo-wasm` bindings pass a [`MemorySink`] that
/// collects writes in memory, which the JSON shim then surfaces to JS.
pub trait ArtifactSink {
    /// Write `contents` to `path`. Parent directories are created on demand —
    /// callers don't need to pre-create them.
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()>;
    fn remove_file(&self, path: &Path) -> Result<()>;
    fn remove_dir_all(&self, path: &Path) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
}

/// `ArtifactSink` that writes through to the real filesystem.
pub struct DiskSink;

impl ArtifactSink for DiskSink {
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|err| -> leo_errors::LeoError {
                errors::util_file_io_error(format_args!("Couldn't create directory {}", parent.display()), err).into()
            })?;
        }
        std::fs::write(path, contents).map_err(|err| errors::failed_to_load_instructions(err).into())
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        // Best-effort — missing entries are ignored (matches legacy cleanup behavior).
        let _ = std::fs::remove_file(path);
        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        let _ = std::fs::remove_dir_all(path);
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

/// `ArtifactSink` that collects writes in memory. Used by the wasm bindings.
///
/// `RefCell` (not `Mutex`) — `handle_build` is single-threaded and the sink
/// never crosses threads. A future caller that wants to share the sink across
/// threads will get a clear `!Send`/`!Sync` compile error here.
#[derive(Default)]
pub struct MemorySink {
    files: std::cell::RefCell<IndexMap<PathBuf, Vec<u8>>>,
}

impl MemorySink {
    pub fn new() -> Self {
        Self::default()
    }

    /// Take the collected writes, draining the sink.
    pub fn into_files(self) -> IndexMap<PathBuf, Vec<u8>> {
        self.files.into_inner()
    }
}

impl ArtifactSink for MemorySink {
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()> {
        self.files.borrow_mut().insert(path.to_path_buf(), contents.to_vec());
        Ok(())
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        self.files.borrow_mut().shift_remove(path);
        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        // `starts_with` is component-wise: a file named exactly `path` matches
        // (and is removed alongside everything strictly below it), while a
        // sibling like `path.aleo` does not.
        self.files.borrow_mut().retain(|p, _| !p.starts_with(path));
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        // `Path::starts_with` is reflexive, so a stored path equal to `path`
        // (file existence) is covered by the same check that detects a path
        // strictly below `path` (directory existence).
        self.files.borrow().keys().any(|p| p.starts_with(path))
    }
}

/// A program queued for post-build snarkVM validation.
///
/// `wasm32` builds skip validation entirely; `cfg_attr` keeps the field
/// reads dead-code-clean on that target.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
struct ProgramForValidation {
    bytecode: String,
    path: PathBuf,
    is_leo_compiled: bool,
}

/// Network-typed `Process` reused across the bytecode-dep disassembly loop so
/// snarkVM's contextual `add_program` sees each dependency's imports already
/// loaded (deps are walked in topological order). A fresh `Process` per call
/// rejects any program with imports — see `disassemble_from_str_for_network`'s
/// doc comment in `crates/disassembler/src/lib.rs`.
#[cfg(not(target_arch = "wasm32"))]
enum DisassembleProcess {
    Mainnet(snarkvm::prelude::Process<snarkvm::prelude::MainnetV0>),
    Testnet(snarkvm::prelude::Process<snarkvm::prelude::TestnetV0>),
    Canary(snarkvm::prelude::Process<snarkvm::prelude::CanaryV0>),
}

#[cfg(not(target_arch = "wasm32"))]
fn init_disassemble_process(network: NetworkName) -> Result<DisassembleProcess> {
    use snarkvm::prelude::{CanaryV0, MainnetV0, Process as SvmProcess, TestnetV0};
    fn init_err<E: std::fmt::Display>(e: E) -> leo_errors::LeoError {
        errors::custom(format!("Failed to initialize snarkVM process for disassembler validation: {e}")).into()
    }
    Ok(match network {
        NetworkName::MainnetV0 => DisassembleProcess::Mainnet(SvmProcess::<MainnetV0>::load().map_err(init_err)?),
        NetworkName::TestnetV0 => DisassembleProcess::Testnet(SvmProcess::<TestnetV0>::load().map_err(init_err)?),
        NetworkName::CanaryV0 => DisassembleProcess::Canary(SvmProcess::<CanaryV0>::load().map_err(init_err)?),
    })
}

/// Drive a `leo build`: load (or re-resolve) the project's `Package`,
/// compile every source unit, write bytecode + ABI artifacts via `sink`,
/// disassemble bytecode deps, and validate the result through snarkVM.
///
/// On native, callers pass a `DiskFileSource` + `DiskSink`. On wasm, callers
/// pass an `InMemoryFileSource` + `MemorySink`; the snarkVM post-build
/// validation step is automatically skipped there (no `Process::load`
/// source on `wasm32`).
///
/// Returns the loaded `Package` so workspace-mode callers can report the
/// last-built member.
///
/// `home_path` / `endpoint` / `network_retries` only matter for the native
/// loader path (registry cache + HTTP fetcher). Wasm callers pass `None` /
/// `None` / `0`.
#[allow(clippy::too_many_arguments)]
pub fn handle_build(
    options: &BuildOptions,
    network: NetworkName,
    package_path: &Path,
    file_source: &dyn FileSource,
    sink: &dyn ArtifactSink,
    home_path: Option<&Path>,
    endpoint: Option<&str>,
    network_retries: u32,
) -> Result<Package> {
    let package = load_package(options, network, package_path, file_source, home_path, endpoint, network_retries)?;

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
    remove_legacy_build_artifacts(sink, &build_directory);

    let handler = Handler::default();
    let node_builder = Rc::new(NodeBuilder::default());

    let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();
    let mut compiled_programs: IndexMap<String, ProgramForValidation> = IndexMap::new();
    let mut written: HashSet<String> = HashSet::new();

    // Hoist a typed `Process` for the active network so bytecode dependencies are
    // validated through `Process::add_program` *before* `disassemble` runs on them
    // (issue #29399 — `from_function_core` panics on shapes the grammar accepts).
    // `add_program` is contextual, so the same process must be reused across the
    // loop in topological order.
    #[cfg(not(target_arch = "wasm32"))]
    let mut disassemble_process = init_disassemble_process(network)?;

    for unit in &package.compilation_units {
        let unit_name = unit.name.to_string();
        let unit_key = leo_package::bare_unit_name(&unit_name).to_string();
        match &unit.data {
            leo_package::ProgramData::Bytecode(bytecode) => {
                let build_path = package.unit_bytecode_path(&unit_name);
                if written.insert(unit_key.clone()) {
                    sink.write(&build_path, bytecode.as_bytes())?;
                }
                // Native: validate via the shared topological process; wasm: parse-only.
                #[cfg(not(target_arch = "wasm32"))]
                let stub = match &mut disassemble_process {
                    DisassembleProcess::Mainnet(p) => {
                        leo_disassembler::disassemble_from_str::<snarkvm::prelude::MainnetV0>(unit.name, bytecode, p)
                    }
                    DisassembleProcess::Testnet(p) => {
                        leo_disassembler::disassemble_from_str::<snarkvm::prelude::TestnetV0>(unit.name, bytecode, p)
                    }
                    DisassembleProcess::Canary(p) => {
                        leo_disassembler::disassemble_from_str::<snarkvm::prelude::CanaryV0>(unit.name, bytecode, p)
                    }
                }?;
                #[cfg(target_arch = "wasm32")]
                let stub = disassemble_from_str_for_network(unit.name, bytecode, network)?;
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
                        file_source,
                    )?;

                    let primary_path = package.unit_bytecode_path(&unit_name);
                    if written.insert(unit_key.clone()) {
                        sink.write(&primary_path, compiled.primary.bytecode.as_bytes())?;
                        if is_main {
                            let abi_path = package.unit_abi_path(&unit_name);
                            let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
                                .map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
                            sink.write(&abi_path, abi_json.as_bytes())?;
                            tracing::info!("✅ Generated ABI for program '{unit_name}'.");
                            // Refresh interface ABIs for the main program.
                            let interfaces_directory = package.unit_interfaces_directory(&unit_name);
                            write_interface_abis(sink, &interfaces_directory, &compiled.interfaces)?;
                        }
                    }

                    for import in &compiled.imports {
                        let import_path = package.unit_bytecode_path(&import.name);
                        let import_key = leo_package::bare_unit_name(&import.name).to_string();
                        if written.insert(import_key.clone()) {
                            sink.write(&import_path, import.bytecode.as_bytes())?;
                            let import_abi_path = package.unit_abi_path(&import.name);
                            let import_abi_json = serde_json::to_string_pretty(&import.abi)
                                .map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
                            sink.write(&import_abi_path, import_abi_json.as_bytes())?;
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
                            file_source,
                        )?;
                        // Interface ABIs are only emitted for the primary library, where the full
                        // frontend has resolved every name and type.
                        let interfaces_directory = package.unit_interfaces_directory(&unit_name);
                        write_interface_abis(sink, &interfaces_directory, &interfaces)?;
                        tracing::info!("✅ Validated '{unit_name}'.");
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
                            file_source,
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
                        file_source,
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
            file_source,
        )?;
        let primary_path = package.unit_bytecode_path(&unit_name);
        sink.write(&primary_path, compiled.primary.bytecode.as_bytes())?;
        let abi_path = package.unit_abi_path(&unit_name);
        let abi_json = serde_json::to_string_pretty(&compiled.primary.abi)
            .map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
        sink.write(&abi_path, abi_json.as_bytes())?;
        let interfaces_directory = package.unit_interfaces_directory(&unit_name);
        write_interface_abis(sink, &interfaces_directory, &compiled.interfaces)?;
        compiled_programs.entry(unit_key).or_insert(ProgramForValidation {
            bytecode: compiled.primary.bytecode.clone(),
            path: primary_path,
            is_leo_compiled: true,
        });
    }

    // Post-build validation is native-only; on wasm we don't have `Process::load`.
    #[cfg(not(target_arch = "wasm32"))]
    validate_compiled_programs(&compiled_programs, network)?;
    #[cfg(target_arch = "wasm32")]
    let _ = (&compiled_programs, network);

    Ok(package)
}

/// Load a `Package` through the unified file_source-aware loader. The native
/// CLI passes a `DiskFileSource` + `Some(home_path)` + `Some(endpoint)`; the
/// wasm shim passes an `InMemoryFileSource` + `None` + `None` + `0`.
#[allow(clippy::too_many_arguments)]
fn load_package(
    options: &BuildOptions,
    network: NetworkName,
    package_path: &Path,
    file_source: &dyn FileSource,
    home_path: Option<&Path>,
    endpoint: Option<&str>,
    network_retries: u32,
) -> Result<Package> {
    if options.build_tests {
        Package::from_directory_with_tests_with_file_source(
            package_path,
            file_source,
            network,
            home_path,
            endpoint,
            network_retries,
            options.no_cache,
            options.no_local,
        )
    } else {
        Package::from_directory_with_file_source(
            package_path,
            file_source,
            network,
            home_path,
            endpoint,
            network_retries,
            options.no_cache,
            options.no_local,
        )
    }
}

/// Shared `Compiler::new` callsite. The four helpers below differ only in what
/// they do with the compiler instance after construction.
#[allow(clippy::too_many_arguments)]
fn make_compiler(
    name: Symbol,
    is_test: bool,
    output_path: PathBuf,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
    network: NetworkName,
) -> Compiler {
    Compiler::new(
        Some(name.to_string()),
        is_test,
        handler.clone(),
        Rc::clone(node_builder),
        output_path,
        Some(options.into()),
        stubs,
        network,
    )
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
    file_source: &dyn FileSource,
) -> Result<Compiled> {
    tracing::info!("🔨 Compiling '{program_name}'");
    let mut compiler =
        make_compiler(program_name, is_test, output_path.to_path_buf(), handler, node_builder, options, stubs, network);

    let compiled = compiler.compile_from_directory_with_file_source(entry_file_path, source_directory, file_source)?;
    let program_size = compiled.primary.bytecode.len();
    let max_size = max_program_size(network);
    if program_size > max_size {
        return Err(errors::program_size_limit_exceeded(program_name, program_size, max_size).into());
    }

    tracing::info!("    {} statements before dead code elimination.", compiler.statements_before_dce);
    tracing::info!("    {} statements after dead code elimination.", compiler.statements_after_dce);

    // Compute per-program checksum and log it; native-only since wasm doesn't ship the snarkVM umbrella.
    #[cfg(not(target_arch = "wasm32"))]
    log_bytecode_checksum(program_name, &compiled.primary.bytecode, network)?;
    #[cfg(not(target_arch = "wasm32"))]
    for import in &compiled.imports {
        let import_name: &str = &import.name;
        log_bytecode_checksum(import_name, &import.bytecode, network)?;
    }

    let (size_kb, max_kb, warning) = format_program_size(program_size, max_size);
    tracing::info!("    Program size: {size_kb:.2} KB / {max_kb:.2} KB");
    if let Some(msg) = warning {
        tracing::warn!("⚠️  Program '{program_name}' is {msg}.");
    }

    tracing::info!("✅ Compiled '{program_name}' into Aleo instructions.");
    Ok(compiled)
}

#[cfg(not(target_arch = "wasm32"))]
fn log_bytecode_checksum(name: impl std::fmt::Display, bytecode: &str, network: NetworkName) -> Result<()> {
    use itertools::Itertools as _;
    use snarkvm::prelude::{CanaryV0, MainnetV0, Program as SvmProgram, TestnetV0};
    use std::str::FromStr as _;
    fn checksum<N: snarkvm::prelude::Network>(name: &impl std::fmt::Display, bytecode: &str) -> Result<String> {
        let program = SvmProgram::<N>::from_str(bytecode).map_err(|e| errors::failed_to_parse_aleo_file(name, e))?;
        Ok(program.to_checksum().iter().join(", "))
    }
    let checksum = match network {
        NetworkName::MainnetV0 => checksum::<MainnetV0>(&name, bytecode)?,
        NetworkName::TestnetV0 => checksum::<TestnetV0>(&name, bytecode)?,
        NetworkName::CanaryV0 => checksum::<CanaryV0>(&name, bytecode)?,
    };
    tracing::info!("    '{name}' checksum: '[{checksum}]'");
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn parse_leo_source_directory(
    entry_file_path: &Path,
    source_directory: &Path,
    program_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    network: NetworkName,
    file_source: &dyn FileSource,
) -> Result<Program> {
    let mut compiler = make_compiler(
        program_name,
        false,
        PathBuf::default(),
        handler,
        node_builder,
        options,
        IndexMap::new(),
        network,
    );
    compiler.parse_program_from_directory_with_file_source(entry_file_path, source_directory, file_source)
}

#[allow(clippy::too_many_arguments)]
fn parse_leo_source_directory_library(
    entry_file_path: &Path,
    source_directory: &Path,
    library_name: Symbol,
    handler: &Handler,
    node_builder: &Rc<NodeBuilder>,
    options: BuildOptions,
    network: NetworkName,
    file_source: &dyn FileSource,
) -> Result<leo_ast::Library> {
    let mut compiler = make_compiler(
        library_name,
        false,
        PathBuf::default(),
        handler,
        node_builder,
        options,
        IndexMap::new(),
        network,
    );
    compiler.parse_library_from_directory_with_file_source(library_name, entry_file_path, source_directory, file_source)
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
    file_source: &dyn FileSource,
) -> Result<(leo_ast::Library, Vec<leo_abi::interfaces::CompiledInterface>)> {
    tracing::info!("🔨 Building library '{library_name}'");
    let mut compiler = make_compiler(
        library_name,
        false,
        snapshots_directory.to_path_buf(),
        handler,
        node_builder,
        options,
        stubs,
        network,
    );
    let library = compiler.build_library_from_directory_with_file_source(
        library_name,
        entry_file_path,
        source_directory,
        file_source,
    )?;
    let interfaces = compiler.generate_library_interface_abis();
    Ok((library, interfaces))
}

/// Write a unit's interface ABIs under `interfaces_directory`, pre-cleaning the
/// directory so renamed/deleted interfaces don't linger across builds.
fn write_interface_abis(
    sink: &dyn ArtifactSink,
    interfaces_dir: &Path,
    interfaces: &[leo_abi::interfaces::CompiledInterface],
) -> Result<()> {
    // Pre-clean stale files from a previous build (renamed/deleted interfaces).
    if sink.exists(interfaces_dir) {
        sink.remove_dir_all(interfaces_dir)?;
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
        file_path.push(format!("{}.json", ci.abi.name));
        let json = serde_json::to_string_pretty(&ci.abi).map_err(|e| errors::failed_to_serialize_abi(e.to_string()))?;
        sink.write(&file_path, json.as_bytes())?;
    }
    tracing::info!("✅ Generated {} interface ABI(s).", interfaces.len());
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_compiled_programs(programs: &IndexMap<String, ProgramForValidation>, network: NetworkName) -> Result<()> {
    use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};
    match network {
        NetworkName::MainnetV0 => validate_compiled_programs_inner::<MainnetV0>(programs),
        NetworkName::TestnetV0 => validate_compiled_programs_inner::<TestnetV0>(programs),
        NetworkName::CanaryV0 => validate_compiled_programs_inner::<CanaryV0>(programs),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_compiled_programs_inner<N: snarkvm::prelude::Network>(
    programs: &IndexMap<String, ProgramForValidation>,
) -> Result<()> {
    use itertools::Itertools as _;
    use snarkvm::prelude::{Process as SvmProcess, Program as SvmProgram};
    use std::str::FromStr as _;

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

fn remove_legacy_build_artifacts(sink: &dyn ArtifactSink, build_directory: &Path) {
    let is_legacy =
        sink.exists(&build_directory.join("main.aleo")) || sink.exists(&build_directory.join(MANIFEST_FILENAME));
    if !is_legacy {
        return;
    }
    for file in ["main.aleo", ABI_FILENAME, MANIFEST_FILENAME] {
        let _ = sink.remove_file(&build_directory.join(file));
    }
    for dir in ["imports", "interfaces"] {
        let _ = sink.remove_dir_all(&build_directory.join(dir));
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use leo_span::{create_session_if_not_set_then, file_source::InMemoryFileSource};

    const MANIFEST: &str = r#"{
"program": "smoke.aleo",
"version": "0.1.0",
"description": "",
"license": "MIT",
"dependencies": null,
"dev_dependencies": null
}"#;

    const MAIN_LEO: &str = r#"program smoke.aleo {
    @noupgrade
    constructor() {}

    fn main(public a: u32, b: u32) -> u32 {
        return a + b;
    }
}
"#;

    /// End-to-end smoke test of the wasm-shaped call path on native: an
    /// `InMemoryFileSource` + `MemorySink` driving `handle_build` to produce
    /// real bytecode and an ABI. Catches regressions in the file_source-aware
    /// loader chain (`Package` → `Compiler` → `CodeGenerating`) without
    /// requiring a wasm runtime to test.
    #[test]
    fn handle_build_in_memory_produces_bytecode_and_abi() {
        let mut fs = InMemoryFileSource::new();
        fs.set(PathBuf::from("/project/program.json"), MANIFEST.to_string());
        fs.set(PathBuf::from("/project/src/main.leo"), MAIN_LEO.to_string());

        create_session_if_not_set_then(|_| {
            let sink = MemorySink::new();
            let package = handle_build(
                &BuildOptions::default(),
                NetworkName::TestnetV0,
                Path::new("/project"),
                &fs,
                &sink,
                None,
                None,
                0,
            )
            .expect("handle_build should succeed on a trivial valid program");

            // Primary unit lookup must resolve to `smoke` regardless of `.aleo` suffix handling.
            let primary = package.primary_unit().expect("package should have a primary unit");
            assert_eq!(primary.name.to_string(), "smoke.aleo");

            let files = sink.into_files();
            let bytecode_path = package.unit_bytecode_path(&primary.name.to_string());
            let abi_path = package.unit_abi_path(&primary.name.to_string());

            let bytecode = std::str::from_utf8(files.get(&bytecode_path).expect("bytecode should be written")).unwrap();
            assert!(bytecode.contains("program smoke.aleo;"), "unexpected bytecode header: {bytecode}");
            assert!(bytecode.contains("function main:"), "missing main function in bytecode");

            let abi_bytes = files.get(&abi_path).expect("ABI should be written");
            // ABI is JSON; just confirm it parses and is non-empty.
            let abi: serde_json::Value = serde_json::from_slice(abi_bytes).expect("ABI must be valid JSON");
            assert!(abi.is_object(), "ABI should serialize as an object");
        });
    }

    /// Workspace glob expansion + member resolution must work end-to-end on an
    /// `InMemoryFileSource`, proving Branch L's file_source-threaded workspace
    /// loader matches CLI behavior. Uses a glob pattern (`members/*`) so the
    /// new `list_files_recursive` + `glob::Pattern` path is exercised.
    #[test]
    fn workspace_resolution_threads_through_file_source() {
        use leo_package::Package;

        // Workspace with one explicit member and one glob-matched member; the
        // primary `consumer` declares a workspace dep on `helper`.
        let workspace_manifest = r#"{"members": ["consumer", "members/*"]}"#;

        let consumer_manifest = r#"{
"program": "consumer.aleo",
"version": "0.1.0",
"description": "",
"license": "MIT",
"dependencies": [{"name": "helper", "location": "workspace", "path": null, "edition": null}],
"dev_dependencies": null
}"#;

        let helper_manifest = r#"{
"program": "helper",
"version": "0.1.0",
"description": "",
"license": "MIT",
"dependencies": null,
"dev_dependencies": null
}"#;

        let mut fs = InMemoryFileSource::new();
        fs.set(PathBuf::from("/ws/workspace.json"), workspace_manifest.into());
        fs.set(PathBuf::from("/ws/consumer/program.json"), consumer_manifest.into());
        fs.set(PathBuf::from("/ws/consumer/src/main.leo"), MAIN_LEO.replace("smoke", "consumer"));
        fs.set(PathBuf::from("/ws/members/helper/program.json"), helper_manifest.into());
        fs.set(PathBuf::from("/ws/members/helper/src/lib.leo"), "fn shared() -> u32 { return 7u32; }\n".into());

        create_session_if_not_set_then(|_| {
            // Load the package directly to verify workspace dep resolution went through
            // the in-memory file source. (A full `handle_build` would additionally exercise
            // cross-member compilation, which is a separate Leo-syntax concern.)
            let package = Package::from_directory_with_file_source(
                Path::new("/ws/consumer"),
                &fs,
                NetworkName::TestnetV0,
                None,
                None,
                0,
                false,
                false,
            )
            .expect("workspace dep should resolve through the in-memory file source");

            let names: Vec<String> = package.compilation_units.iter().map(|u| u.name.to_string()).collect();
            assert!(
                names.iter().any(|n| n == "helper" || n.starts_with("helper")),
                "glob-matched workspace member `helper` should appear: {names:?}",
            );
            assert!(names.iter().any(|n| n.starts_with("consumer")), "primary `consumer` should appear: {names:?}",);
        });
    }

    /// A build that violates the program-size limit must surface as a build error,
    /// not panic, when the `validate` step runs.
    #[test]
    fn handle_build_rejects_unknown_dependency() {
        let manifest_with_bad_dep = r#"{
"program": "smoke.aleo",
"version": "0.1.0",
"description": "",
"license": "MIT",
"dependencies": [{"name": "missing.aleo", "location": "network", "path": null, "edition": null}],
"dev_dependencies": null
}"#;
        let mut fs = InMemoryFileSource::new();
        fs.set(PathBuf::from("/project/program.json"), manifest_with_bad_dep.to_string());
        fs.set(PathBuf::from("/project/src/main.leo"), MAIN_LEO.to_string());

        create_session_if_not_set_then(|_| {
            let sink = MemorySink::new();
            // No endpoint + a network dep → must fail-closed, not panic.
            let result = handle_build(
                &BuildOptions::default(),
                NetworkName::TestnetV0,
                Path::new("/project"),
                &fs,
                &sink,
                None,
                None,
                0,
            );
            assert!(result.is_err(), "expected a build error for an unfetchable network dep");
        });
    }
}
