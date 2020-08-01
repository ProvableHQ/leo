use crate::{
    cli::*,
    cli_types::*,
    commands::SetupCommand,
    errors::CLIError,
    files::{Manifest, ProofFile},
};

use snarkos_algorithms::snark::groth16::{Groth16, PreparedVerifyingKey, Proof};
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_models::algorithms::SNARK;

use clap::ArgMatches;
use rand::thread_rng;
use std::{convert::TryFrom, env::current_dir, time::Instant};

#[derive(Debug)]
pub struct ProveCommand;

impl CLI for ProveCommand {
    type Options = ();
    type Output = (Proof<Bls12_377>, PreparedVerifyingKey<Bls12_377>);

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
        let (program, parameters, prepared_verifying_key) = SetupCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        log::info!("Proving...");

        // Start the timer
        let start = Instant::now();

        let rng = &mut thread_rng();
        let program_proof = Groth16::<Bls12_377, _, Vec<Fr>>::prove(&parameters, program, rng)?;

        // Output the proving time
        log::info!("Prover completed in {:?} milliseconds", start.elapsed().as_millis());

        // Write the proof file to the output directory
        let mut proof = vec![];
        program_proof.write(&mut proof)?;
        ProofFile::new(&package_name).write_to(&path, &proof)?;

        log::info!("Completed program proving");

        Ok((program_proof, prepared_verifying_key))
    }
}
