use crate::{cli::*, cli_types::*};
use crate::commands::BuildCommand;
use crate::errors::CLIError;
use crate::files::{ProvingKeyFile, VerificationKeyFile};
use crate::manifest::Manifest;
use leo_compiler::compiler::Compiler;

use snarkos_algorithms::snark::{
    generate_random_parameters, prepare_verifying_key, Parameters, PreparedVerifyingKey,
};
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_utilities::bytes::ToBytes;

use clap::ArgMatches;
use rand::thread_rng;
use std::convert::TryFrom;
use std::env::current_dir;
use std::time::Instant;

#[derive(Debug)]
pub struct SetupCommand;

impl CLI for SetupCommand {
    type Options = ();
    type Output = (
        Compiler<Fr>,
        Parameters<Bls12_377>,
        PreparedVerifyingKey<Bls12_377>,
    );

    const NAME: NameType = "setup";
    const ABOUT: AboutType = "Run a program setup";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let program = BuildCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        let start = Instant::now();

        let rng = &mut thread_rng();
        let parameters =
            generate_random_parameters::<Bls12_377, _, _>(program.clone(), rng).unwrap();
        let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&parameters.vk);

        log::info!("Setup completed in {:?} milliseconds", start.elapsed().as_millis());

        // Write the proving key file to the inputs directory
        let mut proving_key = vec![];
        parameters.write(&mut proving_key)?;
        ProvingKeyFile::new(&package_name).write_to(&path, &proving_key)?;

        // Write the proving key file to the inputs directory
        let mut verification_key = vec![];
        prepared_verifying_key.write(&mut verification_key)?;
        VerificationKeyFile::new(&package_name).write_to(&path, &verification_key)?;

        Ok((program, parameters, prepared_verifying_key))
    }
}
