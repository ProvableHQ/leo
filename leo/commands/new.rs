use crate::{
    cli::*,
    cli_types::*,
    directories::{InputsDirectory, SourceDirectory},
    errors::{CLIError, NewError},
    files::{Gitignore, InputsFile, LibFile, MainFile, Manifest},
};

use clap::ArgMatches;
use std::{env::current_dir, fs};

#[derive(Debug)]
pub struct NewCommand;

impl CLI for NewCommand {
    type Options = (Option<String>, bool);
    type Output = ();

    const ABOUT: AboutType = "Create a new Leo package in a new directory";
    const ARGUMENTS: &'static [ArgumentType] = &[
        // (name, description, required, index)
        (
            "NAME",
            "Sets the resulting package name, defaults to the directory name",
            true,
            1u64,
        ),
    ];
    const FLAGS: &'static [FlagType] = &[("--lib"), ("--bin")];
    const NAME: NameType = "new";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        let mut is_lib = false;
        if arguments.is_present("lib") {
            is_lib = true;
        }

        match arguments.value_of("NAME") {
            Some(name) => Ok((Some(name.to_string()), is_lib)),
            None => Ok((None, is_lib)),
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let mut path = current_dir()?;

        // Derive the package name
        let package_name = match options.0 {
            Some(name) => name,
            None => path
                .file_stem()
                .ok_or_else(|| NewError::ProjectNameInvalid(path.as_os_str().to_owned()))?
                .to_string_lossy()
                .to_string(),
        };

        // Derive the package directory path
        path.push(&package_name);

        // Verify the package directory path does not exist yet
        if path.exists() {
            return Err(NewError::DirectoryAlreadyExists(path.as_os_str().to_owned()).into());
        }

        // Create the package directory
        fs::create_dir_all(&path)
            .map_err(|error| NewError::CreatingRootDirectory(path.as_os_str().to_owned(), error))?;

        // Create the manifest file
        Manifest::new(&package_name).write_to(&path)?;

        // Create the .gitignore file
        Gitignore::new().write_to(&path)?;

        // Create the source directory
        SourceDirectory::create(&path)?;

        // Create a new library or binary file
        if options.1 {
            // Create the library file in the source directory
            LibFile::new(&package_name).write_to(&path)?;
        } else {
            // Create the inputs directory
            InputsDirectory::create(&path)?;

            // Create the inputs file in the inputs directory
            InputsFile::new(&package_name).write_to(&path)?;

            // Create the main file in the source directory
            MainFile::new(&package_name).write_to(&path)?;
        }

        Ok(())
    }
}
