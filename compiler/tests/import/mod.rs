use crate::{compile_program, integers::u32::output_one};

const DIRECTORY_NAME: &str = "tests/import/";

#[test]
fn test_basic() {
    let program = compile_program(DIRECTORY_NAME, "basic.leo").unwrap();
    output_one(program);
}

#[test]
fn test_multiple() {
    let program = compile_program(DIRECTORY_NAME, "multiple.leo").unwrap();
    output_one(program);
}

#[test]
fn test_star() {
    let program = compile_program(DIRECTORY_NAME, "star.leo").unwrap();
    output_one(program);
}

#[test]
fn test_alias() {
    let program = compile_program(DIRECTORY_NAME, "alias.leo").unwrap();
    output_one(program);
}
