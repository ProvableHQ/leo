use crate::{ast, Program, ResolvedProgram, ResolvedValue};
use crate::errors::CompilerError;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem},
};

use from_pest::FromPest;
use std::{fs, marker::PhantomData, path::PathBuf};

#[derive(Clone)]
pub struct Compiler<F: Field + PrimeField> {
    package_name: String,
    main_file_path: PathBuf,
    output: Option<ResolvedValue<F>>,
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField> Compiler<F> {
    pub fn init(package_name: String, main_file_path: PathBuf) -> Self {
        Self {
            package_name,
            main_file_path,
            output: None,
            _engine: PhantomData,
        }
    }

    pub fn evaluate_program<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<ResolvedValue<F>, CompilerError> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path).map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;

        // Parse the file using leo.pest
        let mut file = ast::parse(&unparsed_file).map_err(|_| CompilerError::FileParsingError)?;

        // Build the abstract syntax tree
        let syntax_tree = ast::File::from_pest(&mut file).map_err(|_| CompilerError::SyntaxTreeError)?;
        log::debug!("{:#?}", syntax_tree);

        // Build program from abstract syntax tree
        let package_name = self.package_name.clone();
        let program = Program::<'_, F>::from(syntax_tree).name(package_name);
        log::info!("Compilation complete:\n{:#?}", program);

        Ok(ResolvedProgram::generate_constraints(cs, program))
    }
}

impl<F: Field + PrimeField> ConstraintSynthesizer<F> for Compiler<F> {
    fn generate_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        self.evaluate_program(cs).expect("error compiling program");
        Ok(())
    }
}
