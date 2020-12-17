// Copyright (C) 2019-2020 Aleo Systems Inc.
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

//! Generates R1CS constraints for a compiled Leo program.

use crate::{
    errors::CompilerError,
    new_scope,
    ConstrainedProgram,
    ConstrainedValue,
    GroupType,
    OutputBytes,
    OutputFile,
};
use leo_ast::{Input, Program};
use leo_imports::ImportParser;
use leo_input::LeoInputParser;
use leo_package::inputs::InputPairs;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSystem, TestConstraintSystem},
};
use std::path::Path;

pub fn generate_constraints<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    program: Program,
    input: Input,
    imported_programs: &ImportParser,
) -> Result<OutputBytes, CompilerError> {
    let mut resolved_program = ConstrainedProgram::<F, G>::new();
    let program_name = program.get_name();
    let main_function_name = new_scope(&program_name, "main");

    resolved_program.store_definitions(&program, imported_programs)?;

    let main = resolved_program.get(&main_function_name).ok_or(CompilerError::NoMain)?;

    match main.clone() {
        ConstrainedValue::Function(_circuit_identifier, function) => {
            let result = resolved_program.enforce_main_function(cs, &program_name, *function, input)?;
            Ok(result)
        }
        _ => Err(CompilerError::NoMainFunction),
    }
}

pub fn generate_test_constraints<F: Field + PrimeField, G: GroupType<F>>(
    program: Program,
    input: InputPairs,
    imported_programs: &ImportParser,
    main_file_path: &Path,
    output_directory: &Path,
) -> Result<(u32, u32), CompilerError> {
    let mut resolved_program = ConstrainedProgram::<F, G>::new();
    let program_name = program.get_name();

    let tests = program.tests.clone();

    // Store definitions
    resolved_program.store_definitions(&program, imported_programs)?;

    // Get default input
    let default = input.pairs.get(&program_name);

    tracing::info!("Running {} tests", tests.len());

    // Count passed and failed tests
    let mut passed = 0;
    let mut failed = 0;

    for (test_name, test) in tests.into_iter() {
        let cs = &mut TestConstraintSystem::<F>::new();
        let full_test_name = format!("{}::{}", program_name.clone(), test_name);
        let mut output_file_name = program_name.clone();

        // get input file name from annotation or use test_name
        let input_pair = match test.input_file {
            Some(file_id) => {
                let file_name = file_id.name;

                output_file_name = file_name.clone();

                match input.pairs.get(&file_name) {
                    Some(pair) => pair.to_owned(),
                    None => return Err(CompilerError::InvalidTestContext(file_name)),
                }
            }
            None => default.ok_or(CompilerError::NoTestInput)?,
        };

        // parse input files to abstract syntax trees
        let input_file = &input_pair.input_file;
        let state_file = &input_pair.state_file;

        let input_ast = LeoInputParser::parse_file(input_file)?;
        let state_ast = LeoInputParser::parse_file(state_file)?;

        // parse input files into input struct
        let mut input = Input::new();
        input.parse_input(input_ast)?;
        input.parse_state(state_ast)?;

        // run test function on new program with input
        let result = resolved_program.enforce_main_function(
            cs,
            &program_name,
            test.function,
            input, // pass program input into every test
        );

        match (result.is_ok(), cs.is_satisfied()) {
            (true, true) => {
                tracing::info!("{} ... ok\n", full_test_name);

                // write result to file
                let output = result?;
                let output_file = OutputFile::new(&output_file_name);

                output_file.write(output_directory, output.bytes()).unwrap();

                // increment passed tests
                passed += 1;
            }
            (true, false) => {
                tracing::error!("{} constraint system not satisfied\n", full_test_name);

                // increment failed tests
                failed += 1;
            }
            (false, _) => {
                // Set file location of error
                let mut error = result.unwrap_err();
                error.set_path(main_file_path);

                tracing::error!("{} failed due to error\n\n{}\n", full_test_name, error);

                // increment failed tests
                failed += 1;
            }
        }
    }

    Ok((passed, failed))
}
