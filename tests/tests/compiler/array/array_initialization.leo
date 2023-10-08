/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition bar(a: bool) -> [bool; 8] {
        let result: [bool; 8] = [a, a, a, a, a, a, a, a];
        return result;
    }
}
