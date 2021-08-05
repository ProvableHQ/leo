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

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use leo_asg::*;
use leo_ast::{Ast, Program};
use leo_synthesizer::{CircuitSynthesizer, SerializedCircuit, SummarizedCircuit};
use leo_test_framework::{
    runner::{Namespace, ParseType, Runner},
    Test,
};
use serde_yaml::Value;
use snarkvm_curves::{bls12_377::Bls12_377, edwards_bls12::Fq};

use crate::{
    compiler::Compiler, errors::CompilerError, targets::edwards_bls12::EdwardsGroupType, AstSnapshotOptions, Output,
};

pub type EdwardsTestCompiler = Compiler<'static, Fq, EdwardsGroupType>;
// pub type EdwardsConstrainedValue = ConstrainedValue<'static, Fq, EdwardsGroupType>;

//convenience function for tests, leaks memory
pub(crate) fn make_test_context() -> AsgContext<'static> {
    let allocator = Box::leak(Box::new(new_alloc_context()));
    new_context(allocator)
}

fn new_compiler(path: PathBuf, theorem_options: Option<AstSnapshotOptions>) -> EdwardsTestCompiler {
    let program_name = "test".to_string();
    let output_dir = PathBuf::from("/tmp/output/");
    std::fs::create_dir_all(output_dir.clone()).unwrap();

    EdwardsTestCompiler::new(
        program_name,
        path,
        output_dir,
        make_test_context(),
        None,
        HashMap::new(),
        theorem_options,
    )
}

fn hash(input: String) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let output = hasher.finalize();
    hex::encode(&output[..])
}

pub(crate) fn parse_program(
    program_string: &str,
    theorem_options: Option<AstSnapshotOptions>,
    cwd: Option<PathBuf>,
) -> Result<EdwardsTestCompiler, CompilerError> {
    let mut compiler = new_compiler(cwd.unwrap_or("compiler-test".into()), theorem_options);

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
    pub initial_ast: String,
    pub canonicalized_ast: String,
    pub type_inferenced_ast: String,
}

impl Namespace for CompileNamespace {
    fn parse_type(&self) -> ParseType {
        ParseType::Whole
    }

    fn run_test(&self, test: Test) -> Result<Value, String> {
        // Check for CWD option:
        // ``` cwd: import ```
        // When set, uses different working directory for current file.
        // If not, uses file path as current working directory.
        let cwd = test.config.get("cwd").map(|val| {
            let mut cwd = test.path.clone();
            cwd.pop();
            cwd.join(&val.as_str().unwrap())
        });
        // .unwrap_or(test.path.clone());

        let parsed = parse_program(
            &test.content,
            Some(AstSnapshotOptions {
                initial: true,
                canonicalized: true,
                type_inferenced: true,
            }),
            cwd,
        )
        .map_err(|x| x.to_string())?;

        // (name, content)
        let mut inputs = vec![];

        if let Some(Value::Sequence(field)) = test.config.get("inputs") {
            for map in field {
                for (name, value) in map.as_mapping().unwrap().iter() {
                    // Try to parse string from 'inputs' map, else fail
                    let value = if let serde_yaml::Value::String(value) = value {
                        value
                    } else {
                        return Err("Expected string in 'inputs' map".to_string());
                    };

                    inputs.push((name.as_str().unwrap().to_string(), value.clone()));
                }
            }
        }

        if let Some(input) = test.config.get("input_file") {
            let input_file: PathBuf = test.path.parent().expect("no test parent dir").into();
            if let Some(name) = input.as_str() {
                let mut input_file = input_file;
                input_file.push(input.as_str().expect("input_file was not a string or array"));
                inputs.push((
                    name.to_string(),
                    std::fs::read_to_string(&input_file).expect("failed to read test input file"),
                ));
            } else if let Some(seq) = input.as_sequence() {
                for name in seq {
                    let mut input_file = input_file.clone();
                    input_file.push(name.as_str().expect("input_file was not a string"));
                    inputs.push((
                        name.as_str().expect("input_file item was not a string").to_string(),
                        std::fs::read_to_string(&input_file).expect("failed to read test input file"),
                    ));
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
            parsed
                .parse_input(&input.1, Path::new("input"), &state, Path::new("state"))
                .map_err(|x| x.to_string())?;
            let mut cs: CircuitSynthesizer<Bls12_377> = Default::default();
            let output = parsed.compile_constraints(&mut cs).map_err(|x| x.to_string())?;
            let circuit: SummarizedCircuit = SerializedCircuit::from(cs).into();

            if circuit.num_constraints == 0 {
                return Err(
                    "- Circuit has no constraints, use inputs and registers in program to produce them".to_string(),
                );
            }

            if let Some(last_circuit) = last_circuit.as_ref() {
                if last_circuit != &circuit {
                    eprintln!(
                        "{}\n{}",
                        serde_yaml::to_string(last_circuit).unwrap(),
                        serde_yaml::to_string(&circuit).unwrap()
                    );
                    return Err("- Circuit changed on different input files".to_string());
                }
            } else {
                last_circuit = Some(circuit);
            }

            output_items.push(OutputItem {
                input_file: input.0,
                output,
            });
        }

        let initial_ast: String = hash(
            Ast::from_json_file("/tmp/output/initial_ast.json".into())
                .unwrap_or_else(|_| Ast::new(Program::new("Error reading initial theorem.".to_string())))
                .to_json_string()
                .unwrap_or_else(|_| "Error converting ast to string.".to_string()),
        );
        let canonicalized_ast: String = hash(
            Ast::from_json_file("/tmp/output/canonicalization_ast.json".into())
                .unwrap_or_else(|_| Ast::new(Program::new("Error reading canonicalized theorem.".to_string())))
                .to_json_string()
                .unwrap_or_else(|_| "Error converting ast to string.".to_string()),
        );
        let type_inferenced_ast = hash(
            Ast::from_json_file("/tmp/output/type_inferenced_ast.json".into())
                .unwrap_or_else(|_| Ast::new(Program::new("Error reading type inferenced theorem.".to_string())))
                .to_json_string()
                .unwrap_or_else(|_| "Error converting ast to string.".to_string()),
        );

        if std::fs::read_dir("/tmp/output").is_ok() {
            std::fs::remove_dir_all(std::path::Path::new("/tmp/output")).expect("Error failed to clean up output dir.");
        }

        let final_output = CompileOutput {
            circuit: last_circuit.unwrap(),
            output: output_items,
            initial_ast,
            canonicalized_ast,
            type_inferenced_ast,
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
