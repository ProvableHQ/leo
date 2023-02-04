/*
namespace: Execute
expectation: Match
cases:
    main:
    - inputs: ["1u32"]
      expected: ["true"]
    - inputs: ["2u32"]
      expected: ["true"]
    - inputs: ["3u32"]
      expected: ["true"]
    - inputs: ["4u32"]
      expected: ["false"]
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
