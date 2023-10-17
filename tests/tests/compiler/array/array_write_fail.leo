/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo(a: [bool; 8]) -> [bool; 8] {
        a[0u32] = false;
        a[1u32] = true;
        return a;
    }
}
