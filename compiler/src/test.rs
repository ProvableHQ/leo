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

use std::{path::{Path, PathBuf}};

use leo_asg::*;
use leo_synthesizer::{CircuitSynthesizer, SerializedCircuit, SummarizedCircuit};
use leo_test_framework::{Test, runner::{Namespace, ParseType, Runner}};
use serde_yaml::Value;
use snarkvm_curves::{bls12_377::Bls12_377, edwards_bls12::Fq};

use crate::{ConstrainedValue, Output, compiler::Compiler, errors::CompilerError, targets::edwards_bls12::EdwardsGroupType};

pub type EdwardsTestCompiler = Compiler<'static, Fq, EdwardsGroupType>;
pub type EdwardsConstrainedValue = ConstrainedValue<'static, Fq, EdwardsGroupType>;

//convenience function for tests, leaks memory
pub(crate) fn make_test_context() -> AsgContext<'static> {
    let allocator = Box::leak(Box::new(new_alloc_context()));
    new_context(allocator)
}

fn new_compiler() -> EdwardsTestCompiler {
    let program_name = "test".to_string();
    let path = PathBuf::from("/test/src/main.leo");
    let output_dir = PathBuf::from("/output/");

    EdwardsTestCompiler::new(program_name, path, output_dir, make_test_context())
}

pub(crate) fn parse_program(program_string: &str) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler();

    compiler.parse_program_from_string(program_string)?;

    Ok(compiler)
}

struct CompileNamespace;

#[derive(serde::Deserialize, serde::Serialize)]
struct OutputItem {
    pub input_file: String,
    pub output: Output,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CompileOutput {
    pub circuit: SummarizedCircuit,
    pub output: Vec<OutputItem>,
}

impl Namespace for CompileNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        let parsed = parse_program(&test.content).map_err(|x| x.to_string())?;

        // (name, content)
        let mut inputs = vec![];

        if let Some(input) = test.config.get("input_file") {
            let input_file: PathBuf = test.path.parent().expect("no test parent dir").into();
            if let Some(name) = input.as_str() {
                let mut input_file = input_file;
                input_file.push(input.as_str().expect("input_file was not a string or array"));
                inputs.push((name.to_string(), std::fs::read_to_string(&input_file).expect("failed to read test input file")));
            } else if let Some(seq) = input.as_sequence() {
                for name in seq {
                    let mut input_file = input_file.clone();
                    input_file.push(name.as_str().expect("input_file was not a string"));
                    inputs.push((name.as_str().expect("input_file item was not a string").to_string(), std::fs::read_to_string(&input_file).expect("failed to read test input file")));
                }
            }
        }
        if inputs.is_empty() {
            inputs.push(("empty".to_string(), "".to_string()));
        }
        
        let state = if let Some(input) = test.config.get("state_file") {
            let mut input_file: PathBuf = test.path.parent().expect("no test parent dir").into();
            input_file.push(input.as_str().expect("state_file was not a string"));
            std::fs::read_to_string(&input_file).expect("failed to read test state file")
        } else {
            "".to_string()
        };
        
        let mut output_items = vec![];
        
        let mut last_circuit = None;
        for input in inputs {
            let mut parsed = parsed.clone();
            parsed.parse_input(&input.1, Path::new("input"), &state, Path::new("state")).map_err(|x| x.to_string())?;
            let mut cs: CircuitSynthesizer<Bls12_377> = Default::default();
            let output = parsed.compile_constraints(&mut cs).map_err(|x| x.to_string())?;
            let circuit: SummarizedCircuit = SerializedCircuit::from(cs).into();
            if let Some(last_circuit) = last_circuit.as_ref() {
                if last_circuit != &circuit {
                    eprintln!("{}\n{}", serde_yaml::to_string(last_circuit).unwrap(), serde_yaml::to_string(&circuit).unwrap());
                    return Err("circuit changed on different input files".to_string());
                }
            } else {
                last_circuit = Some(circuit);
            }
            output_items.push(OutputItem {
                input_file: input.0,
                output,
            });
        }
        


        
        let final_output = CompileOutput {
            circuit: last_circuit.unwrap(),
            output: output_items,
        };
        Ok(serde_yaml::to_value(&final_output).expect("serialization failed"))
    }
}

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "Compile" => Box::new(CompileNamespace),
            _ => return None,
        })
    }
}

#[test]
pub fn compiler_tests() {
    leo_test_framework::run_tests(&TestRunner, "compiler");
}
