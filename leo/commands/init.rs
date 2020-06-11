use crate::{
    cli::*,
    cli_types::*,
    directories::{InputsDirectory, SourceDirectory},
    errors::{CLIError, InitError},
    files::{Gitignore, MainFile, Manifest},
};

use crate::files::InputsFile;
use clap::ArgMatches;
use std::env::current_dir;

#[derive(Debug)]
pub struct InitCommand;

impl CLI for InitCommand {
    type Options = Option<String>;
    type Output = ();

    const ABOUT: AboutType = "Create a new Leo package in an existing directory";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "init";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(None)
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let name = options;
        let path = current_dir()?;

        // Derive the package name
        let package_name = match name {
            Some(name) => name,
            None => path
                .file_stem()
                .ok_or_else(|| InitError::ProjectNameInvalid(path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        };

        // Verify the directory exists
        if !path.exists() {
            return Err(InitError::DirectoryDoesNotExist(path.as_os_str().to_owned()).into());
        }

        // Verify a manifest file does not already exist
        if Manifest::exists_at(&path) {
            return Err(InitError::PackageAlreadyExists(path.as_os_str().to_owned()).into());
        }

        // Create the manifest file
        Manifest::new(&package_name).write_to(&path)?;

        // Create the .gitignore file
        Gitignore::new().write_to(&path)?;

        // Create the source directory
        SourceDirectory::create(&path)?;

        // Create the inputs directory
        InputsDirectory::create(&path)?;

        // Verify the inputs file does not exist
        if !InputsFile::exists_at(&path) {
            // Create the main file in the source directory
            InputsFile::new(&package_name).write_to(&path)?;
        }

        // Verify the main file does not exist
        if !MainFile::exists_at(&path) {
            // Create the main file in the source directory
            MainFile::new(&package_name).write_to(&path)?;
        }

        Ok(())
    }
}
