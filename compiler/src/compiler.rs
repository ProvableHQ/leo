//! Compiles a Leo program from a file path.

use crate::{
    constraints::{generate_constraints, generate_test_constraints, ConstrainedValue},
    errors::CompilerError,
    GroupType, Program,
};
use leo_ast::{ast, files::File};
use leo_types::InputValue;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem, TestConstraintSystem},
};

use from_pest::FromPest;
use sha2::{Digest, Sha256};
use std::{fs, marker::PhantomData, path::PathBuf};

#[derive(Clone)]
pub struct Compiler<F: Field + PrimeField, G: GroupType<F>> {
    package_name: String,
    main_file_path: PathBuf,
    program: Program,
    program_inputs: Vec<Option<InputValue>>,
    output: Option<ConstrainedValue<F, G>>,
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField, G: GroupType<F>> Compiler<F, G> {
    pub fn init(package_name: String, main_file_path: PathBuf) -> Result<Self, CompilerError> {
        let mut program = Self {
            package_name,
            main_file_path,
            program: Program::new(),
            program_inputs: vec![],
            output: None,
            _engine: PhantomData,
        };

        // Generate the abstract syntax tree and assemble the program
        program.parse_program()?;

        Ok(program)
    }

    pub fn set_inputs(&mut self, program_inputs: Vec<Option<InputValue>>) {
        self.program_inputs = program_inputs;
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
        generate_constraints(cs, self.program, self.program_inputs)
    }

    pub fn compile_test_constraints(
        self,
        cs: &mut TestConstraintSystem<F>,
    ) -> Result<(), CompilerError> {
        generate_test_constraints::<F, G>(cs, self.program)
    }

    // pub fn compile(&self) -> Result<File, CompilerError> {
    //     // Read in the main file as string
    //     let unparsed_file = fs::read_to_string(&self.main_file_path).map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;
    //
    //     // Parse the file using leo.pest
    //     let mut file = ast::parse(&unparsed_file).map_err(|_| CompilerError::FileParsingError)?;
    //
    //     // Build the abstract syntax tree
    //     let syntax_tree = File::from_pest(&mut file).map_err(|_| CompilerError::SyntaxTreeError)?;
    //     log::debug!("{:#?}", syntax_tree);
    //
    //     Ok(syntax_tree)
    // }

    fn parse_program(&mut self) -> Result<(), CompilerError> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;

        // Parse the file using leo.pest
        let mut file = ast::parse(&unparsed_file).map_err(|error| {
            CompilerError::from(error.with_path(&self.main_file_path.to_str().unwrap()))
        })?;

        // Build the abstract syntax tree
        let syntax_tree =
            File::from_pest(&mut file).map_err(|_| CompilerError::SyntaxTreeError)?;
        log::debug!("{:#?}", syntax_tree);

        // Build program from abstract syntax tree
        let package_name = self.package_name.clone();

        self.program = Program::from(syntax_tree, package_name);
        self.program_inputs = vec![None; self.program.num_parameters];

        log::debug!("Program parsing complete\n{:#?}", self.program);

        Ok(())
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstraintSynthesizer<F> for Compiler<F, G> {
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let _result =
            generate_constraints::<_, G, _>(cs, self.program, self.program_inputs).unwrap();

        // Write results to file or something

        Ok(())
    }
}
