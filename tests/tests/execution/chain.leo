/*
namespace: Execute
expectation: Pass
cases:
    main:
    - input: ["1u32"]
    - input: ["2u32"]
    - input: ["3u32"]
    - input: ["4u32"]
*/

program test.aleo {
    transition main(x: u32) -> bool {
        let c: u32 = 0u32;

        if x == 1u32 {
            c = 1u32;
        } else if x == 2u32 {
            c = 2u32;
        } else {
            c = 3u32;
        }
        return c == x;
    }
}
