use crate::{
    cli::*,
    cli_types::*,
    errors::{CLIError, InitError},
};
use leo_package::{
    files::{Gitignore, Manifest},
    inputs::*,
    source::{LibFile, MainFile, SourceDirectory},
};

use clap::ArgMatches;
use std::env::current_dir;

#[derive(Debug)]
pub struct InitCommand;

impl CLI for InitCommand {
    type Options = bool;
    type Output = ();

    const ABOUT: AboutType = "Create a new Leo package in an existing directory";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[("--lib"), ("--bin")];
    const NAME: NameType = "init";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(arguments.is_present("lib"))
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let path = current_dir()?;

        // Derive the package name
        let package_name = path
            .file_stem()
            .ok_or_else(|| InitError::ProjectNameInvalid(path.as_os_str().to_owned()))?
            .to_string_lossy()
            .to_string();

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

        // Create a new library or binary file

        if options {
            // Verify the library file does not exist
            if !LibFile::exists_at(&path) {
                // Create the library file in the source directory
                LibFile::new(&package_name).write_to(&path)?;
            }
        } else {
            // Create the input directory
            InputDirectory::create(&path)?;

            // Verify the input file does not exist
            let input_file = InputFile::new(&package_name);
            if !input_file.exists_at(&path) {
                // Create the input file in the input directory
                input_file.write_to(&path)?;
            }

            // Verify the main file does not exist
            if !MainFile::exists_at(&path) {
                // Create the main file in the source directory
                MainFile::new(&package_name).write_to(&path)?;
            }
        }

        Ok(())
    }
}
