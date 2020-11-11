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

//! A Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of ast statements and expressions.

use crate::{load_annotation, Circuit, Function, FunctionInput, Identifier, ImportStatement, TestFunction};
use leo_grammar::{definitions::Definition, files::File};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Stores the Leo program abstract syntax tree.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub expected_input: Vec<FunctionInput>,
    pub imports: Vec<ImportStatement>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
    pub tests: HashMap<Identifier, TestFunction>,
}

const MAIN_FUNCTION_NAME: &str = "main";

impl<'ast> Program {
    //! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.
    pub fn from(program_name: &str, program_ast: &File<'ast>) -> Self {
        let mut imports = vec![];
        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();
        let mut expected_input = vec![];

        program_ast
            .definitions
            .to_owned()
            .into_iter()
            .for_each(|definition| match definition {
                Definition::Import(import) => imports.push(ImportStatement::from(import)),
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
                    tests.insert(test.function.identifier.clone(), test);
                }
                Definition::Annotated(annotated_definition) => {
                    load_annotation(
                        annotated_definition,
                        &mut imports,
                        &mut circuits,
                        &mut functions,
                        &mut tests,
                        &mut expected_input,
                    );
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
