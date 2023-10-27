/*
namespace: Compile
expectation: Pass
*/


program test.aleo {
    transition foo(a: [bool; 4]) {
        for i: u32 in 0u32..4u32 {
            assert(a[i]);
        }
    }
}
