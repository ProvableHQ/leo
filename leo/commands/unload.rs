use crate::{
    cli::*,
    cli_types::*,
    commands::BuildCommand,
    errors::{CLIError, RunError},
};
use leo_package::{
    files::Manifest,
    source::{MAIN_FILE_NAME, SOURCE_DIRECTORY_NAME},
};

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct UnloadCommand;

impl CLI for UnloadCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Uninstall a package from the current package (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "unload";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let path = current_dir()?;

        match BuildCommand::output(options)? {
            Some((_program, _checksum_differs)) => {
                // Get the package name
                let _package_name = Manifest::try_from(&path)?.get_package_name();

                log::info!("Unimplemented - `leo load`");

                Ok(())
            }
            None => {
                let mut main_file_path = path.clone();
                main_file_path.push(SOURCE_DIRECTORY_NAME);
                main_file_path.push(MAIN_FILE_NAME);

                Err(CLIError::RunError(RunError::MainFileDoesNotExist(
                    main_file_path.into_os_string(),
                )))
            }
        }
    }
}
