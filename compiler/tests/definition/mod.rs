use crate::{assert_satisfied, import::set_local_dir, parse_program};

#[test]
fn test_out_of_order() {
    let program_bytes = include_bytes!("out_of_order.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_out_of_order_with_import() {
    set_local_dir();

    let program_bytes = include_bytes!("out_of_order_with_import.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}
