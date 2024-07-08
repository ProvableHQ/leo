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

use super::*;

use leo_ast::Stub;
use leo_compiler::{Compiler, CompilerOptions, OutputOptions};
use leo_errors::{CliError, UtilError};
use leo_package::{build::BuildDirectory, outputs::OutputsDirectory, source::SourceDirectory};
use leo_retriever::{Manifest, NetworkName, Retriever};
use leo_span::Symbol;

use snarkvm::{
    package::Package,
    prelude::{MainnetV0, Network, ProgramID, TestnetV0},
};

use indexmap::IndexMap;
use snarkvm::prelude::CanaryV0;
use std::{
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

impl From<BuildOptions> for CompilerOptions {
    fn from(options: BuildOptions) -> Self {
        let mut out_options = Self {
            build: leo_compiler::BuildOptions {
                dce_enabled: options.enable_dce,
                conditional_block_max_depth: options.conditional_block_max_depth,
                disable_conditional_branch_type_checking: options.disable_conditional_branch_type_checking,
            },
            output: OutputOptions {
                symbol_table_spans_enabled: options.enable_symbol_table_spans,
                initial_symbol_table: options.enable_initial_symbol_table_snapshot,
                type_checked_symbol_table: options.enable_type_checked_symbol_table_snapshot,
                unrolled_symbol_table: options.enable_unrolled_symbol_table_snapshot,
                ast_spans_enabled: options.enable_ast_spans,
                initial_ast: options.enable_initial_ast_snapshot,
                unrolled_ast: options.enable_unrolled_ast_snapshot,
                ssa_ast: options.enable_ssa_ast_snapshot,
                flattened_ast: options.enable_flattened_ast_snapshot,
                destructured_ast: options.enable_destructured_ast_snapshot,
                inlined_ast: options.enable_inlined_ast_snapshot,
                dce_ast: options.enable_dce_ast_snapshot,
            },
        };
        if options.enable_all_ast_snapshots {
            out_options.output.initial_ast = true;
            out_options.output.unrolled_ast = true;
            out_options.output.ssa_ast = true;
            out_options.output.flattened_ast = true;
            out_options.output.destructured_ast = true;
            out_options.output.inlined_ast = true;
            out_options.output.dce_ast = true;
        }

        out_options
    }
}

/// Compile and build program command.
#[derive(Parser, Debug)]
pub struct Build {
    #[clap(flatten)]
    pub(crate) options: BuildOptions,
}

impl Command for Build {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Parse the network.
        let network = NetworkName::try_from(context.get_network(&self.options.network)?)?;
        match network {
            NetworkName::MainnetV0 => handle_build::<MainnetV0>(&self, context),
            NetworkName::TestnetV0 => handle_build::<TestnetV0>(&self, context),
            NetworkName::CanaryV0 => handle_build::<CanaryV0>(&self, context),
        }
    }
}

// A helper function to handle the build command.
fn handle_build<N: Network>(command: &Build, context: Context) -> Result<<Build as Command>::Output> {
    // Get the package path.
    let package_path = context.dir()?;
    let home_path = context.home()?;

    // Get the program id.
    let manifest = Manifest::read_from_dir(&package_path)?;
    let program_id = ProgramID::<N>::from_str(manifest.program())?;

    // Clear and recreate the build directory.
    let build_directory = package_path.join("build");
    if build_directory.exists() {
        std::fs::remove_dir_all(&build_directory).map_err(CliError::build_error)?;
    }
    Package::create(&build_directory, &program_id).map_err(CliError::build_error)?;

    // Initialize error handler
    let handler = Handler::default();

    // Retrieve all local dependencies in post order
    let main_sym = Symbol::intern(&program_id.name().to_string());
    let mut retriever = Retriever::<N>::new(
        main_sym,
        &package_path,
        &home_path,
        context.get_endpoint(&command.options.endpoint)?.to_string(),
    )
    .map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;
    let mut local_dependencies =
        retriever.retrieve().map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;

    // Push the main program at the end of the list to be compiled after all of its dependencies have been processed
    local_dependencies.push(main_sym);

    // Recursive build will recursively compile all local dependencies. Can disable to save compile time.
    let recursive_build = !command.options.non_recursive;

    // Loop through all local dependencies and compile them in order
    for dependency in local_dependencies.into_iter() {
        if recursive_build || dependency == main_sym {
            // Get path to the local project
            let (local_path, stubs) = retriever.prepare_local(dependency)?;

            // Create the outputs directory.
            let local_outputs_directory = OutputsDirectory::create(&local_path)?;

            // Open the build directory.
            let local_build_directory = BuildDirectory::create(&local_path)?;

            // Fetch paths to all .leo files in the source directory.
            let local_source_files = SourceDirectory::files(&local_path)?;

            // Check the source files.
            SourceDirectory::check_files(&local_source_files)?;

            // Compile all .leo files into .aleo files.
            for file_path in local_source_files {
                compile_leo_file(
                    file_path,
                    &ProgramID::<N>::try_from(format!("{}.aleo", dependency))
                        .map_err(|_| UtilError::snarkvm_error_building_program_id(Default::default()))?,
                    &local_outputs_directory,
                    &local_build_directory,
                    &handler,
                    command.options.clone(),
                    stubs.clone(),
                )?;
            }
        }

        // Writes `leo.lock` as well as caches objects (when target is an intermediate dependency)
        retriever.process_local(dependency, recursive_build)?;
    }

    // `Package::open` checks that the build directory and that `main.aleo` and all imported files are well-formed.
    Package::<N>::open(&build_directory).map_err(CliError::failed_to_execute_build)?;

    Ok(())
}

/// Compiles a Leo file in the `src/` directory.
#[allow(clippy::too_many_arguments)]
fn compile_leo_file<N: Network>(
    file_path: PathBuf,
    program_id: &ProgramID<N>,
    outputs: &Path,
    build: &Path,
    handler: &Handler,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
) -> Result<()> {
    // Construct program name from the program_id found in `package.json`.
    let program_name = program_id.name().to_string();

    // Create the path to the Aleo file.
    let mut aleo_file_path = build.to_path_buf();
    aleo_file_path.push(format!("main.{}", program_id.network()));

    // Create a new instance of the Leo compiler.
    let mut compiler = Compiler::<N>::new(
        program_name.clone(),
        program_id.network().to_string(),
        handler,
        file_path.clone(),
        outputs.to_path_buf(),
        Some(options.into()),
        stubs,
    );

    // Compile the Leo program into Aleo instructions.
    let instructions = compiler.compile()?;

    // Write the instructions.
    std::fs::File::create(&aleo_file_path)
        .map_err(CliError::failed_to_load_instructions)?
        .write_all(instructions.as_bytes())
        .map_err(CliError::failed_to_load_instructions)?;

    tracing::info!("✅ Compiled '{program_name}.aleo' into Aleo instructions");
    Ok(())
}
