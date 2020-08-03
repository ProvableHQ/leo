//! Compiles a Leo program from a file path.

use crate::{
    constraints::{generate_constraints, generate_test_constraints},
    errors::CompilerError,
    GroupType,
    ImportParser,
    OutputBytes,
    OutputsFile,
};
use leo_ast::LeoAst;
use leo_inputs::LeoInputsParser;
use leo_types::{Inputs, LeoTypedAst, MainInputs, Program};

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
    outputs_directory: PathBuf,
    program: Program,
    program_inputs: Inputs,
    imported_programs: ImportParser,
    _engine: PhantomData<F>,
    _group: PhantomData<G>,
}

impl<F: Field + PrimeField, G: GroupType<F>> Compiler<F, G> {
    pub fn new(package_name: String, main_file_path: PathBuf, outputs_directory: PathBuf) -> Self {
        Self {
            package_name: package_name.clone(),
            main_file_path,
            outputs_directory,
            program: Program::new(package_name),
            program_inputs: Inputs::new(),
            imported_programs: ImportParser::new(),
            _engine: PhantomData,
            _group: PhantomData,
        }
    }

    /// Parse the input and state files.
    /// Stores a typed ast of all inputs to the program.
    pub fn parse_inputs(&mut self, inputs_string: &str, state_string: &str) -> Result<(), CompilerError> {
        let inputs_syntax_tree = LeoInputsParser::parse_file(&inputs_string)?;
        let state_syntax_tree = LeoInputsParser::parse_file(&state_string)?;

        self.program_inputs.parse_inputs(inputs_syntax_tree)?;
        self.program_inputs.parse_state(state_syntax_tree)?;

        Ok(())
    }

    /// Parses program files.
    /// Returns a compiler struct that stores the typed program abstract syntax trees (ast).
    pub fn parse_program_without_inputs(
        package_name: String,
        main_file_path: PathBuf,
        outputs_directory: PathBuf,
    ) -> Result<Self, CompilerError> {
        let mut compiler = Self::new(package_name, main_file_path, outputs_directory);

        compiler.parse_program()?;

        Ok(compiler)
    }

    /// Parses input, state, and program files.
    /// Returns a compiler struct that stores the typed inputs and typed program abstract syntax trees (ast).
    pub fn parse_program_with_inputs(
        package_name: String,
        main_file_path: PathBuf,
        outputs_directory: PathBuf,
        inputs_string: &str,
        state_string: &str,
    ) -> Result<Self, CompilerError> {
        let mut compiler = Self::new(package_name, main_file_path, outputs_directory);

        compiler.parse_inputs(inputs_string, state_string)?;
        compiler.parse_program()?;

        Ok(compiler)
    }

    /// Parses the Leo program file, constructs a syntax tree, and generates a program.
    pub fn parse_program(&mut self) -> Result<(), CompilerError> {
        // Use the parser to construct the abstract syntax tree.
        let program_string = LeoAst::load_file(&self.main_file_path)?;

        self.parse_program_from_string(&program_string)
    }

    /// Parses the Leo program string, constructs a syntax tree, and generates a program.
    /// Used for testing only.
    #[deprecated(note = "Please use the 'parse_program' method instead.")]
    pub fn parse_program_from_string(&mut self, program_string: &str) -> Result<(), CompilerError> {
        // Use the given bytes to construct the abstract syntax tree.
        let ast = LeoAst::new(&self.main_file_path, &program_string)?;

        // Derive the package name.
        let package_name = self.package_name.clone();

        // Use the typed parser to construct the typed syntax tree.
        let typed_tree = LeoTypedAst::new(&package_name, &ast);

        self.program = typed_tree.into_repr();
        self.imported_programs = ImportParser::parse(&self.program)?;

        log::debug!("Program parsing complete\n{:#?}", self.program);

        Ok(())
    }

    /// Manually sets main function inputs
    pub fn set_main_inputs(&mut self, inputs: MainInputs) {
        self.program_inputs.set_main_inputs(inputs);
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

    /// Synthesizes the circuit without program inputs to verify correctness.
    pub fn compile_constraints<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<OutputBytes, CompilerError> {
        let path = self.main_file_path;
        let inputs = self.program_inputs.empty();

        generate_constraints::<F, G, CS>(cs, self.program, inputs, &self.imported_programs).map_err(|mut error| {
            error.set_path(path);

            error
        })
    }

    /// Synthesizes the circuit for test functions with program inputs.
    pub fn compile_test_constraints(self, cs: &mut TestConstraintSystem<F>) -> Result<(), CompilerError> {
        generate_test_constraints::<F, G>(cs, self.program, self.program_inputs, &self.imported_programs)
    }

    /// Calls the internal generate_constraints method with arguments
    pub fn generate_constraints_helper<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<OutputBytes, CompilerError> {
        let path = self.main_file_path;
        generate_constraints::<_, G, _>(cs, self.program, self.program_inputs, &self.imported_programs).map_err(
            |mut error| {
                error.set_path(path);
                error
            },
        )
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CompilerError> {
        Ok(bincode::serialize(&self.program)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CompilerError> {
        let program: Program = bincode::deserialize(bytes)?;
        let program_inputs = Inputs::new();

        Ok(Self {
            package_name: program.name.clone(),
            main_file_path: PathBuf::new(),
            outputs_directory: PathBuf::new(),
            program,
            program_inputs,
            imported_programs: ImportParser::new(),
            _engine: PhantomData,
            _group: PhantomData,
        })
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstraintSynthesizer<F> for Compiler<F, G> {
    /// Synthesizes the circuit with program inputs.
    fn generate_constraints<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let outputs_directory = self.outputs_directory.clone();
        let package_name = self.package_name.clone();
        let result = self.generate_constraints_helper(cs).map_err(|e| {
            log::error!("{}", e);
            SynthesisError::Unsatisfiable
        })?;

        log::info!("Program circuit successfully synthesized!");

        // Write results to file
        let outputs_file = OutputsFile::new(&package_name);
        outputs_file.write(&outputs_directory, result.bytes()).unwrap();

        Ok(())
    }
}
