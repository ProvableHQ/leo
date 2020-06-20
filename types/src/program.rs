//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::{Circuit, Function, FunctionInput, Identifier, Import, TestFunction};
use leo_ast::files::File;

use std::collections::HashMap;

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub expected_inputs: Vec<FunctionInput>,
    pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
    pub tests: HashMap<Identifier, TestFunction>,
}

impl<'ast> Program {
    //! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.
    pub fn from(file: File<'ast>, name: String) -> Self {
        // Compiled ast -> aleo program representation
        let imports = file
            .imports
            .into_iter()
            .map(|import| Import::from(import))
            .collect::<Vec<Import>>();

        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();
        let mut expected_inputs = vec![];

        file.circuits.into_iter().for_each(|circuit| {
            circuits.insert(Identifier::from(circuit.identifier.clone()), Circuit::from(circuit));
        });
        file.functions.into_iter().for_each(|function_def| {
            let function = Function::from(function_def);
            if function.function_name.name.eq("main") {
                expected_inputs = function.inputs.clone();
            }
            functions.insert(function.function_name.clone(), function);
        });
        file.tests.into_iter().for_each(|test_def| {
            let test = TestFunction::from(test_def);
            tests.insert(test.0.function_name.clone(), test);
        });

        Self {
            name,
            expected_inputs,
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
            expected_inputs: vec![],
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
