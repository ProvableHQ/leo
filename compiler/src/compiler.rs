//! Compiles a Leo program from a file path.

use crate::{
    constraints::{generate_constraints, generate_test_constraints, ConstrainedValue},
    errors::CompilerError,
    GroupType,
};
use leo_ast::LeoParser;
use leo_inputs::LeoInputsParser;
use leo_types::{InputValue, Inputs, Program};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem, TestConstraintSystem},
};

use sha2::{Digest, Sha256};
use std::{fs, marker::PhantomData, path::PathBuf};

#[derive(Clone)]
pub struct Compiler<F: Field + PrimeField, G: GroupType<F>> {
    package_name: String,
    main_file_path: PathBuf,
    program: Program,
    program_inputs: Inputs,
    output: Option<ConstrainedValue<F, G>>,
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField, G: GroupType<F>> Compiler<F, G> {
    pub fn new() -> Self {
        Self {
            package_name: "".to_string(),
            main_file_path: PathBuf::new(),
            program: Program::new(),
            program_inputs: Inputs::new(),
            output: None,
            _engine: PhantomData,
        }
    }

    pub fn init(package_name: String, main_file_path: PathBuf) -> Result<Self, CompilerError> {
        let mut program = Self {
            package_name,
            main_file_path,
            program: Program::new(),
            program_inputs: Inputs::new(),
            output: None,
            _engine: PhantomData,
        };

        // Generate the abstract syntax tree and assemble the program
        let program_string = program.load_program()?;
        program.parse_program(&program_string)?;

        Ok(program)
    }

    pub fn set_inputs(&mut self, program_inputs: Vec<Option<InputValue>>) {
        self.program_inputs.set_private_inputs(program_inputs);
    }

    pub fn checksum(&self) -> Result<String, CompilerError> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;

        // Hash the file contents
        let mut hasher = Sha256::new();
        hasher.input(unparsed_file.as_bytes());
        let hash = hasher.result();

        Ok(hex::encode(hash))
    }

    pub fn compile_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<ConstrainedValue<F, G>, CompilerError> {
        generate_constraints(cs, self.program, self.program_inputs.get_private_inputs())
    }

    pub fn compile_test_constraints(self, cs: &mut TestConstraintSystem<F>) -> Result<(), CompilerError> {
        generate_test_constraints::<F, G>(cs, self.program)
    }

    fn load_program(&mut self) -> Result<String, CompilerError> {
        // Load the program syntax tree from the file path
        let file_path = &self.main_file_path;
        Ok(LeoParser::load_file(file_path)?)
    }

    pub fn parse_program(&mut self, program_string: &str) -> Result<(), CompilerError> {
        // Parse the program syntax tree
        let syntax_tree = LeoParser::parse_file(&self.main_file_path, program_string)?;

        // Build program from syntax tree
        let package_name = self.package_name.clone();

        self.program = Program::from(syntax_tree, package_name);
        self.program_inputs.set_private_inputs_size(self.program.num_parameters);

        log::debug!("Program parsing complete\n{:#?}", self.program);

        Ok(())
    }

    pub fn parse_inputs(&mut self, file_path: &PathBuf) -> Result<(), CompilerError> {
        let mut path = file_path.clone();
        path.push("inputs");
        path.push("inputs.leo");

        let input_file = &LeoInputsParser::load_file(&path)?;
        let syntax_tree = LeoInputsParser::parse_file(&path, input_file)?;
        // println!("{:?}", syntax_tree);

        // Check number of private parameters here

        self.program_inputs = Inputs::from_inputs_file(syntax_tree)?;

        Ok(())
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstraintSynthesizer<F> for Compiler<F, G> {
    fn generate_constraints<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let _result =
            generate_constraints::<_, G, _>(cs, self.program, self.program_inputs.get_private_inputs()).unwrap();

        // Write results to file or something

        Ok(())
    }
}
