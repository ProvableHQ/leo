use crate::{assert_satisfied, import::set_local_dir, parse_program};

#[test]
fn test_out_of_order() {
    set_local_dir();

    let program_bytes = include_bytes!("out_of_order.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}
