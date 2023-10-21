/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo(a: [bool; 8]) -> [bool; 8] {
        if a[0u32] {
            a[1u32] = false;
        } else {
            a[1u32] = true;
        }
        return a;
    }
}
