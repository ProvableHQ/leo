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

use crate::{cmd::Cmd, context::Context};
use anyhow::{anyhow, Error};

use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};
use leo_package::{
    outputs::{ProvingKeyFile, VerificationKeyFile},
    source::{MAIN_FILENAME, SOURCE_DIRECTORY_NAME},
};

use rand::thread_rng;
use snarkvm_algorithms::snark::groth16::{Groth16, Parameters, PreparedVerifyingKey, VerifyingKey};
use snarkvm_curves::bls12_377::{Bls12_377, Fr};
use snarkvm_models::algorithms::snark::SNARK;

use std::time::Instant;
use structopt::StructOpt;

use super::build::Build;
use tracing::span::Span;

/// Run setup ceremony for Leo program Command
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Setup {}

impl Setup {
    pub fn new() -> Setup {
        Setup {}
    }
}

impl Cmd for Setup {
    type Input = <Build as Cmd>::Output;
    type Output = (
        Compiler<Fr, EdwardsGroupType>,
        Parameters<Bls12_377>,
        PreparedVerifyingKey<Bls12_377>,
    );

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Setup")
    }

    fn prelude(&self) -> Result<Self::Input, Error> {
        Build::new().execute()
    }

    fn apply(self, ctx: Context, input: Self::Input) -> Result<Self::Output, Error> {
        let path = ctx.dir()?;
        let package_name = ctx.manifest()?.get_package_name();

        match input {
            Some((program, checksum_differs)) => {
                // Check if a proving key and verification key already exists
                let keys_exist = ProvingKeyFile::new(&package_name).exists_at(&path)
                    && VerificationKeyFile::new(&package_name).exists_at(&path);

                // If keys do not exist or the checksum differs, run the program setup
                // If keys do not exist or the checksum differs, run the program setup
                let (proving_key, prepared_verifying_key) = if !keys_exist || checksum_differs {
                    tracing::info!("Starting...");

                    // Run the program setup operation
                    let rng = &mut thread_rng();
                    let (proving_key, prepared_verifying_key) =
                        Groth16::<Bls12_377, Compiler<Fr, _>, Vec<Fr>>::setup(&program, rng).unwrap();

                    // TODO (howardwu): Convert parameters to a 'proving key' struct for serialization.
                    // Write the proving key file to the output directory
                    let proving_key_file = ProvingKeyFile::new(&package_name);
                    tracing::info!("Saving proving key ({:?})", proving_key_file.full_path(&path));
                    let mut proving_key_bytes = vec![];
                    proving_key.write(&mut proving_key_bytes)?;
                    let _ = proving_key_file.write_to(&path, &proving_key_bytes)?;
                    tracing::info!("Complete");

                    // Write the verification key file to the output directory
                    let verification_key_file = VerificationKeyFile::new(&package_name);
                    tracing::info!("Saving verification key ({:?})", verification_key_file.full_path(&path));
                    let mut verification_key = vec![];
                    proving_key.vk.write(&mut verification_key)?;
                    let _ = verification_key_file.write_to(&path, &verification_key)?;
                    tracing::info!("Complete");

                    (proving_key, prepared_verifying_key)
                } else {
                    tracing::info!("Detected saved setup");

                    // Start the timer for setup
                    let setup_start = Instant::now();

                    // Read the proving key file from the output directory
                    tracing::info!("Loading proving key...");
                    let proving_key_bytes = ProvingKeyFile::new(&package_name).read_from(&path)?;
                    let proving_key = Parameters::<Bls12_377>::read(proving_key_bytes.as_slice(), true)?;
                    tracing::info!("Complete");

                    // Read the verification key file from the output directory
                    tracing::info!("Loading verification key...");
                    let verifying_key_bytes = VerificationKeyFile::new(&package_name).read_from(&path)?;
                    let verifying_key = VerifyingKey::<Bls12_377>::read(verifying_key_bytes.as_slice())?;

                    // Derive the prepared verifying key file from the verifying key
                    let prepared_verifying_key = PreparedVerifyingKey::<Bls12_377>::from(verifying_key);
                    tracing::info!("Complete");

                    (proving_key, prepared_verifying_key)
                };

                Ok((program, proving_key, prepared_verifying_key))
            }
            None => {
                let mut main_file_path = path;
                main_file_path.push(SOURCE_DIRECTORY_NAME);
                main_file_path.push(MAIN_FILENAME);

                Err(anyhow!("Unable to build, check that main file exists"))
            }
        }
    }
}
