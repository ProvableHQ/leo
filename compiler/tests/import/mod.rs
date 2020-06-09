use crate::{integers::u32::output_one, parse_program};

// Import tests rely on knowledge of local directories. They should be run locally only.

#[test]
#[ignore]
fn test_basic() {
    let bytes = include_bytes!("basic.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
#[ignore]
fn test_multiple() {
    let bytes = include_bytes!("multiple.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
#[ignore]
fn test_star() {
    let bytes = include_bytes!("star.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
#[ignore]
fn test_alias() {
    let bytes = include_bytes!("alias.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}
