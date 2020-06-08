//! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.

use crate::{types, Import, ImportSymbol};
use leo_ast::{
    circuits::{
        Circuit,
        CircuitFieldDefinition as AstCircuitFieldDefinition,
        CircuitFunction,
        CircuitMember
    },
    files::File,
    imports::{
        Import as AstImport,
        ImportSymbol as AstImportSymbol,
    },
};
use leo_types::{Function, Identifier, TestFunction, Type};

use std::collections::HashMap;

/// pest ast -> types::Circuit

impl<'ast> From<AstCircuitFieldDefinition<'ast>> for types::CircuitMember {
    fn from(circuit_value: AstCircuitFieldDefinition<'ast>) -> Self {
        types::CircuitMember::CircuitField(
            Identifier::from(circuit_value.identifier),
            Type::from(circuit_value._type),
        )
    }
}

impl<'ast> From<CircuitFunction<'ast>> for types::CircuitMember {
    fn from(circuit_function: CircuitFunction<'ast>) -> Self {
        types::CircuitMember::CircuitFunction(
            circuit_function._static.is_some(),
            Function::from(circuit_function.function),
        )
    }
}

impl<'ast> From<CircuitMember<'ast>> for types::CircuitMember {
    fn from(object: CircuitMember<'ast>) -> Self {
        match object {
            CircuitMember::CircuitFieldDefinition(circuit_value) => {
                types::CircuitMember::from(circuit_value)
            }
            CircuitMember::CircuitFunction(circuit_function) => {
                types::CircuitMember::from(circuit_function)
            }
        }
    }
}

impl<'ast> From<Circuit<'ast>> for types::Circuit {
    fn from(circuit: Circuit<'ast>) -> Self {
        let variable = Identifier::from(circuit.identifier);
        let members = circuit
            .members
            .into_iter()
            .map(|member| types::CircuitMember::from(member))
            .collect();

        types::Circuit {
            identifier: variable,
            members,
        }
    }
}

/// pest ast -> Import

impl<'ast> From<AstImportSymbol<'ast>> for ImportSymbol {
    fn from(symbol: AstImportSymbol<'ast>) -> Self {
        ImportSymbol {
            symbol: Identifier::from(symbol.value),
            alias: symbol.alias.map(|alias| Identifier::from(alias)),
        }
    }
}

impl<'ast> From<AstImport<'ast>> for Import {
    fn from(import: AstImport<'ast>) -> Self {
        Import {
            path_string: import.source.value,
            symbols: import
                .symbols
                .into_iter()
                .map(|symbol| ImportSymbol::from(symbol))
                .collect(),
        }
    }
}

/// pest ast -> types::Program

impl<'ast> types::Program {
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
        let mut num_parameters = 0usize;

        file.circuits.into_iter().for_each(|circuit| {
            circuits.insert(
                Identifier::from(circuit.identifier.clone()),
                types::Circuit::from(circuit),
            );
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
            num_parameters = main_function.inputs.len();
        }

        types::Program {
            name: Identifier::new(name),
            num_parameters,
            imports,
            circuits,
            functions,
            tests,
        }
    }
}
