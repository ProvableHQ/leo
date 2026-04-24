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

//! Benchmark fixtures and compiler helpers for Criterion-based frontend benchmarks.
//!
//! Run benchmarks with `cargo bench -p leo-benchmarks --bench compiler_benchmarks`.

#![forbid(unsafe_code)]

use leo_ast::{AleoProgram, FunctionStub, Identifier, NetworkName, NodeBuilder, ProgramId, Stub};
use leo_compiler::{Compiler, CompilerOptions};
use leo_errors::Handler;
use leo_package::{Package, ProgramData};
use leo_parser::parse_program;
use leo_span::{Symbol, source_map::FileName, with_session_globals};
use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};

use std::{
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use indexmap::IndexMap;
use walkdir::WalkDir;

pub const BENCH_NETWORK: NetworkName = NetworkName::TestnetV0;

/// Loaded benchmark fixture ready for compiler construction.
pub struct FixtureData {
    pub program_name: String,
    pub source: String,
    pub filename: FileName,
    pub modules: Vec<(String, FileName)>,
    pub import_stubs: IndexMap<Symbol, Stub>,
    pub network: NetworkName,
}

impl FixtureData {
    pub fn module_refs(&self) -> Vec<(&str, FileName)> {
        self.modules.iter().map(|(source, name)| (source.as_str(), name.clone())).collect()
    }
}

fn read_modules(source_dir: &Path, entry: &Path) -> Result<Vec<(String, FileName)>, String> {
    let mut modules = Vec::new();

    for file in WalkDir::new(source_dir).into_iter().filter_map(Result::ok) {
        if !file.file_type().is_file()
            || file.path() == entry
            || file.path().extension().and_then(|ext| ext.to_str()) != Some("leo")
        {
            continue;
        }

        let source = match fs::read_to_string(file.path()) {
            Ok(source) => source,
            Err(err) => {
                return Err(format!("failed to read fixture module {}: {err}", file.path().display()));
            }
        };
        modules.push((source, FileName::Real(file.path().to_path_buf())));
    }

    Ok(modules)
}

/// Parses a local Leo dependency and extracts its public interface as a
/// `Stub::FromAleo`.
///
/// Only the program scope's entry-point function signatures, struct definitions,
/// and mappings are kept.  This avoids the scope-ordering issues that occur when
/// `Stub::FromLeo` (which wraps a full `Program` AST) is processed by
/// reconstructor-based passes.
fn parse_interface_stub(program_name: Symbol, source: &Path, source_dir: &Path) -> Result<Stub, String> {
    let source_text = match fs::read_to_string(source) {
        Ok(source_text) => source_text,
        Err(err) => {
            return Err(format!(
                "failed to read dependency {}.aleo source file {}: {err}",
                program_name,
                source.display()
            ));
        }
    };
    let modules = read_modules(source_dir, source)?;
    let source_file =
        with_session_globals(|s| s.source_map.new_source(&source_text, FileName::Real(source.to_path_buf())));
    let module_source_files = modules
        .iter()
        .map(|(source, filename)| with_session_globals(|s| s.source_map.new_source(source, filename.clone())))
        .collect::<Vec<_>>();
    let node_builder = NodeBuilder::default();

    let program =
        match parse_program(Handler::default(), &node_builder, &source_file, &module_source_files, BENCH_NETWORK) {
            Ok(ast) => ast,
            Err(err) => {
                return Err(format!(
                    "failed to parse dependency {}.aleo interface source {}: {err}",
                    program_name,
                    source.display()
                ));
            }
        };

    // Extract the single program scope.
    let scope = match program.program_scopes.values().next() {
        Some(scope) => scope,
        None => {
            return Err(format!("no program scope found for dependency {}.aleo in {}", program_name, source.display()));
        }
    };

    // Build function stubs from entry-point signatures (no bodies).
    let functions = scope
        .functions
        .iter()
        .map(|(sym, func)| {
            (*sym, FunctionStub {
                annotations: func.annotations.clone(),
                variant: func.variant,
                identifier: func.identifier,
                input: func.input.clone(),
                output: func.output.clone(),
                output_type: func.output_type.clone(),
                span: func.span,
                id: func.id,
            })
        })
        .collect();

    let imports = program
        .imports
        .keys()
        .map(|sym| {
            // program.imports keys are full ProgramId symbols (e.g. "bench_policy.aleo").
            // ProgramId.name must be just the bare name; the network suffix is stored separately.
            let sym_str = sym.to_string();
            let name_only = sym_str.strip_suffix(".aleo").unwrap_or(&sym_str);
            ProgramId {
                name: Identifier { name: Symbol::intern(name_only), span: Default::default(), id: Default::default() },
                network: Identifier { name: Symbol::intern("aleo"), span: Default::default(), id: Default::default() },
            }
        })
        .collect();

    let aleo_program = AleoProgram {
        imports,
        stub_id: scope.program_id,
        consts: scope.consts.clone(),
        composites: scope.composites.clone(),
        mappings: scope.mappings.clone(),
        functions,
        span: scope.span,
    };

    Ok(aleo_program.into())
}

fn disassemble_bytecode(program_name: Symbol, bytecode: &str) -> Result<Stub, String> {
    let disassembled = match BENCH_NETWORK {
        NetworkName::MainnetV0 => leo_disassembler::disassemble_from_str::<MainnetV0>(program_name, bytecode),
        NetworkName::TestnetV0 => leo_disassembler::disassemble_from_str::<TestnetV0>(program_name, bytecode),
        NetworkName::CanaryV0 => leo_disassembler::disassemble_from_str::<CanaryV0>(program_name, bytecode),
    };

    match disassembled {
        Ok(stub) => Ok(stub.into()),
        Err(err) => Err(format!("failed to disassemble dependency {}.aleo bytecode: {err}", program_name)),
    }
}

/// Loads source/modules and resolves transitive dependencies with `Package::from_directory`
/// (same flow as CLI `leo build`).
///
/// Local Leo dependencies are parsed and their public interfaces extracted into
/// `Stub::FromAleo` stubs to avoid scope-hierarchy conflicts with the main
/// program's compiler passes.
///
pub fn load_fixture(package_dir: &Path) -> Result<FixtureData, String> {
    let home = aleo_std::aleo_dir();

    let package = match Package::from_directory(
        package_dir,
        &home,
        false, // no_cache
        false, // no_local
        Some(BENCH_NETWORK),
        None, // no endpoint needed for local-only fixtures
        2,
    ) {
        Ok(package) => package,
        Err(err) => {
            return Err(format!(
                "failed to resolve fixture dependencies for {} (network unavailable?): {err}",
                package_dir.display()
            ));
        }
    };

    let mut import_stubs = IndexMap::new();

    for unit in &package.compilation_units {
        match &unit.data {
            ProgramData::Bytecode(bytecode) => {
                let stub = disassemble_bytecode(unit.name, bytecode)?;
                import_stubs.insert(unit.name, stub);
            }
            ProgramData::SourcePath { directory, source } => {
                let source_dir =
                    if source.starts_with(directory.join("src")) { directory.join("src") } else { directory.clone() };

                let stub = parse_interface_stub(unit.name, source, &source_dir)?;
                import_stubs.insert(unit.name, stub);
            }
        }
    }

    let entry = package_dir.join("src/main.leo");
    let source = match fs::read_to_string(&entry) {
        Ok(source) => source,
        Err(err) => {
            return Err(format!("failed to read fixture source {}: {err}", entry.display()));
        }
    };
    let modules = read_modules(&package_dir.join("src"), &entry)?;
    let program_name = package.manifest.program.clone();

    Ok(FixtureData {
        program_name,
        source,
        filename: FileName::Real(entry),
        modules,
        import_stubs,
        network: BENCH_NETWORK,
    })
}

/// Loads a standalone `.leo` source file as a fixture with no dependencies.
pub fn load_source_fixture(source_path: &Path) -> Result<FixtureData, String> {
    let source = match fs::read_to_string(source_path) {
        Ok(source) => source,
        Err(err) => {
            return Err(format!("failed to read source fixture {}: {err}", source_path.display()));
        }
    };
    Ok(FixtureData {
        program_name: String::new(),
        source,
        filename: FileName::Real(source_path.to_path_buf()),
        modules: Vec::new(),
        import_stubs: IndexMap::new(),
        network: BENCH_NETWORK,
    })
}

/// Creates a fresh [`Compiler`] with all fixture dependencies pre-loaded.
pub fn create_compiler(fixture: &FixtureData) -> Compiler {
    let expected_unit_name = if fixture.program_name.is_empty() { None } else { Some(fixture.program_name.clone()) };

    Compiler::new(
        expected_unit_name,
        false,
        Handler::default(),
        Rc::new(NodeBuilder::default()),
        PathBuf::default(),
        Some(CompilerOptions::default()),
        fixture.import_stubs.clone(),
        fixture.network,
    )
}

/// Creates a lightweight [`Compiler`] with no import stubs for parse-only benchmarks.
pub fn create_parse_only_compiler(fixture: &FixtureData) -> Compiler {
    let expected_unit_name = if fixture.program_name.is_empty() { None } else { Some(fixture.program_name.clone()) };

    Compiler::new(
        expected_unit_name,
        false,
        Handler::default(),
        Rc::new(NodeBuilder::default()),
        PathBuf::default(),
        Some(CompilerOptions::default()),
        IndexMap::new(),
        fixture.network,
    )
}
