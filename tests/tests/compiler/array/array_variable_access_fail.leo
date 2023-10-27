/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo(a: [bool; 8], index: u32) -> bool {
        return a[index];
    }
}
