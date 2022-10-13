/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record Token {
        owner: address,
        gates: u64,
        amounts: (u64, u64),
    }
}

