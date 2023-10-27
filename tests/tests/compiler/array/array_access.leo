/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo(a: [bool; 8]) -> bool {
        return a[0u32];
    }
}
