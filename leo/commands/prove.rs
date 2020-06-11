use crate::{
    cli::*,
    cli_types::*,
    commands::SetupCommand,
    errors::CLIError,
    files::{Manifest, ProofFile},
};

use snarkos_algorithms::snark::{create_random_proof, Proof};
use snarkos_curves::bls12_377::Bls12_377;

use clap::ArgMatches;
use leo_compiler::{compiler::Compiler, edwards_bls12::EdwardsGroupType};
use rand::thread_rng;
use snarkos_curves::edwards_bls12::Fq;
use std::{convert::TryFrom, env::current_dir, time::Instant};

#[derive(Debug)]
pub struct ProveCommand;

impl CLI for ProveCommand {
    type Options = ();
    type Output = (Compiler<Fq, EdwardsGroupType>, Proof<Bls12_377>);

    const ABOUT: AboutType = "Run the program and produce a proof";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "prove";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let (mut program, parameters, _) = SetupCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        // Fetch private inputs here
        program.parse_inputs(&path)?;

        // Start the timer
        let start = Instant::now();

        let rng = &mut thread_rng();
        let program_proof = create_random_proof(program.clone(), &parameters, rng)?;

        log::info!("Prover completed in {:?} milliseconds", start.elapsed().as_millis());

        // Write the proof file to the outputs directory
        let mut proof = vec![];
        program_proof.write(&mut proof)?;
        ProofFile::new(&package_name).write_to(&path, &proof)?;

        log::info!("Completed program proving");

        Ok((program, program_proof))
    }
}
