use crate::{integers::u32::output_one, parse_program};

#[test]
fn test_basic() {
    let bytes = include_bytes!("basic.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
fn test_multiple() {
    let bytes = include_bytes!("multiple.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
fn test_star() {
    let bytes = include_bytes!("star.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
fn test_alias() {
    let bytes = include_bytes!("alias.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}
