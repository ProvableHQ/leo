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
    commands::BuildCommand,
    errors::{CLIError, RunError},
};
use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};
use leo_package::{
    outputs::{ProvingKeyFile, VerificationKeyFile},
    root::Manifest,
    source::{MAIN_FILE_NAME, SOURCE_DIRECTORY_NAME},
};

use snarkos_algorithms::snark::groth16::{Groth16, Parameters, PreparedVerifyingKey, VerifyingKey};
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_models::algorithms::snark::SNARK;

use clap::ArgMatches;
use rand::thread_rng;
use std::{convert::TryFrom, env::current_dir, time::Instant};

#[derive(Debug)]
pub struct SetupCommand;

impl CLI for SetupCommand {
    type Options = ();
    type Output = (
        Compiler<Fr, EdwardsGroupType>,
        Parameters<Bls12_377>,
        PreparedVerifyingKey<Bls12_377>,
    );

    const ABOUT: AboutType = "Run a program setup";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "setup";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        match BuildCommand::output(options)? {
            Some((program, checksum_differs)) => {
                // Begin "Setup" context for console logging
                let span = tracing::span!(tracing::Level::INFO, "Setup");
                let enter = span.enter();

                // Check if a proving key and verification key already exists
                let keys_exist = ProvingKeyFile::new(&package_name).exists_at(&path)
                    && VerificationKeyFile::new(&package_name).exists_at(&path);

                // If keys do not exist or the checksum differs, run the program setup
                // If keys do not exist or the checksum differs, run the program setup
                let (proving_key, prepared_verifying_key) = if !keys_exist || checksum_differs {
                    tracing::info!("Starting...");

                    // Start the timer for setup
                    let setup_start = Instant::now();

                    // Run the program setup operation
                    let rng = &mut thread_rng();
                    let (proving_key, prepared_verifying_key) =
                        Groth16::<Bls12_377, Compiler<Fr, _>, Vec<Fr>>::setup(program.clone(), rng).unwrap();

                    // Output the setup time
                    tracing::info!("Completed in {:?} milliseconds", setup_start.elapsed().as_millis());

                    // TODO (howardwu): Convert parameters to a 'proving key' struct for serialization.
                    // Write the proving key file to the output directory
                    let mut proving_key_bytes = vec![];
                    proving_key.write(&mut proving_key_bytes)?;
                    let lpk_file = ProvingKeyFile::new(&package_name).write_to(&path, &proving_key_bytes)?;
                    tracing::info!("Saving proving key ({:?})", lpk_file);

                    // Write the verification key file to the output directory
                    let mut verification_key = vec![];
                    proving_key.vk.write(&mut verification_key)?;
                    let lvk_file = VerificationKeyFile::new(&package_name).write_to(&path, &verification_key)?;
                    tracing::info!("Saving verification key ({:?})", lvk_file);

                    (proving_key, prepared_verifying_key)
                } else {
                    tracing::info!("Loading saved setup...");

                    // Read the proving key file from the output directory
                    let proving_key_bytes = ProvingKeyFile::new(&package_name).read_from(&path)?;
                    let proving_key = Parameters::<Bls12_377>::read(proving_key_bytes.as_slice(), true)?;

                    // Read the verification key file from the output directory
                    let verifying_key_bytes = VerificationKeyFile::new(&package_name).read_from(&path)?;
                    let verifying_key = VerifyingKey::<Bls12_377>::read(verifying_key_bytes.as_slice())?;

                    // Derive the prepared verifying key file from the verifying key
                    let prepared_verifying_key = PreparedVerifyingKey::<Bls12_377>::from(verifying_key);

                    (proving_key, prepared_verifying_key)
                };

                // Drop "Setup" context for console logging
                drop(enter);

                // Begin "Finished" context for console logging
                tracing::span!(tracing::Level::INFO, "Finished").in_scope(|| {
                    tracing::info!("Program setup complete\n");
                });

                Ok((program, proving_key, prepared_verifying_key))
            }
            None => {
                let mut main_file_path = path.clone();
                main_file_path.push(SOURCE_DIRECTORY_NAME);
                main_file_path.push(MAIN_FILE_NAME);

                Err(CLIError::RunError(RunError::MainFileDoesNotExist(
                    main_file_path.into_os_string(),
                )))
            }
        }
    }
}
