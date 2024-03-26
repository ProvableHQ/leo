/*
namespace: Execute
expectation: Pass
cases:
    - program: test.aleo
      function: main
      input: ["1u32"]
    - program: test.aleo
      function: main
      input: ["2u32"]
    - program: test.aleo
      function: main
      input: ["3u32"]
    - program: test.aleo
      function: main
      input: ["4u32"]
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
