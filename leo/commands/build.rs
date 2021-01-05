// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{
    cli::*,
    cli_types::*,
    errors::CLIError,
    synthesizer::{CircuitSynthesizer, SerializedCircuit},
};
use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};
use leo_package::{
    inputs::*,
    outputs::{ChecksumFile, CircuitFile, OutputsDirectory, OUTPUTS_DIRECTORY_NAME},
    root::Manifest,
    source::{LibraryFile, MainFile, LIBRARY_FILENAME, MAIN_FILENAME, SOURCE_DIRECTORY_NAME},
};

use snarkvm_curves::{bls12_377::Bls12_377, edwards_bls12::Fq};
use snarkvm_models::gadgets::r1cs::ConstraintSystem;

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir, time::Instant};

#[derive(Debug)]
pub struct BuildCommand;

impl CLI for BuildCommand {
    type Options = ();
    type Output = Option<(Compiler<Fq, EdwardsGroupType>, bool)>;

    const ABOUT: AboutType = "Compile the current package as a program";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "build";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(_options: Self::Options) -> Result<Self::Output, CLIError> {
        // Begin "Compiling" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Compiling");
        let enter = span.enter();

        let path = current_dir()?;

        // Get the package name
        let manifest = Manifest::try_from(path.as_path())?;
        let package_name = manifest.get_package_name();

        // Sanitize the package path to the root directory
        let mut package_path = path.clone();
        if package_path.is_file() {
            package_path.pop();
        }

        // Construct the path to the output directory
        let mut output_directory = package_path.clone();
        output_directory.push(OUTPUTS_DIRECTORY_NAME);

        tracing::info!("Starting...");

        // Start the timer
        let start = Instant::now();

        // Compile the package starting with the lib.leo file
        if LibraryFile::exists_at(&package_path) {
            // Construct the path to the library file in the source directory
            let mut lib_file_path = package_path.clone();
            lib_file_path.push(SOURCE_DIRECTORY_NAME);
            lib_file_path.push(LIBRARY_FILENAME);

            // Log compilation of library file to console
            tracing::info!("Compiling library... ({:?})", lib_file_path);

            // Compile the library file but do not output
            let _program = Compiler::<Fq, EdwardsGroupType>::parse_program_without_input(
                package_name.clone(),
                lib_file_path,
                output_directory.clone(),
            )?;
            tracing::info!("Complete");
        };

        // Compile the main.leo file along with constraints
        if MainFile::exists_at(&package_path) {
            // Create the output directory
            OutputsDirectory::create(&package_path)?;

            // Construct the path to the main file in the source directory
            let mut main_file_path = package_path.clone();
            main_file_path.push(SOURCE_DIRECTORY_NAME);
            main_file_path.push(MAIN_FILENAME);

            // Load the input file at `package_name.in`
            let (input_string, input_path) = InputFile::new(&package_name).read_from(&path)?;

            // Load the state file at `package_name.in`
            let (state_string, state_path) = StateFile::new(&package_name).read_from(&path)?;

            // Log compilation of files to console
            tracing::info!("Compiling main program... ({:?})", main_file_path);

            // Load the program at `main_file_path`
            let program = Compiler::<Fq, EdwardsGroupType>::parse_program_with_input(
                package_name.clone(),
                main_file_path,
                output_directory,
                &input_string,
                &input_path,
                &state_string,
                &state_path,
            )?;

            // Compute the current program checksum
            let program_checksum = program.checksum()?;

            // Generate the program on the constraint system and verify correctness
            {
                let mut cs = CircuitSynthesizer::<Bls12_377> {
                    at: vec![],
                    bt: vec![],
                    ct: vec![],
                    input_assignment: vec![],
                    aux_assignment: vec![],
                };
                let temporary_program = program.clone();
                let output = temporary_program.compile_constraints(&mut cs)?;

                tracing::debug!("Number of constraints - {:#?}", cs.num_constraints());

                // Serialize the circuit
                let circuit_object = SerializedCircuit::new(cs, output);
                let json = circuit_object.to_json_string().unwrap();
                // println!("json: {}", json);

                // Write serialized circuit to circuit `.json` file.
                let circuit_file = CircuitFile::new(&package_name);
                circuit_file.write_to(&path, json)?;

                // Check that we can read the serialized circuit file
                let serialized = circuit_file.read_from(&package_path)?;

                // Deserialize the circuit
                let deserialized = SerializedCircuit::from_json_string(&serialized).unwrap();
                let _circuit_synthesizer = CircuitSynthesizer::<Bls12_377>::try_from(deserialized).unwrap();
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
                checksum_file.write_to(&path, program_checksum)?;

                tracing::debug!("Checksum saved ({:?})", path);
            }

            tracing::info!("Complete");

            // Drop "Compiling" context for console logging
            drop(enter);

            // Begin "Done" context for console logging todo: @collin figure a way to get this output with tracing without dropping span
            tracing::span!(tracing::Level::INFO, "Done").in_scope(|| {
                tracing::info!("Finished in {} milliseconds\n", start.elapsed().as_millis());
            });

            return Ok(Some((program, checksum_differs)));
        }

        drop(enter);

        // Return None when compiling a package for publishing
        // The published package does not need to have a main.leo
        Ok(None)
    }
}
