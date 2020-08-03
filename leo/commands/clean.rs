use crate::{cli::*, cli_types::*, errors::CLIError};
use leo_package::{
    files::Manifest,
    outputs::{ChecksumFile, ProofFile, ProvingKeyFile, VerificationKeyFile},
};

use clap::ArgMatches;
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
        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        // Remove the checksum from the output directory
        ChecksumFile::new(&package_name).remove(&path)?;

        // Remove the proving key from the output directory
        ProvingKeyFile::new(&package_name).remove(&path)?;

        // Remove the verification key from the output directory
        VerificationKeyFile::new(&package_name).remove(&path)?;

        // Remove the proof from the output directory
        ProofFile::new(&package_name).remove(&path)?;

        Ok(())
    }
}
