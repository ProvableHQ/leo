use crate::{cli::*, cli_types::*};
use crate::commands::BuildCommand;
use crate::errors::CLIError;
use leo_compiler::compiler::Compiler;

use snarkos_algorithms::snark::{generate_random_parameters, prepare_verifying_key, Parameters, PreparedVerifyingKey};
use snarkos_curves::bls12_377::{Bls12_377, Fr};

use clap::ArgMatches;
use rand::thread_rng;
use std::time::Instant;

#[derive(Debug)]
pub struct SetupCommand;

impl CLI for SetupCommand {
    type Options = ();
    type Output = (Compiler<Fr>, Parameters<Bls12_377>, PreparedVerifyingKey<Bls12_377>);

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
        let circuit = BuildCommand::output(options)?;

        let start = Instant::now();

        let rng = &mut thread_rng();
        let parameters = generate_random_parameters::<Bls12_377, _, _>(circuit.clone(), rng).unwrap();
        let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&parameters.vk);

        let finish = start.elapsed();

        println!(" ");
        println!("  Setup time      : {:?} milliseconds", finish.as_millis());
        println!(" ");

        Ok((circuit, parameters, prepared_verifying_key))
    }
}
