//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::{Circuit, Function, Identifier, Import, InputVariable, TestFunction};
use leo_ast::{definitions::Definition, files::File};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub expected_input: Vec<InputVariable>,
    pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
    pub tests: HashMap<Identifier, TestFunction>,
}

const MAIN_FUNCTION_NAME: &str = "main";

impl<'ast> Program {
    //! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.
    pub fn from(program_name: &str, program_ast: &File<'ast>) -> Self {
        // Compiled ast -> aleo program representation
        let imports = program_ast
            .imports
            .to_owned()
            .into_iter()
            .map(|import| Import::from(import))
            .collect::<Vec<Import>>();

        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();
        let mut expected_input = vec![];

        program_ast
            .definitions
            .to_owned()
            .into_iter()
            .for_each(|definition| match definition {
                Definition::Circuit(circuit) => {
                    circuits.insert(Identifier::from(circuit.identifier.clone()), Circuit::from(circuit));
                }
                Definition::Function(function_def) => {
                    let function = Function::from(function_def);
                    if function.identifier.name.eq(MAIN_FUNCTION_NAME) {
                        expected_input = function.input.clone();
                    }
                    functions.insert(function.identifier.clone(), function);
                }
                Definition::TestFunction(test_def) => {
                    let test = TestFunction::from(test_def);
                    tests.insert(test.0.identifier.clone(), test);
                }
            });

        Self {
            name: program_name.to_string(),
            expected_input,
            imports,
            circuits,
            functions,
            tests,
        }
    }
}

impl Program {
    pub fn new(name: String) -> Self {
        Self {
            name,
            expected_input: vec![],
            imports: vec![],
            circuits: HashMap::new(),
            functions: HashMap::new(),
            tests: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
