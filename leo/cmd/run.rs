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

use anyhow::Error;
use structopt::StructOpt;

use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};

use snarkvm_algorithms::snark::groth16::Groth16;
use snarkvm_curves::bls12_377::{Bls12_377, Fr};
use snarkvm_models::algorithms::SNARK;

use super::prove::Prove;
use tracing::span::Span;

/// Build, Prove and Run Leo program with inputs
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Run {}

impl Run {
    pub fn new() -> Run {
        Run {}
    }
}

impl Cmd for Run {
    type Input = <Prove as Cmd>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Verifying")
    }

    fn prelude(&self) -> Result<Self::Input, Error> {
        Prove::new().execute()
    }

    fn apply(self, _ctx: Context, input: Self::Input) -> Result<Self::Output, Error> {
        let (proof, prepared_verifying_key) = input;

        tracing::info!("Starting...");

        // Run the verifier
        let is_success = Groth16::<Bls12_377, Compiler<Fr, EdwardsGroupType>, Vec<Fr>>::verify(
            &prepared_verifying_key,
            &vec![],
            &proof,
        )?;

        // Log the verifier output
        match is_success {
            true => tracing::info!("Proof is valid"),
            false => tracing::error!("Proof is invalid"),
        };

        Ok(())
    }
}
