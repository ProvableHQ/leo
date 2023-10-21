/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo(a: [bool; 8]) -> [bool; 8] {
        for i: u8 in 0u8..8u8 {
            a[i] = false;
        }
        return a;
    }
}
