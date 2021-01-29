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

use crate::{
    load_annotation,
    Circuit,
    Define,
    DeprecatedError,
    Function,
    FunctionInput,
    Identifier,
    ImportStatement,
    TestFunction,
};
use leo_grammar::{definitions::Definition, files::File};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
/// Stores the Leo program abstract syntax tree.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub expected_input: Vec<FunctionInput>,
    pub imports: Vec<ImportStatement>,
    pub circuits: IndexMap<Identifier, Circuit>,
    pub defines: IndexMap<Identifier, Define>,
    pub functions: IndexMap<Identifier, Function>,
    pub tests: IndexMap<Identifier, TestFunction>,
}

const MAIN_FUNCTION_NAME: &str = "main";

impl<'ast> Program {
    //! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.
    pub fn from(program_name: &str, program_ast: &File<'ast>) -> Result<Self, DeprecatedError> {
        let mut imports = vec![];
        let mut circuits = IndexMap::new();
        let mut defines = IndexMap::new();
        let mut functions = IndexMap::new();
        let mut tests = IndexMap::new();
        let mut expected_input = vec![];

        program_ast
            .definitions
            .to_owned()
            .into_iter()
            // Use of Infallible to say we never expect an Some(Ok(...))
            .find_map::<Result<std::convert::Infallible, _>, _>(|definition| match definition {
                Definition::Import(import) => {
                    imports.push(ImportStatement::from(import));
                    None
                }
                Definition::Circuit(circuit) => {
                    circuits.insert(Identifier::from(circuit.identifier.clone()), Circuit::from(circuit));
                    None
                }
                Definition::Define(define) => {
                    defines.insert(Identifier::from(define.identifier.clone()), Define::from(define));
                    None
                }
                Definition::Function(function_def) => {
                    let function = Function::from(function_def);
                    if function.identifier.name.eq(MAIN_FUNCTION_NAME) {
                        expected_input = function.input.clone();
                    }
                    functions.insert(function.identifier.clone(), function);
                    None
                }
                Definition::Deprecated(deprecated) => {
                    Some(Err(DeprecatedError::from(deprecated)))
                }
                Definition::Annotated(annotated_definition) => {
                    let loaded_annotation = load_annotation(
                        annotated_definition,
                        &mut imports,
                        &mut circuits,
                        &mut functions,
                        &mut tests,
                        &mut expected_input,
                    );

                    match loaded_annotation {
                        Ok(_) => None,
                        Err(deprecated_err) => Some(Err(deprecated_err))
                    }
                }
            })
            .transpose()?;

        Ok(Self {
            name: program_name.to_string(),
            expected_input,
            imports,
            circuits,
            defines,
            functions,
            tests,
        })
    }
}

impl Program {
    pub fn new(name: String) -> Self {
        Self {
            name,
            expected_input: vec![],
            imports: vec![],
            circuits: IndexMap::new(),
            defines: IndexMap::new(),
            functions: IndexMap::new(),
            tests: IndexMap::new(),
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
