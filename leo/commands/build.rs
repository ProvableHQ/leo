// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{commands::Command, context::Context};
use leo_compiler::{
    compiler::{thread_leaked_context, Compiler},
    CompilerOptions, OutputOptions, DEFAULT_INLINE_LIMIT,
};
use leo_errors::{CliError, Result};
use leo_package::{
    inputs::*,
    outputs::{ChecksumFile, CircuitFile, OutputsDirectory},
    source::MainFile,
    PackageDirectory, PackageFile,
};
use leo_parser::parse_program_input;
use leo_synthesizer::{CircuitSynthesizer, SerializedCircuit};

use snarkvm_curves::bls12_377::Bls12_377;
use snarkvm_eval::edwards_bls12::EdwardsGroupType;
use snarkvm_r1cs::ConstraintSystem;
use structopt::StructOpt;
use tracing::span::Span;

// Compiler Options wrapper for Build command. Also used by other commands which
// require Build command output as their input.
#[derive(StructOpt, Clone, Debug)]
pub struct BuildOptions {
    #[structopt(long, help = "Disable constant folding compiler optimization.")]
    pub disable_constant_folding: bool,
    #[structopt(long, help = "Disable dead code elimination compiler optimization.")]
    pub disable_code_elimination: bool,
    #[structopt(long, help = "Disable all compiler optimizations.")]
    pub disable_all_optimizations: bool,
    #[structopt(
        long,
        help = "Writes all AST snapshots for the different compiler phases and emits IR."
    )]
    pub enable_all_snapshots: bool,
    #[structopt(long, help = "Enable spans in AST snapshots.")]
    pub enable_spans: bool,
    #[structopt(long, help = "Writes AST snapshot of the initial parse.")]
    pub enable_initial_ast_snapshot: bool,
    #[structopt(long, help = "Writes AST snapshot after the import resolution phase.")]
    pub enable_imports_resolved_ast_snapshot: bool,
    #[structopt(long, help = "Writes AST snapshot after the canonicalization phase.")]
    pub enable_canonicalized_ast_snapshot: bool,
    #[structopt(long, help = "Writes AST snapshot after the type inference phase.")]
    pub enable_type_inferenced_ast_snapshot: bool,
    #[structopt(
        long,
        default_value = "1000",
        help = "The maximum call depth that leo can attempt to inline"
    )]
    pub inline_limit: u32,
    #[structopt(long, help = "Writes formatted and raw IR.")]
    pub emit_ir: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            disable_constant_folding: Default::default(),
            disable_code_elimination: Default::default(),
            disable_all_optimizations: Default::default(),
            enable_all_snapshots: Default::default(),
            enable_initial_ast_snapshot: Default::default(),
            enable_imports_resolved_ast_snapshot: Default::default(),
            enable_canonicalized_ast_snapshot: Default::default(),
            enable_type_inferenced_ast_snapshot: Default::default(),
            inline_limit: DEFAULT_INLINE_LIMIT,
            enable_spans: Default::default(),
            emit_ir: Default::default(),
        }
    }
}

impl From<BuildOptions> for CompilerOptions {
    fn from(options: BuildOptions) -> Self {
        if options.disable_all_optimizations {
            CompilerOptions {
                constant_folding_enabled: false,
                dead_code_elimination_enabled: false,
                inline_limit: DEFAULT_INLINE_LIMIT,
            }
        } else {
            CompilerOptions {
                constant_folding_enabled: !options.disable_constant_folding,
                dead_code_elimination_enabled: !options.disable_code_elimination,
                inline_limit: options.inline_limit,
            }
        }
    }
}

impl From<BuildOptions> for OutputOptions {
    fn from(options: BuildOptions) -> Self {
        if options.enable_all_snapshots {
            OutputOptions {
                spans_enabled: options.enable_spans,
                ast_initial: true,
                ast_imports_resolved: true,
                ast_canonicalized: true,
                ast_type_inferenced: true,
                emit_ir: true,
            }
        } else {
            OutputOptions {
                spans_enabled: options.enable_spans,
                ast_initial: options.enable_initial_ast_snapshot,
                ast_imports_resolved: options.enable_imports_resolved_ast_snapshot,
                ast_canonicalized: options.enable_canonicalized_ast_snapshot,
                ast_type_inferenced: options.enable_type_inferenced_ast_snapshot,
                emit_ir: options.emit_ir,
            }
        }
    }
}

/// Compile and build program command.
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Build {
    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl<'a> Command<'a> for Build {
    type Input = ();
    type Output = (Compiler<'static, 'a>, leo_ast::Input, bool);

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Build")
    }

    fn prelude(&self, _: Context<'a>) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, context: Context<'a>, _: Self::Input) -> Result<Self::Output> {
        let path = context.dir()?;
        let manifest = context.manifest().map_err(|_| CliError::manifest_file_not_found())?;
        let package_name = manifest.get_package_name();
        let imports_map = manifest.get_imports_map().unwrap_or_default();

        // Error out if there are dependencies but no lock file found.
        if !imports_map.is_empty() && !context.lock_file_exists()? {
            return Err(CliError::dependencies_are_not_installed().into());
        }

        // Sanitize the package path to the root directory.
        let mut package_path = path.clone();
        if package_path.is_file() {
            package_path.pop();
        }

        // Construct the path to the output directory.
        let mut output_directory = package_path.clone();
        output_directory.push(OutputsDirectory::NAME);

        tracing::info!("Starting...");

        let main_file = MainFile::new(&package_name);

        // Compile the main.leo file along with constraints
        if !main_file.exists_at(&package_path) {
            return Err(CliError::package_main_file_not_found().into());
        }

        // Create the output directory
        OutputsDirectory::create(&package_path)?;

        // Construct the path to the main file in the source directory
        let mut main_file_path = package_path.clone();
        main_file_path.push(main_file.filename());

        // Load the input file at `package_name.in`
        let (input_string, input_path) = InputFile::new(&package_name).read_from(&path)?;

        // Load the state file at `package_name.in`
        let (state_string, state_path) = StateFile::new(&package_name).read_from(&path)?;

        // Log compilation of files to console
        tracing::info!("Compiling main program... ({:?})", main_file_path);

        let imports_map = if context.lock_file_exists()? {
            context.lock_file()?.to_import_map()
        } else {
            Default::default()
        };

        // parse the program input
        let input = parse_program_input(
            &input_string,
            &input_path.to_str().unwrap(),
            &state_string,
            &state_path.to_str().unwrap(),
        )?;

        // Load the program at `main_file_path`
        let program = Compiler::parse_program_without_input(
            context.handler,
            package_name.clone(),
            main_file_path,
            output_directory,
            thread_leaked_context(),
            Some(self.compiler_options.clone().into()),
            imports_map,
            Some(self.compiler_options.into()),
        )?;

        // Compute the current program checksum
        let program_checksum = program.checksum()?;

        // Generate the program on the constraint system and verify correctness
        {
            let mut cs = CircuitSynthesizer::<Bls12_377>::default();
            let temporary_program = program.clone();
            let output = temporary_program.compile::<_, EdwardsGroupType, _>(&mut cs, &input)?;

            tracing::debug!("Compiled output - {:#?}", output);
            tracing::info!("Number of constraints - {:#?}", cs.num_constraints());

            // Serialize the circuit
            let circuit_object = SerializedCircuit::from(cs);
            let json = circuit_object.to_json_string().unwrap();
            // println!("json: {}", json);

            // Write serialized circuit to circuit `.json` file.
            let circuit_file = CircuitFile::new(&package_name);
            circuit_file.write_to(&path, json.as_bytes())?;

            // Check that we can read the serialized circuit file
            // let serialized = circuit_file.read_from(&package_path)?;

            // Deserialize the circuit
            // let deserialized = SerializedCircuit::from_json_string(&serialized).unwrap();
            // let _circuit_synthesizer = CircuitSynthesizer::<Bls12_377>::try_from(deserialized).unwrap();
            // println!("deserialized {:?}", circuit_synthesizer.num_constraints());
        }

        // If a checksum file exists, check if it differs from the new checksum
        let checksum_file = ChecksumFile::new(&package_name);
        let checksum_differs = if checksum_file.exists_at(&package_path) {
            let previous_checksum = checksum_file.read_from(&package_path)?;
            program_checksum != previous_checksum
        } else {
            // By default, the checksum differs if there is no checksum to compare against
            true
        };

        // If checksum differs, compile the program
        if checksum_differs {
            // Write the new checksum to the output directory
            checksum_file.write_to(&path, program_checksum.as_bytes())?;

            tracing::debug!("Checksum saved ({:?})", path);
        }

        tracing::info!("Complete");

        Ok((program, input, checksum_differs))
    }
}
