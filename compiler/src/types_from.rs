//! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.

use crate::{types, Import, ImportSymbol};
use leo_ast::{
    files::File,
    imports::{
        Import as AstImport,
        ImportSymbol as AstImportSymbol,
    },
};
use leo_types::{Circuit, Function, Identifier, TestFunction};

use std::collections::HashMap;

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
                Circuit::from(circuit),
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
