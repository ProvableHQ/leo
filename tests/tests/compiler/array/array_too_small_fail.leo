/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo(a: [bool; 0]) -> bool {
        return true;
    }
}
