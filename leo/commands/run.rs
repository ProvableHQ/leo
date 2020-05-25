use crate::commands::{ProveCommand, SetupCommand};
use crate::errors::CLIError;
use crate::{cli::*, cli_types::*};

use snarkos_algorithms::snark::verify_proof;

use clap::ArgMatches;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RunCommand;

impl CLI for RunCommand {
    type Options = ();
    type Output = ();

    const NAME: NameType = "run";
    const ABOUT: AboutType = "Run a program with inputs";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<(), CLIError> {
        let (_program, _parameters, prepared_verifying_key) = SetupCommand::output(options)?;
        let proof = ProveCommand::output(options)?;

        let mut verifying = Duration::new(0, 0);

        // let _inputs: Vec<_> = [1u32; 1].to_vec();

        let start = Instant::now();

        let is_success = verify_proof(&prepared_verifying_key, &proof, &[]).unwrap();

        verifying += start.elapsed();

        println!(" ");
        println!(
            "  Verifier time   : {:?} milliseconds",
            verifying.as_millis()
        );
        println!("  Verifier output : {}", is_success);
        println!(" ");

        Ok(())
    }
}
