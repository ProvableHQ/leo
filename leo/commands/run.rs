// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use super::build::BuildOptions;
use crate::{commands::Command, context::Context};
use leo_errors::Result;

// use aleo::commands::CLI as AleoCLI;

use clap::StructOpt;
use tracing::span::Span;

/// Build, Prove and Run Leo program with inputs
#[derive(StructOpt, Debug)]
pub struct Run {
    #[structopt(long = "skip-key-check", help = "Skip key verification on Setup stage")]
    pub(crate) skip_key_check: bool,

    #[structopt(flatten)]
    pub(crate) compiler_options: BuildOptions,
}

impl Command for Run {
    type Input = ();
    type Output = ();

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Executing")
    }

    fn prelude(&self, _context: Context) -> Result<Self::Input> {
        Ok(()) // todo: call aleo build here?
    }

    fn apply(self, _context: Context, _input: Self::Input) -> Result<Self::Output> {
        tracing::info!("Starting...");

        // Execute the aleo program.
        // let cli = AleoCLI::parse_from(&["aleo", "run", "main"]);

        // Log the verifier output
        // tracing::info!("Result: {}", res);

        Ok(())
    }
}
