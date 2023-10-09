/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo(a: [bool; 33]) -> bool {
        return true;
    }
}
