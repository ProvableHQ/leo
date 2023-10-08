/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition bar(a: bool) -> [bool; 8] {
        return [a, a, a, a, a, a, a, a];
    }
}
