/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record Token {
        owner: address,
        amounts: (u64, u64),
    }
}

