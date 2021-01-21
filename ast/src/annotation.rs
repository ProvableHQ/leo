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

use crate::{Circuit, Function, FunctionInput, Identifier, ImportStatement, TestFunction};
use leo_grammar::{
    annotations::{Annotation, AnnotationArguments, AnnotationName},
    definitions::{AnnotatedDefinition, Definition},
};

use indexmap::IndexMap;

pub fn load_annotation(
    annotated_definition: AnnotatedDefinition,
    _imports: &mut Vec<ImportStatement>,
    _circuits: &mut IndexMap<Identifier, Circuit>,
    _functions: &mut IndexMap<Identifier, Function>,
    tests: &mut IndexMap<Identifier, TestFunction>,
    _expected: &mut Vec<FunctionInput>,
) {
    let ast_annotation = annotated_definition.annotation;
    let ast_definition = *annotated_definition.definition;

    match ast_definition {
        Definition::Import(_) => unimplemented!("annotated imports are not supported yet"),
        Definition::Circuit(_) => unimplemented!("annotated circuits are not supported yet"),
        // TODO need someone to take functions annotated with @test to be moved from function to tests.
        Definition::Function(function) => match ast_annotation.name {
            AnnotationName::Test(_) | AnnotationName::TestWithContext(_) => {
                let ident = Identifier::from(function.identifier.clone());
                _functions.remove(&ident.clone());

                let test_function = leo_grammar::functions::TestFunction::from(function);
                let test = TestFunction::from(test_function);
                tests.insert(ident, test.clone());

                load_annotated_test(test, ast_annotation, tests);
            }
            _ => unimplemented!("annotated functions are not supported yet"),
        },
        Definition::Annotated(_) => unimplemented!("nested annotations are not supported yet"),
    }
}

pub fn load_annotated_test(test: TestFunction, annotation: Annotation, tests: &mut IndexMap<Identifier, TestFunction>) {
    let name = annotation.name;
    let ast_arguments = annotation.arguments;

    match name {
        AnnotationName::Test(_) => (),
        AnnotationName::TestWithContext(_) if ast_arguments.is_some() => {
            load_annotated_test_context(test, ast_arguments.unwrap(), tests)
        }
        AnnotationName::TestWithContext(_) if ast_arguments.is_some() => {
            load_annotated_test_context(test, ast_arguments.unwrap(), tests)
        }
        AnnotationName::Context(_) if ast_arguments.is_some() => {
            load_annotated_test_context(test, ast_arguments.unwrap(), tests)
        }
        _ => (),
    }
}

pub fn load_annotated_test_context(
    mut test: TestFunction,
    ast_arguments: AnnotationArguments,
    tests: &mut IndexMap<Identifier, TestFunction>,
) {
    let arguments = ast_arguments.arguments;

    if arguments.len() != 1 {
        panic!("text context annotation must have one argument identifier")
    }

    let ast_input_file = arguments[0].to_owned();
    let input_file = Identifier::from(ast_input_file);

    test.input_file = Some(input_file);

    tests.insert(test.function.identifier.clone(), test);
}
