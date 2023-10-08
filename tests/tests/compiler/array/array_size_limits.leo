/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition foo(a: [bool; 1]) -> bool {
        return true;
    }

    transition bar(a: [bool; 32]) -> bool {
        return true;
    }
}
