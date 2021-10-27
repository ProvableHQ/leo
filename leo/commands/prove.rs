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

use super::{build::BuildOptions, setup::Setup};
use crate::{commands::Command, context::Context};
use leo_errors::{CliError, Result, SnarkVMError};
use leo_package::{outputs::ProofFile, PackageFile};
use snarkvm_algorithms::{
    snark::groth16::{Groth16, PreparedVerifyingKey, Proof},
    traits::SNARK,
};
use snarkvm_curves::bls12_377::{Bls12_377, Fr};
use snarkvm_utilities::bytes::ToBytes;

use rand::thread_rng;
use structopt::StructOpt;
use tracing::span::Span;

/// Run the program and produce a proof
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Prove {
    #[structopt(long = "skip-key-check", help = "Skip key verification on Setup stage")]
    pub(crate) skip_key_check: bool,

    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for Prove {
    type Input = <Setup as Command>::Output;
    type Output = (Proof<Bls12_377>, PreparedVerifyingKey<Bls12_377>);

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Proving")
    }

    fn prelude(&self, context: Context) -> Result<Self::Input> {
        (Setup {
            skip_key_check: self.skip_key_check,
            compiler_options: self.compiler_options.clone(),
        })
        .execute(context)
    }

    fn apply(self, context: Context, input: Self::Input) -> Result<Self::Output> {
        let (program, parameters, prepared_verifying_key) = input;

        // Get the package name
        let path = context.dir()?;
        let package_name = context.manifest()?.get_package_name();

        tracing::info!("Starting...");

        let rng = &mut thread_rng();
        // TODO fix this once snarkvm has better errors.
        let program_proof =
            Groth16::<Bls12_377, _, Vec<Fr>>::prove(&parameters, &program, rng).map_err(|_| SnarkVMError::default())?;

        // Write the proof file to the output directory
        let mut proof = vec![];
        program_proof.write_le(&mut proof).map_err(CliError::cli_io_error)?;
        ProofFile::new(&package_name).write_to(&path, &proof[..])?;

        Ok((program_proof, prepared_verifying_key))
    }
}
