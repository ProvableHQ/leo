use crate::{assert_satisfied, parse_program};

#[test]
fn test_out_of_order() {
    let program_bytes = include_bytes!("out_of_order.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_import_fail() {
    let program_bytes = include_bytes!("import_fail.leo");

    let syntax_error = parse_program(program_bytes).is_err();

    assert!(syntax_error);
}
