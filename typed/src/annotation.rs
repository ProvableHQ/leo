use crate::{Circuit, Function, Identifier, Import, InputVariable, TestFunction};
use leo_ast::{
    annotations::{Annotation, AnnotationArguments, AnnotationName},
    definitions::{AnnotatedDefinition, Definition},
};

use std::collections::HashMap;

pub fn load_annotation(
    annotated_definition: AnnotatedDefinition,
    _imports: &mut Vec<Import>,
    _circuits: &mut HashMap<Identifier, Circuit>,
    _functions: &mut HashMap<Identifier, Function>,
    tests: &mut HashMap<Identifier, TestFunction>,
    _expected: &mut Vec<InputVariable>,
) {
    let ast_annotation = annotated_definition.annotation;
    let ast_definition = *annotated_definition.definition;

    match ast_definition {
        Definition::Import(_) => unimplemented!("annotated imports are not supported yet"),
        Definition::Circuit(_) => unimplemented!("annotated circuits are not supported yet"),
        Definition::Function(_) => unimplemented!("annotated functions are not supported yet"),
        Definition::TestFunction(ast_test) => {
            let test = TestFunction::from(ast_test);
            load_annotated_test(test, ast_annotation, tests)
        }
        Definition::Annotated(_) => unimplemented!("nested annotations are not supported yet"),
    }
}

pub fn load_annotated_test(test: TestFunction, annotation: Annotation, tests: &mut HashMap<Identifier, TestFunction>) {
    let name = annotation.name;
    let ast_arguments = annotation.arguments;

    match name {
        AnnotationName::Context(_) => load_annotated_test_context(test, ast_arguments, tests),
    }
}

pub fn load_annotated_test_context(
    mut test: TestFunction,
    ast_arguments: AnnotationArguments,
    tests: &mut HashMap<Identifier, TestFunction>,
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
