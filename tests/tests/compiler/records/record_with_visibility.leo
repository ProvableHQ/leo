/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    record Token {
        // The token amount.
        private amount: u64,
        // The token owner.
        private owner: address,
        // A constant value.
        flag: bool,
    }

    transition main(a: u64, b:u64) -> Token {
        return Token { amount: a + b, owner: self.caller, flag: true};
    }
}
