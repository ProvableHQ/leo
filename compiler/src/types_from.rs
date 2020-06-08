//! Logic to convert from an abstract syntax tree (ast) representation to a Leo program.

use crate::{types, Import, ImportSymbol};
use leo_ast::{
    circuits::{
        Circuit,
        CircuitFieldDefinition as AstCircuitFieldDefinition,
        CircuitFunction,
        CircuitMember
    },
    common::{
        Visibility,
        Private,
    },
    files::File,
    functions::{
        Function,
        FunctionInput,
        TestFunction
    },
    imports::{
        Import as AstImport,
        ImportSymbol as AstImportSymbol,
    },
};
use leo_types::{Identifier, Statement, Type};

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
            types::Function::from(circuit_function.function),
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

/// pest ast -> function types::Parameters

impl<'ast> From<FunctionInput<'ast>> for types::InputModel {
    fn from(parameter: FunctionInput<'ast>) -> Self {
        types::InputModel {
            identifier: Identifier::from(parameter.identifier),
            mutable: parameter.mutable.is_some(),
            // private by default
            private: parameter.visibility.map_or(true, |visibility| {
                visibility.eq(&Visibility::Private(Private {}))
            }),
            _type: Type::from(parameter._type),
        }
    }
}

/// pest ast -> types::Function

impl<'ast> From<Function<'ast>> for types::Function {
    fn from(function_definition: Function<'ast>) -> Self {
        let function_name = Identifier::from(function_definition.function_name);
        let parameters = function_definition
            .parameters
            .into_iter()
            .map(|parameter| types::InputModel::from(parameter))
            .collect();
        let returns = function_definition
            .returns
            .into_iter()
            .map(|return_type| Type::from(return_type))
            .collect();
        let statements = function_definition
            .statements
            .into_iter()
            .map(|statement| Statement::from(statement))
            .collect();

        types::Function {
            function_name,
            inputs: parameters,
            returns,
            statements,
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

/// pest ast -> Test
impl<'ast> From<TestFunction<'ast>> for types::Test {
    fn from(test: TestFunction) -> Self {
        types::Test(types::Function::from(test.function))
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
                types::Function::from(function_def),
            );
        });
        file.tests.into_iter().for_each(|test_def| {
            tests.insert(
                Identifier::from(test_def.function.function_name.clone()),
                types::Test::from(test_def),
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
