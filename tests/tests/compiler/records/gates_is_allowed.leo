/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
        // `gates` as a normal field.
        gates: u64,
    }

    transition main(a: u64, b:u64) -> Token {
        return Token { owner: self.caller, amount: a, gates: b};
    }
}
