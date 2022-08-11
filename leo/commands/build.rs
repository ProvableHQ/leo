// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::commands::ALEO_CLI_COMMAND;
use crate::{commands::Command, context::Context};

use leo_ast::Circuit;
use leo_compiler::{Compiler, InputAst, OutputOptions};
use leo_errors::{CliError, CompilerError, PackageError, Result};
use leo_package::source::{SourceDirectory, MAIN_FILENAME};
use leo_package::{inputs::InputFile, outputs::OutputsDirectory};
use leo_span::symbol::with_session_globals;

use aleo::commands::Build as AleoBuild;

use clap::StructOpt;
use colored::Colorize;
use indexmap::IndexMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use leo_errors::emitter::Handler;
use leo_package::build::BuildDirectory;
use leo_package::imports::ImportsDirectory;
use leo_span::Symbol;
use tracing::span::Span;

/// Compiler Options wrapper for Build command. Also used by other commands which
/// require Build command output as their input.
#[derive(StructOpt, Clone, Debug, Default)]
pub struct BuildOptions {
    #[structopt(long, help = "Enables offline mode.")]
    pub offline: bool,
    #[structopt(long, help = "Enable spans in AST snapshots.")]
    pub enable_spans: bool,
    #[structopt(long, help = "Writes all AST snapshots for the different compiler phases.")]
    pub enable_all_ast_snapshots: bool,
    #[structopt(long, help = "Writes Input AST snapshot of the initial parse.")]
    pub enable_initial_input_ast_snapshot: bool,
    #[structopt(long, help = "Writes AST snapshot of the initial parse.")]
    pub enable_initial_ast_snapshot: bool,
    #[structopt(long, help = "Writes AST snapshot of the inlined AST.")]
    pub enable_inlined_ast_snapshot: bool,
    #[structopt(long, help = "Writes AST snapshot of the unrolled AST.")]
    pub enable_unrolled_ast_snapshot: bool,
    #[structopt(long, help = "Writes AST snapshot of the SSA AST.")]
    pub enable_ssa_ast_snapshot: bool,
    // Note: This is currently made optional since code generation is just a prototype.
    #[structopt(
        long,
        help = "Runs the code generation stage of the compiler and prints the resulting bytecode."
    )]
    pub enable_code_generation: bool,
}

impl From<BuildOptions> for OutputOptions {
    fn from(options: BuildOptions) -> Self {
        let mut out_options = Self {
            spans_enabled: options.enable_spans,
            initial_input_ast: options.enable_initial_input_ast_snapshot,
            initial_ast: options.enable_initial_ast_snapshot,
            inlined_ast: options.enable_inlined_ast_snapshot,
            unrolled_ast: options.enable_unrolled_ast_snapshot,
            ssa_ast: options.enable_ssa_ast_snapshot,
        };
        if options.enable_all_ast_snapshots {
            out_options.initial_input_ast = true;
            out_options.initial_ast = true;
            out_options.inlined_ast = true;
            out_options.unrolled_ast = true;
            out_options.ssa_ast = true;
        }

        out_options
    }
}

/// Compile and build program command.
#[derive(StructOpt, Debug)]
pub struct Build {
    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for Build {
    type Input = ();
    type Output = (Option<InputAst>, IndexMap<Symbol, Circuit>);

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Build")
    }

    fn prelude(&self, _: Context) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context, _: Self::Input) -> Result<Self::Output> {
        // Get the package path.
        let package_path = context.dir()?;

        // Get the program name.
        let package_name = context.open_manifest()?.program_id().name().to_string();

        // Create the outputs directory.
        let outputs_directory = OutputsDirectory::create(&package_path)?;

        // Open the build directory.
        let build_directory = BuildDirectory::open(&package_path)?;

        // Initialize error handler
        let handler = Handler::default();

        // Fetch paths to all .leo files in the source directory.
        let source_files = SourceDirectory::files(&package_path)?;

        // Store all circuits declarations made in the source files.
        let mut circuits = IndexMap::new();

        // Compile all .leo files into .aleo files.
        for file_path in source_files.into_iter() {
            circuits.extend(compile_leo_file(
                file_path,
                &package_path,
                &package_name,
                &outputs_directory,
                &build_directory,
                &handler,
                self.compiler_options.clone(),
            )?);
        }

        if !ImportsDirectory::is_empty(&package_path)? {
            // Create Aleo build/imports/ directory.
            let build_imports_directory = ImportsDirectory::create(&build_directory)?;

            // Fetch paths to all .leo files in the imports directory.
            let import_files = ImportsDirectory::files(&package_path)?;

            // Compile all .leo files into .aleo files.
            for file_path in import_files.into_iter() {
                circuits.extend(compile_leo_file(
                    file_path,
                    &package_path,
                    &package_name,
                    &outputs_directory,
                    &build_imports_directory,
                    &handler,
                    self.compiler_options.clone(),
                )?);
            }
        }

        // Load the input file at `package_name.in`
        let input_file_path = InputFile::new(&package_name).setup_file_path(&package_path);

        // Parse the input file.
        let input_ast = if input_file_path.exists() {
            // Load the input file into the source map.
            let input_sf = with_session_globals(|s| s.source_map.load_file(&input_file_path))
                .map_err(|e| CompilerError::file_read_error(&input_file_path, e))?;

            // TODO: This is a hack to notify the user that something is wrong with the input file. Redesign.
            leo_parser::parse_input(&handler, &input_sf.src, input_sf.start_pos)
                .map_err(|_e| println!("Warning: Failed to parse input file"))
                .ok()
        } else {
            None
        };

        // Change the cwd to the build directory to compile aleo files.
        std::env::set_current_dir(&build_directory)
            .map_err(|err| PackageError::failed_to_set_cwd(build_directory.display(), err))?;

        // Call the `aleo build` command with the appropriate from the Aleo SDK.
        let mut args = vec![ALEO_CLI_COMMAND];
        if self.compiler_options.offline {
            args.push("--offline");
        }
        let command = AleoBuild::try_parse_from(&args).map_err(CliError::failed_to_execute_aleo_build)?;
        let result = command.parse().map_err(CliError::failed_to_execute_aleo_build)?;

        // Log the result of the build
        tracing::info!("{}", result);

        Ok((input_ast, circuits))
    }
}

fn compile_leo_file(
    file_path: PathBuf,
    _package_path: &Path,
    package_name: &String,
    outputs: &Path,
    build: &Path,
    handler: &Handler,
    options: BuildOptions,
) -> Result<IndexMap<Symbol, Circuit>> {
    // Construct the Leo file name with extension `foo.leo`.
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(PackageError::failed_to_get_file_name)?;

    // Construct program name from file_path name `foo`.
    let program_name = file_name
        .strip_suffix(".leo")
        .ok_or_else(PackageError::failed_to_get_file_name)?;

    // Construct program id header for aleo file.
    // Do not create a program with main.aleo as the ID.
    let program_id_name = if file_name.eq(MAIN_FILENAME) {
        package_name
    } else {
        program_name
    };

    // Create a new instance of the Leo compiler.
    let mut program = Compiler::new(
        program_id_name.to_string(),
        String::from("aleo"), // todo: fetch this from Network::Testnet3
        handler,
        file_path.clone(),
        outputs.to_path_buf(),
        Some(options.into()),
    );

    // TODO: Temporarily removing checksum files. Need to redesign this scheme.
    // // Check if we need to compile the Leo program.
    // let checksum_differs = {
    //     // Compute the current program checksum.
    //     let program_checksum = program.checksum()?;
    //
    //     // Get the current program checksum.
    //     let checksum_file = ChecksumFile::new(program_name);
    //
    //     // If a checksum file exists, check if it differs from the new checksum.
    //     let checksum_differs = if checksum_file.exists_at(package_path) {
    //         let previous_checksum = checksum_file.read_from(package_path)?;
    //         program_checksum != previous_checksum
    //     } else {
    //         // By default, the checksum differs if there is no checksum to compare against.
    //         true
    //     };
    //
    //     // If checksum differs, compile the program
    //     if checksum_differs {
    //         // Write the new checksum to the output directory
    //         checksum_file.write_to(package_path, program_checksum)?;
    //
    //         tracing::debug!("Checksum saved ({:?})", package_path);
    //     }
    //
    //     checksum_differs
    // };

    // if checksum_differs {
    // Compile the Leo program into Aleo instructions.
    let (symbol_table, instructions) = program.compile_and_generate_instructions()?;

    // Create the path to the Aleo file.
    let mut aleo_file_path = build.to_path_buf();
    aleo_file_path.push(format!("{}.aleo", program_name));

    // Write the instructions.
    std::fs::File::create(&aleo_file_path)
        .map_err(CliError::failed_to_load_instructions)?
        .write_all(instructions.as_bytes())
        .map_err(CliError::failed_to_load_instructions)?;

    // Prepare the path string.
    let path_string = format!("(in \"{}\")", aleo_file_path.display());

    // Log the build as successful.
    tracing::info!(
        "✅ Compiled '{}' into Aleo instructions {}",
        file_name,
        path_string.dimmed()
    );
    // }

    Ok(symbol_table.circuits)
}
