use crate::directories::{source::SOURCE_DIRECTORY_NAME, OutputsDirectory};
use crate::errors::{BuildError, CLIError};
use crate::files::{MainFile, MAIN_FILE_NAME};
use crate::manifest::Manifest;
use crate::{cli::*, cli_types::*};
use leo_compiler::compiler::Compiler;

use snarkos_curves::bls12_377::Fr;

use clap::ArgMatches;
use std::convert::TryFrom;
use std::env::current_dir;

#[derive(Debug)]
pub struct BuildCommand;

impl CLI for BuildCommand {
    type Options = ();
    type Output = Compiler<Fr>;

    const NAME: NameType = "build";
    const ABOUT: AboutType = "Compile the current package";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(_options: Self::Options) -> Result<Self::Output, CLIError> {
        let path = current_dir()?;

        // Get the package name
        let manifest = Manifest::try_from(&path)?;
        let package_name = manifest.get_package_name();

        // Sanitize the package path to the root directory
        let mut package_path = path.clone();
        if package_path.is_file() {
            package_path.pop();
        }

        // Verify the main file exists
        if !MainFile::exists_at(&package_path) {
            return Err(
                BuildError::MainFileDoesNotExist(package_path.as_os_str().to_owned()).into(),
            );
        }

        // Create the outputs directory
        OutputsDirectory::create(&package_path)?;

        // Construct the path to the main file in the source directory
        let mut main_file_path = package_path.clone();
        main_file_path.push(SOURCE_DIRECTORY_NAME);
        main_file_path.push(MAIN_FILE_NAME);

        log::info!("Compiling program located in {:?}", main_file_path);

        // Compile from the main file path
        let circuit = Compiler::<Fr>::init(package_name, main_file_path);

        Ok(circuit)
    }
}
