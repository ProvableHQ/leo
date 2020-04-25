// use leo_program::{self, ast};
//
// use snarkos_algorithms::snark::{
//     create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
// };
// use snarkos_curves::bls12_377::{Bls12_377, Fr};
// use snarkos_errors::gadgets::SynthesisError;
// use snarkos_models::{
//     curves::{Field, PrimeField},
//     gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem},
// };
//
// use from_pest::FromPest;
// use rand::thread_rng;
// use std::{
//     fs,
//     marker::PhantomData,
//     time::{Duration, Instant},
// };
//
// pub struct Benchmark<F: Field + PrimeField> {
//     _engine: PhantomData<F>,
// }
//
// impl<F: Field + PrimeField> Benchmark<F> {
//     pub fn init() -> Self {
//         Self {
//             _engine: PhantomData,
//         }
//     }
// }
//
// impl<F: Field + PrimeField> ConstraintSynthesizer<F> for Benchmark<F> {
//     fn generate_constraints<CS: ConstraintSystem<F>>(
//         self,
//         cs: &mut CS,
//     ) -> Result<(), SynthesisError> {
//         // Read in file as string
//         let unparsed_file = fs::read_to_string("simple.leo").expect("cannot read file");
//
//         // Parse the file using leo.pest
//         let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");
//
//         // Build the abstract syntax tree
//         let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");
//         // println!("{:#?}", syntax_tree);
//
//         let program = leo_program::Program::<'_, F>::from(syntax_tree);
//         println!(" compiled: {:#?}", program);
//
//         let program = program.name("simple".into());
//         leo_program::ResolvedProgram::generate_constraints(cs, program);
//
//         Ok(())
//     }
// }
//
// fn main() {
//     let mut setup = Duration::init(0, 0);
//     let mut proving = Duration::init(0, 0);
//     let mut verifying = Duration::init(0, 0);
//
//     let rng = &mut thread_rng();
//
//     let start = Instant::now();
//
//     let params = {
//         let circuit = Benchmark::<Fr>::init();
//         generate_random_parameters::<Bls12_377, _, _>(circuit, rng).unwrap()
//     };
//
//     let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&params.vk);
//
//     setup += start.elapsed();
//
//     let start = Instant::now();
//     let proof = {
//         let c = Benchmark::init();
//         create_random_proof(c, &params, rng).unwrap()
//     };
//
//     proving += start.elapsed();
//
//     // let _inputs: Vec<_> = [1u32; 1].to_vec();
//
//     let start = Instant::now();
//
//     let is_success = verify_proof(&prepared_verifying_key, &proof, &[]).unwrap();
//
//     verifying += start.elapsed();
//
//     println!(" ");
//     println!("  Setup time      : {:?} milliseconds", setup.as_millis());
//     println!("  Prover time     : {:?} milliseconds", proving.as_millis());
//     println!(
//         "  Verifier time   : {:?} milliseconds",
//         verifying.as_millis()
//     );
//     println!("  Verifier output : {}", is_success);
//     println!(" ");
//
//     // let mut cs = TestConstraintSystem::<Fr>::init();
//     //
//     // println!("\n satisfied: {:?}", cs.is_satisfied());
//     //
//     // println!(
//     //     "\n number of constraints for input: {}",
//     //     cs.num_constraints()
//     // );
//     //
// }

use leo::{cli::*, commands::*};
use leo::errors::CLIError;

use clap::{App, AppSettings};

#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), CLIError> {
    let arguments = App::new("leo")
        .version("v0.1.0")
        .about("Leo compiler and package manager")
        .author("The Aleo Team <hello@aleo.org>")
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableVersion,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(vec![
            InitCommand::new(),
        ])
        .set_term_width(0)
        .get_matches();


    match arguments.subcommand() {
        ("init", Some(arguments)) => {
            InitCommand::output(InitCommand::parse(arguments)?)
        },
        _ => unreachable!(),
    }
}