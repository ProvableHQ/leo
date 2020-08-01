//! Compiles a Leo program from a file path.

use crate::{
    constraints::{generate_constraints, generate_test_constraints},
    errors::CompilerError,
    value::ConstrainedValue,
    GroupType,
    ImportParser,
};
use leo_ast::LeoAst;
use leo_inputs::LeoInputsParser;
use leo_typed::{InputValue, Inputs, LeoTypedAst, Program};

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
    imported_programs: ImportParser,
    output: Option<ConstrainedValue<F, G>>,
    _engine: PhantomData<F>,
}

impl<F: Field + PrimeField, G: GroupType<F>> Compiler<F, G> {
    pub fn new(package_name: String) -> Self {
        Self {
            package_name: package_name.clone(),
            main_file_path: PathBuf::new(),
            program: Program::new(package_name),
            program_inputs: Inputs::new(),
            imported_programs: ImportParser::new(),
            output: None,
            _engine: PhantomData,
        }
    }

    pub fn new_from_path(package_name: String, main_file_path: PathBuf) -> Result<Self, CompilerError> {
        let mut compiler = Self::new(package_name);
        compiler.set_path(main_file_path);

        // Generate the abstract syntax tree and assemble the program.
        compiler.parse_program()?;

        Ok(compiler)
    }

    pub fn set_path(&mut self, main_file_path: PathBuf) {
        self.main_file_path = main_file_path
    }

    pub fn set_inputs(&mut self, program_inputs: Vec<Option<InputValue>>) {
        self.program_inputs.set_inputs(program_inputs);
    }

    pub fn checksum(&self) -> Result<String, CompilerError> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;

        // Hash the file contents
        let mut hasher = Sha256::new();
        hasher.update(unparsed_file.as_bytes());
        let hash = hasher.finalize();

        Ok(hex::encode(hash))
    }

    pub fn compile_constraints<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<ConstrainedValue<F, G>, CompilerError> {
        let path = self.main_file_path;
        let inputs = self.program_inputs.get_inputs();

        generate_constraints(cs, self.program, inputs, &self.imported_programs).map_err(|mut error| {
            error.set_path(path);

            error
        })
    }

    pub fn compile_test_constraints(self, cs: &mut TestConstraintSystem<F>) -> Result<(), CompilerError> {
        generate_test_constraints::<F, G>(cs, self.program, &self.imported_programs)
    }

    /// Parses the Leo program file, constructs a syntax tree, and generates a program.
    pub fn parse_program(&mut self) -> Result<(), CompilerError> {
        // Use the parser to construct the abstract syntax tree.
        let program_string = LeoAst::load_file(&self.main_file_path)?;
        let ast = LeoAst::new(&self.main_file_path, &program_string)?;

        // Derive the package name.
        let package_name = self.package_name.clone();

        // Use the typed parser to construct the typed syntax tree.
        let typed_tree = LeoTypedAst::new(&package_name, &ast);

        self.program = typed_tree.into_repr();
        self.program_inputs.set_inputs_size(self.program.expected_inputs.len());
        self.imported_programs = ImportParser::parse(&self.program)?;

        log::debug!("Program parsing complete\n{:#?}", self.program);

        Ok(())
    }

    /// Parses the Leo program string, constructs a syntax tree, and generates a program.
    /// Used for testing only.
    pub fn parse_program_from_string(&mut self, program_string: &str) -> Result<(), CompilerError> {
        // Use the given bytes to construct the abstract syntax tree.
        let ast = LeoAst::new(&self.main_file_path, &program_string)?;

        // Derive the package name.
        let package_name = self.package_name.clone();

        // Use the typed parser to construct the typed syntax tree.
        let typed_tree = LeoTypedAst::new(&package_name, &ast);

        self.program = typed_tree.into_repr();
        self.program_inputs.set_inputs_size(self.program.expected_inputs.len());
        self.imported_programs = ImportParser::parse(&self.program)?;

        log::debug!("Program parsing complete\n{:#?}", self.program);

        Ok(())
    }

    pub fn parse_inputs(&mut self, inputs_string: &str) -> Result<(), CompilerError> {
        let syntax_tree = LeoInputsParser::parse_file(&inputs_string)?;

        // Check number/order of parameters here
        self.program_inputs = Inputs::from_inputs_file(syntax_tree, self.program.expected_inputs.clone())?;

        Ok(())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CompilerError> {
        Ok(bincode::serialize(&self.program)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CompilerError> {
        let program: Program = bincode::deserialize(bytes)?;
        let mut program_inputs = Inputs::new();

        program_inputs.set_inputs_size(program.expected_inputs.len());

        Ok(Self {
            package_name: program.name.clone(),
            main_file_path: PathBuf::new(),
            program,
            program_inputs,
            imported_programs: ImportParser::new(),
            output: None,
            _engine: PhantomData,
        })
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstraintSynthesizer<F> for Compiler<F, G> {
    fn generate_constraints<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let result = generate_constraints::<_, G, _>(
            cs,
            self.program,
            self.program_inputs.get_inputs(),
            &self.imported_programs,
        )
        .map_err(|e| {
            log::error!("{}", e);
            SynthesisError::Unsatisfiable
        })?;

        // Write results to file or something
        log::info!("{}", result);

        Ok(())
    }
}
