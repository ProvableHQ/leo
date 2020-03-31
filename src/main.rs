use language::*;

use from_pest::FromPest;
use std::{
    fs,
    marker::PhantomData,
    time::{Duration, Instant},
};

use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::Field;
use snarkos_models::gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem};

use snarkos_algorithms::snark::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};

use rand::thread_rng;

// use std::env;

pub struct Benchmark<F: Field> {
    _engine: PhantomData<F>,
}

impl<F: Field> Benchmark<F> {
    pub fn new() -> Self {
        Self {
            _engine: PhantomData,
        }
    }
}

impl<F: Field> ConstraintSynthesizer<F> for Benchmark<F> {
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        // Read in file as string
        let unparsed_file = fs::read_to_string("simple.program").expect("cannot read file");

        // Parse the file using langauge.pest
        let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");

        // Build the abstract syntax tree
        let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");
        // println!("{:#?}", syntax_tree);

        let program = aleo_program::Program::from(syntax_tree);
        println!(" compiled: {:#?}", program);

        aleo_program::generate_constraints(cs, program);

        Ok(())
    }
}

fn main() {
    let mut setup = Duration::new(0, 0);
    let mut proving = Duration::new(0, 0);
    let mut verifying = Duration::new(0, 0);

    let rng = &mut thread_rng();

    let start = Instant::now();

    let params = {
        let c = Benchmark::<Fr>::new();
        generate_random_parameters::<Bls12_377, _, _>(c, rng).unwrap()
    };

    let prepared_verifying_key = prepare_verifying_key(&params.vk);

    setup += start.elapsed();

    let start = Instant::now();
    let proof = {
        let c = Benchmark::new();
        create_random_proof(c, &params, rng).unwrap()
    };

    proving += start.elapsed();

    // let _inputs: Vec<_> = [1u32; 1].to_vec();

    let start = Instant::now();

    let _ = verify_proof(&prepared_verifying_key, &proof, &[]).unwrap();

    verifying += start.elapsed();

    println!("\n  Setup     time: {:?} seconds", setup.as_secs());
    println!("  Proving   time: {:?} seconds", proving.as_secs());
    println!("  Verifying time: {:?} seconds", verifying.as_secs());

    // let mut cs = TestConstraintSystem::<Fr>::new();
    //
    // println!("\n satisfied: {:?}", cs.is_satisfied());
    //
    // println!(
    //     "\n number of constraints for input: {}",
    //     cs.num_constraints()
    // );
    //
}
