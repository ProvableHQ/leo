/*
namespace: Execute
expectation: Pass
cases:
    main:
    - input: ["0group"]
    - input: ["2group"]
*/

program test.aleo {
    transition main(a: group) -> (field, field) {
        let b: field = group::to_x_coordinate(a);
        let c: field = a as field;
        assert_eq(b, c);
        let d: field = group::to_y_coordinate(a);
        return (b, d);
    }
}
