use crate::{cli::*, cli_types::*};
use crate::commands::BuildCommand;
use crate::compiler::Compiler;
use crate::errors::CLIError;

use snarkos_curves::bls12_377::Fr;

use clap::ArgMatches;

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
        let circuit = BuildCommand::output(options)?;

        fn run(circuit: Compiler<Fr>) {

            use snarkos_algorithms::snark::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
            use snarkos_curves::bls12_377::Bls12_377;

            use rand::thread_rng;
            use std::time::{Duration, Instant};

            let mut setup = Duration::new(0, 0);
            let mut proving = Duration::new(0, 0);
            let mut verifying = Duration::new(0, 0);

            let rng = &mut thread_rng();

            let start = Instant::now();

            let params = generate_random_parameters::<Bls12_377, _, _>(circuit.clone(), rng).unwrap();

            let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&params.vk);

            setup += start.elapsed();

            let start = Instant::now();
            let proof = create_random_proof(circuit, &params, rng).unwrap();

            proving += start.elapsed();

            // let _inputs: Vec<_> = [1u32; 1].to_vec();

            let start = Instant::now();

            let is_success = verify_proof(&prepared_verifying_key, &proof, &[]).unwrap();

            verifying += start.elapsed();

            println!(" ");
            println!("  Setup time      : {:?} milliseconds", setup.as_millis());
            println!("  Prover time     : {:?} milliseconds", proving.as_millis());
            println!(
                "  Verifier time   : {:?} milliseconds",
                verifying.as_millis()
            );
            println!("  Verifier output : {}", is_success);
            println!(" ");
        }

        run(circuit);

        // let source_files = SourceDirectory::files(&package_path)?;
        // BuildDirectory::create(&circuit_path).map_err(Error::BuildDirectory)?;
        // DataDirectory::create(&circuit_path).map_err(Error::DataDirectory)?;

        // Compiler::build(
        //     self.verbosity,
        //     &self.witness,
        //     &self.public_data,
        //     &self.circuit,
        //     &source_file_paths,
        // )
        //     .map_err(Error::Compiler)?;
        //
        // VirtualMachine::run(
        //     self.verbosity,
        //     &self.circuit,
        //     &self.witness,
        //     &self.public_data,
        // )
        //     .map_err(Error::VirtualMachine)?;

        Ok(())
    }
}
