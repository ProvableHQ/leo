/*
namespace: Execute
expectation: Pass
cases:
    main:
    - input: ["0u32", "1u32"]
    - input: ["1u32", "1u32"]
*/

program test.aleo {
    transition main(x: u32, y: u32) {
        assert_eq(x, y);
    }
}
