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

use crate::{cli::*, cli_types::*, errors::CLIError};
use leo_package::{
    outputs::{ChecksumFile, ProofFile, ProvingKeyFile, VerificationKeyFile},
    root::Manifest,
};

use clap::ArgMatches;
use leo_compiler::OutputFile;
use leo_package::outputs::CircuitFile;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct CleanCommand;

impl CLI for CleanCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Clean the output directory";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "clean";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(_options: Self::Options) -> Result<Self::Output, CLIError> {
        // Begin "Clean" context for console logging
        let span = tracing::span!(tracing::Level::INFO, "Cleaning");
        let enter = span.enter();

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        // Remove the checksum from the output directory
        ChecksumFile::new(&package_name).remove(&path)?;

        // Remove the serialized circuit from the output directory
        CircuitFile::new(&package_name).remove(&path)?;

        // Remove the program output file from the output directory
        OutputFile::new(&package_name).remove(&path)?;

        // Remove the proving key from the output directory
        ProvingKeyFile::new(&package_name).remove(&path)?;

        // Remove the verification key from the output directory
        VerificationKeyFile::new(&package_name).remove(&path)?;

        // Remove the proof from the output directory
        ProofFile::new(&package_name).remove(&path)?;

        // Drop "Compiling" context for console logging
        drop(enter);

        // Begin "Done" context for console logging
        tracing::span!(tracing::Level::INFO, "Done").in_scope(|| {
            tracing::info!("Program workspace cleaned\n");
        });

        Ok(())
    }
}
