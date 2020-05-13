//! Compiles a Leo program from a file path.

use crate::{
    ast,
    constraints::{generate_constraints, ConstrainedValue},
    errors::CompilerError,
    InputValue, Program,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Group, Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem},
};

use from_pest::FromPest;
use sha2::{Digest, Sha256};
use std::{fs, marker::PhantomData, path::PathBuf};

#[derive(Clone)]
pub struct Compiler<F: Field + PrimeField, G: Group> {
    package_name: String,
    main_file_path: PathBuf,
    program: Program<F, G>,
    program_inputs: Vec<Option<InputValue<F, G>>>,
    output: Option<ConstrainedValue<F, G>>,
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField, G: Group> Compiler<F, G> {
    pub fn init(package_name: String, main_file_path: PathBuf) -> Self {
        Self {
            package_name,
            main_file_path,
            program: Program::new(),
            program_inputs: vec![],
            output: None,
            _engine: PhantomData,
        }
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

    // pub fn compile(&self) -> Result<ast::File, CompilerError> {
    //     // Read in the main file as string
    //     let unparsed_file = fs::read_to_string(&self.main_file_path).map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;
    //
    //     // Parse the file using leo.pest
    //     let mut file = ast::parse(&unparsed_file).map_err(|_| CompilerError::FileParsingError)?;
    //
    //     // Build the abstract syntax tree
    //     let syntax_tree = ast::File::from_pest(&mut file).map_err(|_| CompilerError::SyntaxTreeError)?;
    //     log::debug!("{:#?}", syntax_tree);
    //
    //     Ok(syntax_tree)
    // }

    pub fn evaluate_program<CS: ConstraintSystem<F>>(&mut self) -> Result<(), CompilerError> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;

        // Parse the file using leo.pest
        let mut file = ast::parse(&unparsed_file).map_err(|_| CompilerError::FileParsingError)?;

        // Build the abstract syntax tree
        let syntax_tree =
            ast::File::from_pest(&mut file).map_err(|_| CompilerError::SyntaxTreeError)?;
        log::debug!("{:#?}", syntax_tree);

        // Build program from abstract syntax tree
        let package_name = self.package_name.clone();

        self.program = Program::<F, G>::from(syntax_tree, package_name);
        self.program_inputs = vec![None; self.program.num_parameters];

        log::debug!("Compilation complete\n{:#?}", self.program);

        Ok(())
    }
}

impl<F: Field + PrimeField, G: Group> ConstraintSynthesizer<F> for Compiler<F, G> {
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let _res = generate_constraints(cs, self.program, self.program_inputs).unwrap();

        // Write results to file or something

        Ok(())
    }
}
