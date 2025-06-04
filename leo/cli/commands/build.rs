// Copyright (C) 2019-2025 Provable Inc.
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

use leo_ast::Stub;
use leo_compiler::{AstSnapshots, Compiler, CompilerOptions};
use leo_errors::{CliError, UtilError};
use leo_package::{Manifest, NetworkName, Package};
use leo_span::Symbol;

use snarkvm::prelude::{MainnetV0, Network, TestnetV0};

use indexmap::IndexMap;
use snarkvm::prelude::CanaryV0;
use std::path::Path;

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
        // Parse the network.
        let network: NetworkName = context.get_network(&self.env_override.network)?.parse()?;
        match network {
            NetworkName::MainnetV0 => handle_build::<MainnetV0>(&self, context),
            NetworkName::TestnetV0 => handle_build::<TestnetV0>(&self, context),
            NetworkName::CanaryV0 => handle_build::<CanaryV0>(&self, context),
        }
    }
}

// A helper function to handle the build command.
fn handle_build<N: Network>(command: &LeoBuild, context: Context) -> Result<<LeoBuild as Command>::Output> {
    let package_path = context.dir()?;
    let home_path = context.home()?;

    let package = if command.options.build_tests {
        leo_package::Package::from_directory_with_tests(&package_path, &home_path, command.options.no_cache)?
    } else {
        leo_package::Package::from_directory(&package_path, &home_path, command.options.no_cache)?
    };

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

    let mut stubs: IndexMap<Symbol, Stub> = IndexMap::new();

    for program in package.programs.iter() {
        let (bytecode, build_path) = match &program.data {
            leo_package::ProgramData::Bytecode(bytecode) => {
                // This was a network dependency, and we've downloaded its bytecode.
                (bytecode.clone(), imports_directory.join(format!("{}.aleo", program.name)))
            }
            leo_package::ProgramData::SourcePath(path) => {
                // This is a local dependency, so we must compile it.
                let build_path = if path == &main_source_path {
                    build_directory.join("main.aleo")
                } else {
                    imports_directory.join(format!("{}.aleo", program.name))
                };
                let bytecode = compile_leo_file::<N>(
                    path,
                    program.name,
                    program.is_test,
                    &outputs_directory,
                    &handler,
                    command.options.clone(),
                    stubs.clone(),
                )?;
                (bytecode, build_path)
            }
        };

        // Write the .aleo file.
        std::fs::write(build_path, &bytecode).map_err(CliError::failed_to_load_instructions)?;

        // Track the Stub.
        let stub = leo_disassembler::disassemble_from_str::<N>(program.name, &bytecode)?;
        stubs.insert(program.name, stub);
    }

    // SnarkVM expects to find a `program.json` file in the build directory, so make
    // a bogus one.
    let build_manifest_path = build_directory.join(leo_package::MANIFEST_FILENAME);
    let fake_manifest = Manifest {
        program: package.manifest.program.clone(),
        version: "0.1.0".to_string(),
        description: String::new(),
        license: String::new(),
        dependencies: None,
        dev_dependencies: None,
    };
    fake_manifest.write_to_file(build_manifest_path)?;

    Ok(package)
}

/// Compiles a Leo file. Writes and returns the compiled bytecode.
#[allow(clippy::too_many_arguments)]
fn compile_leo_file<N: Network>(
    source_file_path: &Path,
    program_name: Symbol,
    is_test: bool,
    output_path: &Path,
    handler: &Handler,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
) -> Result<String> {
    // Create a new instance of the Leo compiler.
    let mut compiler = Compiler::<N>::new(
        Some(program_name.to_string()),
        is_test,
        handler.clone(),
        output_path.to_path_buf(),
        Some(options.into()),
        stubs,
    );

    // Compile the Leo program into Aleo instructions.
    let bytecode = compiler.compile_from_file(source_file_path)?;

    tracing::info!("    {} statements before dead code elimination.", compiler.statements_before_dce);
    tracing::info!("    {} statements after dead code elimination.", compiler.statements_after_dce);

    tracing::info!("✅ Compiled '{program_name}.aleo' into Aleo instructions");
    Ok(bytecode)
}
