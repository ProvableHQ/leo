use leo::*;

use snarkos_algorithms::snark::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem}
};

use from_pest::FromPest;
use rand::thread_rng;
use std::{
    fs,
    marker::PhantomData,
    time::{Duration, Instant},
};

pub struct Benchmark<F: Field + PrimeField> {
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField> Benchmark<F> {
    pub fn new() -> Self {
        Self {
            _engine: PhantomData,
        }
    }
}

impl<F: Field + PrimeField> ConstraintSynthesizer<F> for Benchmark<F> {
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        // Read in file as string
        let unparsed_file = fs::read_to_string("simple.leo").expect("cannot read file");

        // Parse the file using langauge.pest
        let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");

        // Build the abstract syntax tree
        let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");
        // println!("{:#?}", syntax_tree);

        let program = program::Program::<'_, F>::from(syntax_tree);
        println!(" compiled: {:#?}", program);

        let program = program.name("simple".into());
        program::ResolvedProgram::generate_constraints(cs, program);

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

    let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&params.vk);

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

    println!("  Setup time    : {:?} seconds", setup.as_secs());
    println!("  Proving time  : {:?} seconds", proving.as_secs());
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
