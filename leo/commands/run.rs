use crate::{cli::*, cli_types::*, commands::ProveCommand, errors::CLIError};

use snarkos_algorithms::snark::verify_proof;
use snarkos_curves::bls12_377::Bls12_377;

use clap::ArgMatches;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RunCommand;

impl CLI for RunCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Run a program with inputs";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "run";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<(), CLIError> {
        let (program, proof, prepared_verifying_key) = ProveCommand::output(options)?;

        let mut verifying = Duration::new(0, 0);

        // fetch public inputs
        let inputs: Vec<_> = program.get_public_inputs::<Bls12_377>().unwrap();

        let start = Instant::now();

        let is_success = verify_proof(&prepared_verifying_key, &proof, &inputs).unwrap();

        verifying += start.elapsed();

        println!(" ");
        println!("  Verifier time   : {:?} milliseconds", verifying.as_millis());
        println!("  Verifier output : {}", is_success);
        println!(" ");

        Ok(())
    }
}
