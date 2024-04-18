/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    record Token {
        // The token amount.
        amount: u64,
        // The token owner.
        owner: address,
    }

    transition main(a: u64, b:u64) -> Token {
        return Token { amount: a + b, owner: self.caller};
    }
}
