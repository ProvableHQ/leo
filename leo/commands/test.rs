use crate::directories::source::SOURCE_DIRECTORY_NAME;
use crate::errors::{CLIError, TestError};
use crate::files::{MainFile, Manifest, MAIN_FILE_NAME};
use crate::{cli::*, cli_types::*};
use leo_compiler::compiler::Compiler;

use snarkos_curves::edwards_bls12::{EdwardsParameters, Fq};
use snarkos_gadgets::curves::edwards_bls12::FqGadget;

use clap::ArgMatches;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;
use std::convert::TryFrom;
use std::env::current_dir;

#[derive(Debug)]
pub struct TestCommand;

impl CLI for TestCommand {
    type Options = ();
    type Output = ();

    const NAME: NameType = "test";
    const ABOUT: AboutType = "Compile and run all tests in the current package";
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
                TestError::MainFileDoesNotExist(package_path.as_os_str().to_owned()).into(),
            );
        }

        // Construct the path to the main file in the source directory
        let mut main_file_path = package_path.clone();
        main_file_path.push(SOURCE_DIRECTORY_NAME);
        main_file_path.push(MAIN_FILE_NAME);

        // Compute the current program checksum
        let program = Compiler::<EdwardsParameters, Fq, FqGadget>::init(
            package_name.clone(),
            main_file_path.clone(),
        )?;

        // Generate the program on the constraint system and verify correctness
        {
            let mut cs = TestConstraintSystem::<Fq>::new();
            let temporary_program = program.clone();
            let output = temporary_program.compile_test_constraints(&mut cs)?;
            log::debug!("Compiled constraints - {:#?}", output);
        }

        Ok(())
    }
}
