use crate::directories::{source::SOURCE_DIRECTORY_NAME, OutputsDirectory};
use crate::errors::{BuildError, CLIError};
use crate::files::{ChecksumFile, MainFile, MAIN_FILE_NAME};
use crate::manifest::Manifest;
use crate::{cli::*, cli_types::*};
use leo_compiler::compiler::Compiler;

use snarkos_algorithms::snark::KeypairAssembly;
use snarkos_curves::bls12_377::{Bls12_377, Fr};

use clap::ArgMatches;
use std::convert::TryFrom;
use std::env::current_dir;

#[derive(Debug)]
pub struct BuildCommand;

impl CLI for BuildCommand {
    type Options = ();
    type Output = (Compiler<Fr>, bool);

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

        // Compute the current program checksum
        let mut program = Compiler::<Fr>::init(package_name.clone(), main_file_path.clone());
        let checksum = program.checksum()?;

        // If a checksum file exists, check if it differs from the new checksum
        let checksum_file = ChecksumFile::new(&package_name);
        let checksum_differs = if checksum_file.exists_at(&package_path) {
            let previous_checksum = checksum_file.read_from(&package_path)?;
            checksum != previous_checksum
        } else {
            // By default, the checksum differs if there is no checksum to compare against
            true
        };

        // If checksum differs, compile the program
        if checksum_differs {
            // Write the new checksum to the outputs directory
            checksum_file.write_to(&path, checksum)?;

            // Generate the program on the constraint system and verify correctness
            // let mut cs = KeypairAssembly::<Bls12_377> {
            //     num_inputs: 0,
            //     num_aux: 0,
            //     num_constraints: 0,
            //     at: vec![],
            //     bt: vec![],
            //     ct: vec![],
            // };
        }
        program.evaluate_program::<KeypairAssembly::<Bls12_377>>()?;

        log::info!("Compiled program in {:?}", main_file_path);

        Ok((program, checksum_differs))
    }
}
