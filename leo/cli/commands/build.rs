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

use leo_ast::{NodeBuilder, Struct, Stub};
use leo_compiler::{Compiler, CompilerOptions, InputAst, OutputOptions};
use leo_package::{build::BuildDirectory, inputs::InputFile, outputs::OutputsDirectory, source::SourceDirectory};
use leo_span::{symbol::with_session_globals, Symbol};

use snarkvm::{
    package::Package,
    prelude::{ProgramID, Testnet3},
};

use indexmap::IndexMap;

use leo_errors::UtilError;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

use retriever::Retriever;

type CurrentNetwork = Testnet3;

impl From<BuildOptions> for CompilerOptions {
    fn from(options: BuildOptions) -> Self {
        let mut out_options = Self {
            build: leo_compiler::BuildOptions { dce_enabled: options.enable_dce },
            output: OutputOptions {
                symbol_table_spans_enabled: options.enable_symbol_table_spans,
                initial_symbol_table: options.enable_initial_symbol_table_snapshot,
                type_checked_symbol_table: options.enable_type_checked_symbol_table_snapshot,
                unrolled_symbol_table: options.enable_unrolled_symbol_table_snapshot,
                ast_spans_enabled: options.enable_ast_spans,
                initial_input_ast: options.enable_initial_input_ast_snapshot,
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
            out_options.output.initial_input_ast = true;
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
    type Output = (Option<InputAst>, IndexMap<Symbol, Struct>);

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Leo")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Get the package path.
        let package_path = context.dir()?;
        let home_path = context.home()?;

        // Open the build directory.
        let build_directory = BuildDirectory::create(&package_path)?;

        // Get the program id.
        let manifest = context.open_manifest()?;
        let program_id = manifest.program_id();

        // Initialize error handler
        let handler = Handler::default();

        // Initialize a node counter.
        let node_builder = NodeBuilder::default();

        // Store all struct declarations made in the source files.
        let mut structs = IndexMap::new();

        // Retrieve all local dependencies in post order
        let main_sym = Symbol::intern(&program_id.name().to_string());
        let mut retriever = Retriever::new(main_sym, &package_path, &home_path)
            .map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;
        let mut local_dependencies =
            retriever.retrieve().map_err(|err| UtilError::failed_to_retrieve_dependencies(err, Default::default()))?;

        // Push the main program at the end of the list to be compiled after all of its dependencies have been processed
        local_dependencies.push(main_sym);

        // Loop through all local dependencies and compile them in order
        for dependency in local_dependencies.into_iter() {
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
                structs.extend(compile_leo_file(
                    file_path,
                    &ProgramID::<Testnet3>::try_from(format!("{}.aleo", dependency))
                        .map_err(|_| UtilError::snarkvm_error_building_program_id(Default::default()))?,
                    &local_outputs_directory,
                    &local_build_directory,
                    &handler,
                    self.options.clone(),
                    stubs.clone(),
                )?);
            }

            // Writes `leo.lock` as well as caches objects (when target is an intermediate dependency)
            retriever.process_local(dependency)?;
        }

        // Load the input file at `package_name.in`
        let input_file_path = InputFile::new(&manifest.program_id().name().to_string()).setup_file_path(&package_path);

        // Parse the input file.
        let input_ast = if input_file_path.exists() {
            // Load the input file into the source map.
            let input_sf = with_session_globals(|s| s.source_map.load_file(&input_file_path))
                .map_err(|e| CompilerError::file_read_error(&input_file_path, e))?;

            // TODO: This is a hack to notify the user that something is wrong with the input file. Redesign.
            leo_parser::parse_input(&handler, &node_builder, &input_sf.src, input_sf.start_pos)
                .map_err(|_e| println!("Warning: Failed to parse input file"))
                .ok()
        } else {
            None
        };

        // `Package::open` checks that the build directory and that `main.aleo` and all imported files are well-formed.
        Package::<CurrentNetwork>::open(&build_directory).map_err(CliError::failed_to_execute_build)?;

        // // Unset the Leo panic hook.
        // let _ = std::panic::take_hook();
        //
        // // Change the cwd to the build directory to compile aleo files.
        // std::env::set_current_dir(&build_directory)
        //     .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;
        //
        // // Call the `build` command.
        // let mut args = vec![SNARKVM_COMMAND];
        // if self.options.offline {
        //     args.push("--offline");
        // }
        // let command = AleoBuild::try_parse_from(&args).map_err(CliError::failed_to_execute_aleo_build)?;
        // let result = command.parse().map_err(CliError::failed_to_execute_aleo_build)?;
        //
        // // Log the result of the build
        // tracing::info!("{}", result);

        Ok((input_ast, structs))
    }
}

/// Compiles a Leo file in the `src/` directory.
#[allow(clippy::too_many_arguments)]
fn compile_leo_file(
    file_path: PathBuf,
    program_id: &ProgramID<Testnet3>,
    outputs: &Path,
    build: &Path,
    handler: &Handler,
    options: BuildOptions,
    stubs: IndexMap<Symbol, Stub>,
) -> Result<IndexMap<Symbol, Struct>> {
    // Construct the Leo file name with extension `foo.leo`.
    let file_name =
        file_path.file_name().and_then(|name| name.to_str()).ok_or_else(PackageError::failed_to_get_file_name)?;

    // Construct program name from the program_id found in `package.json`.
    let program_name = program_id.name().to_string();

    // Create the path to the Aleo file.
    let mut aleo_file_path = build.to_path_buf();
    aleo_file_path.push(format!("main.{}", program_id.network()));

    // Create a new instance of the Leo compiler.
    let mut compiler = Compiler::new(
        program_name,
        program_id.network().to_string(),
        handler,
        file_path.clone(),
        outputs.to_path_buf(),
        Some(options.into()),
        stubs,
    );

    // Compile the Leo program into Aleo instructions.
    let (symbol_table, instructions) = compiler.compile()?;

    // Write the instructions.
    std::fs::File::create(&aleo_file_path)
        .map_err(CliError::failed_to_load_instructions)?
        .write_all(instructions.as_bytes())
        .map_err(CliError::failed_to_load_instructions)?;

    tracing::info!("✅ Compiled '{}' into Aleo instructions", file_name);
    Ok(symbol_table.structs)
}
