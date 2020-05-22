//! Compiles a Leo program from a file path.

use crate::{
    ast,
    constraints::{generate_constraints, ConstrainedValue},
    errors::CompilerError,
    InputValue, Program,
};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

use from_pest::FromPest;
use sha2::{Digest, Sha256};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::gadgets::r1cs::ConstraintSynthesizer;
use std::{fs, marker::PhantomData, path::PathBuf};

#[derive(Clone)]
pub struct Compiler<
    P: std::clone::Clone + TEModelParameters,
    F: Field + PrimeField,
    FG: FieldGadget<P::BaseField, F>,
    FF: FieldGadget<F, F>,
> {
    package_name: String,
    main_file_path: PathBuf,
    program: Program<P::BaseField, F>,
    program_inputs: Vec<Option<InputValue<P::BaseField, F>>>,
    output: Option<ConstrainedValue<P, F, FG, FF>>,
    _engine: PhantomData<F>,
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
        FF: FieldGadget<F, F>,
    > Compiler<P, F, FG, FF>
{
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

    pub fn set_inputs(&mut self, program_inputs: Vec<Option<InputValue<P::BaseField, F>>>) {
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
    ) -> Result<ConstrainedValue<P, F, FG, FF>, CompilerError> {
        generate_constraints(cs, self.program, self.program_inputs)
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

    fn parse_program(&mut self) -> Result<(), CompilerError> {
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

        self.program = Program::<P::BaseField, F>::from(syntax_tree, package_name);
        self.program_inputs = vec![None; self.program.num_parameters];

        log::debug!("Program parsing complete\n{:#?}", self.program);

        Ok(())
    }
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
        FF: FieldGadget<F, F>,
    > ConstraintSynthesizer<F> for Compiler<P, F, FG, FF>
{
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let _result =
            generate_constraints::<P, F, FG, FF, CS>(cs, self.program, self.program_inputs)
                .unwrap();

        // Write results to file or something

        Ok(())
    }
}
