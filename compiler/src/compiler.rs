// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! Compiles a Leo program from a file path.

use crate::{
    constraints::{generate_constraints, generate_test_constraints},
    errors::CompilerError,
    GroupType,
    OutputBytes,
    OutputFile,
};
use leo_ast::{Ast, Input, MainInput, Program};
use leo_grammar::Grammar;
use leo_input::LeoInputParser;
use leo_package::inputs::InputPairs;
use leo_state::verify_local_data_commitment;

use leo_asg::Program as AsgProgram;
use snarkvm_dpc::{base_dpc::instantiated::Components, SystemParameters};
use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem},
};

use sha2::{Digest, Sha256};
use std::{
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

/// Stores information to compile a Leo program.
#[derive(Clone)]
pub struct Compiler<F: Field + PrimeField, G: GroupType<F>> {
    program_name: String,
    main_file_path: PathBuf,
    output_directory: PathBuf,
    program: Program,
    program_input: Input,
    asg: Option<AsgProgram>,
    _engine: PhantomData<F>,
    _group: PhantomData<G>,
}

impl<F: Field + PrimeField, G: GroupType<F>> Compiler<F, G> {
    ///
    /// Returns a new Leo program compiler.
    ///
    pub fn new(package_name: String, main_file_path: PathBuf, output_directory: PathBuf) -> Self {
        Self {
            program_name: package_name.clone(),
            main_file_path,
            output_directory,
            program: Program::new(package_name),
            program_input: Input::new(),
            asg: None,
            _engine: PhantomData,
            _group: PhantomData,
        }
    }

    ///
    /// Returns a new `Compiler` from the given main file path.
    ///
    /// Parses and stores a program from the main file path.
    /// Parses and stores all imported programs.
    /// Performs type inference checking on the program and imported programs.
    ///
    pub fn parse_program_without_input(
        package_name: String,
        main_file_path: PathBuf,
        output_directory: PathBuf,
    ) -> Result<Self, CompilerError> {
        let mut compiler = Self::new(package_name, main_file_path, output_directory);

        compiler.parse_program()?;

        Ok(compiler)
    }

    ///
    /// Returns a new `Compiler` from the given main file path.
    ///
    /// Parses and stores program input from from the input file path and state file path
    /// Parses and stores a program from the main file path.
    /// Parses and stores all imported programs.
    /// Performs type inference checking on the program, imported programs, and program input.
    ///
    pub fn parse_program_with_input(
        package_name: String,
        main_file_path: PathBuf,
        output_directory: PathBuf,
        input_string: &str,
        input_path: &Path,
        state_string: &str,
        state_path: &Path,
    ) -> Result<Self, CompilerError> {
        let mut compiler = Self::new(package_name, main_file_path, output_directory);

        compiler.parse_input(input_string, input_path, state_string, state_path)?;

        compiler.parse_program()?;

        Ok(compiler)
    }

    ///
    /// Parses and stores program input from from the input file path and state file path
    ///
    /// Calls `set_path()` on compiler errors with the given input file path or state file path
    ///
    pub fn parse_input(
        &mut self,
        input_string: &str,
        input_path: &Path,
        state_string: &str,
        state_path: &Path,
    ) -> Result<(), CompilerError> {
        let input_syntax_tree = LeoInputParser::parse_file(&input_string).map_err(|mut e| {
            e.set_path(input_path);

            e
        })?;
        let state_syntax_tree = LeoInputParser::parse_file(&state_string).map_err(|mut e| {
            e.set_path(state_path);

            e
        })?;

        self.program_input.parse_input(input_syntax_tree).map_err(|mut e| {
            e.set_path(input_path);

            e
        })?;
        self.program_input.parse_state(state_syntax_tree).map_err(|mut e| {
            e.set_path(state_path);

            e
        })?;

        Ok(())
    }

    ///
    /// Parses and stores the main program file, constructs a syntax tree, and generates a program.
    ///
    /// Parses and stores all programs imported by the main program file.
    ///
    pub fn parse_program(&mut self) -> Result<(), CompilerError> {
        // Load the program file.
        let program_string = Grammar::load_file(&self.main_file_path)?;

        self.parse_program_from_string(&program_string)
    }

    ///
    /// Equivalent to parse_and_check_program but uses the given program_string instead of a main
    /// file path.
    ///
    pub fn parse_program_from_string(&mut self, program_string: &str) -> Result<(), CompilerError> {
        // Use the parser to construct the pest abstract syntax tree (ast).
        let grammar = Grammar::new(&self.main_file_path, &program_string).map_err(|mut e| {
            e.set_path(&self.main_file_path);

            e
        })?;

        // Construct the AST from the grammar.
        let core_ast = Ast::new(&self.program_name, &grammar)?;

        // Store the main program file.
        self.program = core_ast.into_repr();

        tracing::debug!("Program parsing complete\n{:#?}", self.program);

        // Create a new symbol table from the program, imported_programs, and program_input.
        let asg = leo_asg::InnerProgram::new(&self.program, &mut leo_imports::ImportParser::default())?;

        tracing::debug!("ASG generation complete");

        // Store the ASG.
        self.asg = Some(asg);

        Ok(())
    }

    ///
    /// Synthesizes the circuit with program input to verify correctness.
    ///
    pub fn compile_constraints<CS: ConstraintSystem<F>>(&self, cs: &mut CS) -> Result<OutputBytes, CompilerError> {
        generate_constraints::<F, G, CS>(cs, self.asg.as_ref().unwrap(), &self.program_input).map_err(|mut error| {
            error.set_path(&self.main_file_path);
            error
        })
    }

    ///
    /// Synthesizes the circuit for test functions with program input.
    ///
    pub fn compile_test_constraints(self, input_pairs: InputPairs) -> Result<(u32, u32), CompilerError> {
        generate_test_constraints::<F, G>(
            self.asg.as_ref().unwrap(),
            input_pairs,
            &self.main_file_path,
            &self.output_directory,
        )
    }

    ///
    /// Returns a SHA256 checksum of the program file.
    ///
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

    /// TODO (howardwu): Incorporate this for real program executions and intentionally-real
    ///  test executions. Exclude it for test executions on dummy data.
    ///
    /// Verifies the input to the program.
    ///
    fn verify_local_data_commitment(
        &self,
        system_parameters: &SystemParameters<Components>,
    ) -> Result<bool, CompilerError> {
        let result = verify_local_data_commitment(system_parameters, &self.program_input)?;

        Ok(result)
    }

    ///
    /// Manually sets main function input.
    ///
    /// Used for testing only.
    ///
    pub fn set_main_input(&mut self, input: MainInput) {
        self.program_input.set_main_input(input);
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstraintSynthesizer<F> for Compiler<F, G> {
    ///
    /// Synthesizes the circuit with program input.
    ///
    fn generate_constraints<CS: ConstraintSystem<F>>(&self, cs: &mut CS) -> Result<(), SynthesisError> {
        let output_directory = self.output_directory.clone();
        let package_name = self.program_name.clone();
        let result = self.compile_constraints(cs).map_err(|e| {
            tracing::error!("{}", e);
            SynthesisError::Unsatisfiable
        })?;

        // Write results to file
        let output_file = OutputFile::new(&package_name);
        output_file.write(&output_directory, result.bytes()).unwrap();

        Ok(())
    }
}
