use crate::{cli::*, cli_types::*, commands::BuildCommand, errors::CLIError};
use leo_package::{
    outputs::OutputsDirectory,
    root::{Manifest, ZipFile},
};

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct PublishCommand;

impl CLI for PublishCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Publish the current package to the package manager (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "publish";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        // Build all program files.
        // It's okay if there's just a lib.leo file here
        let _output = BuildCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        // Create the output directory
        OutputsDirectory::create(&path)?;

        // Create zip file
        let zip_file = ZipFile::new(&package_name);
        if zip_file.exists_at(&path) {
            log::info!("Existing package zip file found. Skipping compression.")
        } else {
            zip_file.write(&path)?;
        }

        Ok(())
    }
}
