use leo_compiler::{self, ast, errors::CompilerError, InputValue, Program};

use from_pest::FromPest;
use rand::thread_rng;
use snarkos_algorithms::snark::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use snarkos_curves::bls12_377::{Bls12_377, Fr};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem},
};
use std::{
    fs,
    marker::PhantomData,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct Benchmark<F: Field + PrimeField> {
    program: Program<F>,
    parameters: Vec<Option<InputValue<F>>>,
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField> Benchmark<F> {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            parameters: vec![],
            _engine: PhantomData,
        }
    }

    pub fn evaluate_program(&mut self) -> Result<(), CompilerError> {
        // Read in file as string
        let unparsed_file = fs::read_to_string("simple.leo").expect("cannot read file");

        // Parse the file using leo.pest
        let mut file = ast::parse(&unparsed_file).expect("unsuccessful parse");

        // Build the abstract syntax tree
        let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");
        // println!("{:#?}", syntax_tree);

        // Build a leo program from the syntax tree
        self.program = Program::<F>::from(syntax_tree, "simple".into());
        self.parameters = vec![None; self.program.num_parameters];

        println!(" compiled: {:#?}\n", self.program);

        Ok(())
    }
}

impl<F: Field + PrimeField> ConstraintSynthesizer<F> for Benchmark<F> {
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let _res =
            leo_compiler::generate_constraints(cs, self.program, self.parameters).unwrap();
        println!(" Result: {}", _res);

        // Write results to file or something

        Ok(())
    }
}

fn main() {
    let mut setup = Duration::new(0, 0);
    let mut proving = Duration::new(0, 0);
    let mut verifying = Duration::new(0, 0);

    let rng = &mut thread_rng();

    let start = Instant::now();

    // Load and compile program
    let mut program = Benchmark::<Fr>::new();
    program.evaluate_program().unwrap();

    // Generate proof parameters
    let params = { generate_random_parameters::<Bls12_377, _, _>(program.clone(), rng).unwrap() };

    let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&params.vk);

    setup += start.elapsed();

    let start = Instant::now();

    // Set main function arguments in compiled program
    // let argument = Some(InputValue::Field(Fr::one()));

    // let bool_true = InputValue::Boolean(true);
    // let array = InputValue::Array(vec![bool_true.clone(), bool_true.clone(), bool_true.clone()]);
    // let argument = Some(array);
    //
    // program.parameters = vec![argument];

    // Generate proof
    let proof = create_random_proof(program, &params, rng).unwrap();

    proving += start.elapsed();

    let start = Instant::now();

    // let public_input = Fr::one();

    // Verify proof
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
