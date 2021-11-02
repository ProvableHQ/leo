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

use super::build::{Build, BuildOptions};
use crate::wrapper::CompilerWrapper;
use crate::{commands::Command, context::Context};
use leo_errors::{CliError, Result};
use leo_package::{
    outputs::{ProvingKeyFile, VerificationKeyFile},
    PackageFile,
};

use snarkvm_algorithms::{
    snark::groth16::{Groth16, PreparedVerifyingKey, ProvingKey, VerifyingKey},
    traits::snark::SNARK,
};
use snarkvm_curves::bls12_377::{Bls12_377, Fr};
use snarkvm_utilities::ToBytes;

use rand::thread_rng;
use structopt::StructOpt;
use tracing::span::Span;

/// Executes the setup command for a Leo program
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Setup {
    #[structopt(long = "skip-key-check", help = "Skip key verification")]
    pub(crate) skip_key_check: bool,

    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl<'a> Command<'a> for Setup {
    type Input = <Build as Command<'a>>::Output;
    type Output = (
        CompilerWrapper<'a>,
        ProvingKey<Bls12_377>,
        PreparedVerifyingKey<Bls12_377>,
    );

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Setup")
    }

    fn prelude(&self, context: Context<'a>) -> Result<Self::Input> {
        (Build {
            compiler_options: self.compiler_options.clone(),
        })
        .execute(context)
    }

    fn apply(self, context: Context<'a>, input: Self::Input) -> Result<Self::Output> {
        let path = context.dir()?;
        let package_name = context.manifest()?.get_package_name();

        // Check if leo build failed
        let (program, input, checksum_differs) = input;
        let constraint_compiler = CompilerWrapper(program, input);

        // Check if a proving key and verification key already exists
        let keys_exist = ProvingKeyFile::new(&package_name).exists_at(&path)
            && VerificationKeyFile::new(&package_name).exists_at(&path);

        // If keys do not exist or the checksum differs, run the program setup
        let (proving_key, prepared_verifying_key) = if !keys_exist || checksum_differs {
            tracing::info!("Starting...");

            // Run the program setup operation
            let rng = &mut thread_rng();
            let (proving_key, prepared_verifying_key) =
                Groth16::<Bls12_377, CompilerWrapper, Vec<Fr>>::setup(&constraint_compiler, rng)
                    .map_err(|_| CliError::unable_to_setup())?;

            // TODO (howardwu): Convert parameters to a 'proving key' struct for serialization.
            // Write the proving key file to the output directory
            let proving_key_file = ProvingKeyFile::new(&package_name);
            tracing::info!("Saving proving key ({:?})", proving_key_file.file_path(&path));
            let mut proving_key_bytes = vec![];
            proving_key
                .write_le(&mut proving_key_bytes)
                .map_err(CliError::cli_io_error)?;
            let _ = proving_key_file.write_to(&path, &proving_key_bytes)?;
            tracing::info!("Complete");

            // Write the verification key file to the output directory
            let verification_key_file = VerificationKeyFile::new(&package_name);
            tracing::info!("Saving verification key ({:?})", verification_key_file.file_path(&path));
            let mut verification_key = vec![];
            proving_key
                .vk
                .write_le(&mut verification_key)
                .map_err(CliError::cli_io_error)?;
            let _ = verification_key_file.write_to(&path, &verification_key)?;
            tracing::info!("Complete");

            (proving_key, prepared_verifying_key)
        } else {
            tracing::info!("Detected saved setup");

            // Read the proving key file from the output directory
            tracing::info!("Loading proving key...");

            if self.skip_key_check {
                tracing::info!("Skipping curve check");
            }
            let proving_key_bytes = ProvingKeyFile::new(&package_name).read_from(&path)?;
            let proving_key = ProvingKey::<Bls12_377>::read(proving_key_bytes.as_slice(), !self.skip_key_check)
                .map_err(CliError::cli_io_error)?;
            tracing::info!("Complete");

            // Read the verification key file from the output directory
            tracing::info!("Loading verification key...");
            let verifying_key_bytes = VerificationKeyFile::new(&package_name)
                .read_from(&path)
                .map_err(CliError::cli_io_error)?;
            let verifying_key =
                VerifyingKey::<Bls12_377>::read(verifying_key_bytes.as_slice()).map_err(CliError::cli_io_error)?;

            // Derive the prepared verifying key file from the verifying key
            let prepared_verifying_key = PreparedVerifyingKey::<Bls12_377>::from(verifying_key);
            tracing::info!("Complete");

            (proving_key, prepared_verifying_key)
        };

        Ok((constraint_compiler, proving_key, prepared_verifying_key))
    }
}
