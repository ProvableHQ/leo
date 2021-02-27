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

use crate::{asg::Asg, ast::Ast};
use leo_compiler::generate_constraints;
// use leo_synthesizer::CircuitSynthesizer;

// use snarkvm_curves::Bls12_377;

use std::path::Path;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Compiler(String);

#[wasm_bindgen]
impl Compiler {
    #[wasm_bindgen(constructor)]
    pub fn new(filepath: &str, program_name: &str, program_string: &str) -> Self {
        let ast = Ast::new(filepath, program_name, program_string).unwrap();
        let asg = Asg::from(&ast);

        // Generate the program on the constraint system and verify correctness
        {
            // let mut cs = CircuitSynthesizer::<Bls12_377> {
            //     at: vec![],
            //     bt: vec![],
            //     ct: vec![],
            //     input_assignment: vec![],
            //     aux_assignment: vec![],
            // };
            // let temporary_program = program.clone();
            // let output = temporary_program.compile_constraints(&mut cs)?;
            //
            // tracing::debug!("Compiled output - {:#?}", output);
            // tracing::info!("Number of constraints - {:#?}", cs.num_constraints());
            //
            // // Serialize the circuit
            // let circuit_object = SerializedCircuit::from(cs);
            // let json = circuit_object.to_json_string().unwrap();
            // // println!("json: {}", json);
            //
            // // Write serialized circuit to circuit `.json` file.
            // let circuit_file = CircuitFile::new(&package_name);
            // circuit_file.write_to(&path, json)?;

            // Check that we can read the serialized circuit file
            // let serialized = circuit_file.read_from(&package_path)?;

            // Deserialize the circuit
            // let deserialized = SerializedCircuit::from_json_string(&serialized).unwrap();
            // let _circuit_synthesizer = CircuitSynthesizer::<Bls12_377>::try_from(deserialized).unwrap();
            // println!("deserialized {:?}", circuit_synthesizer.num_constraints());
        }

        // TODO (howardwu): Support program inputs.
        generate_constraints(cs, &asg.as_ref().unwrap(), &Input::new()).unwrap();
        Self(ast.to_string())
    }

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn ast_test() {}
}
