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

use crate::{cli::*, cli_types::*, commands::SetupCommand, errors::CLIError};
use leo_package::{outputs::ProofFile, root::Manifest};

use snarkos_algorithms::snark::groth16::{Groth16, PreparedVerifyingKey, Proof};
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_models::algorithms::SNARK;

use clap::ArgMatches;
use rand::thread_rng;
use std::{convert::TryFrom, env::current_dir, time::Instant};

#[derive(Debug)]
pub struct ProveCommand;

impl CLI for ProveCommand {
    type Options = ();
    type Output = (Proof<Bls12_377>, PreparedVerifyingKey<Bls12_377>);

    const ABOUT: AboutType = "Run the program and produce a proof";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "prove";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let (program, parameters, prepared_verifying_key) = SetupCommand::output(options)?;

        // Begin "Prover" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Prover");
        let enter = span.enter();

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        tracing::info!("Starting...");

        // Start the timer
        let start = Instant::now();

        let rng = &mut thread_rng();
        let program_proof = Groth16::<Bls12_377, _, Vec<Fr>>::prove(&parameters, program, rng)?;

        // Finish the timer
        let end = start.elapsed().as_millis();

        // Write the proof file to the output directory
        let mut proof = vec![];
        program_proof.write(&mut proof)?;
        ProofFile::new(&package_name).write_to(&path, &proof)?;

        // Drop "Prover" context for console logging
        drop(enter);

        // Begin "Finished" context for console logging
        tracing::span!(tracing::Level::INFO, "Finished").in_scope(|| {
            tracing::info!("Completed in {:?} milliseconds\n", end);
        });

        Ok((program_proof, prepared_verifying_key))
    }
}
