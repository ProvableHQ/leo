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

use super::{build::BuildOptions, prove::Prove};
use crate::{
    commands::{Command, ProgramSNARK},
    context::Context,
};
use leo_errors::{Result, SnarkVMError};

use snarkvm_algorithms::traits::SNARK;
use snarkvm_dpc::ProgramPublicVariables;
use structopt::StructOpt;
use tracing::span::Span;

/// Build, Prove and Run Leo program with inputs
#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Run {
    #[structopt(long = "skip-key-check", help = "Skip key verification on Setup stage")]
    pub(crate) skip_key_check: bool,

    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl<'a> Command<'a> for Run {
    type Input = <Prove as Command<'a>>::Output;
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Verifying")
    }

    fn prelude(&self, context: Context<'a>) -> Result<Self::Input> {
        (Prove {
            skip_key_check: self.skip_key_check,
            compiler_options: self.compiler_options.clone(),
        })
        .execute(context)
    }

    fn apply(self, _context: Context<'a>, input: Self::Input) -> Result<Self::Output> {
        let (proof, verifying_key) = input;

        tracing::info!("Starting...");

        // Run the verifier
        let is_success = ProgramSNARK::verify(&verifying_key, &ProgramPublicVariables::blank(), &proof)
            .map_err(|_| SnarkVMError::default())?;

        // Log the verifier output
        match is_success {
            true => tracing::info!("Proof is valid"),
            false => tracing::error!("Proof is invalid"),
        };

        Ok(())
    }
}
