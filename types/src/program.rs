//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::{Circuit, Function, FunctionInput, Identifier, Import, TestFunction};
use leo_ast::files::File;

use std::collections::HashMap;

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program {
    pub name: Identifier,
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
            functions.insert(
                Identifier::from(function_def.function_name.clone()),
                Function::from(function_def),
            );
        });
        file.tests.into_iter().for_each(|test_def| {
            tests.insert(
                Identifier::from(test_def.function.function_name.clone()),
                TestFunction::from(test_def),
            );
        });

        if let Some(main_function) = functions.get(&Identifier::new("main".into())) {
            expected_inputs = main_function.inputs.clone();
        }

        Self {
            name: Identifier::new(name),
            expected_inputs,
            imports,
            circuits,
            functions,
            tests,
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            name: Identifier::new("".into()),
            expected_inputs: vec![],
            imports: vec![],
            circuits: HashMap::new(),
            functions: HashMap::new(),
            tests: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.name.clone()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Identifier::new(name);
        self
    }
}
