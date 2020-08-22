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

use crate::{cli::*, cli_types::*, commands::ProveCommand, errors::CLIError};
use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};

use snarkos_algorithms::snark::groth16::Groth16;
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_models::algorithms::SNARK;

use clap::ArgMatches;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RunCommand;

impl CLI for RunCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Run a program with input variables";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "run";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<(), CLIError> {
        let (proof, prepared_verifying_key) = ProveCommand::output(options)?;

        // Begin "Verifying" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Verifying");
        let _enter = span.enter();

        let mut verifying = Duration::new(0, 0);

        let start = Instant::now();

        let is_success = Groth16::<Bls12_377, Compiler<Fr, EdwardsGroupType>, Vec<Fr>>::verify(
            &prepared_verifying_key,
            &vec![],
            &proof,
        )
        .unwrap();

        verifying += start.elapsed();

        println!("  Verifier time   : {:?} milliseconds", verifying.as_millis());
        println!("  Verifier output : {}", is_success);
        println!(" ");

        Ok(())
    }
}
