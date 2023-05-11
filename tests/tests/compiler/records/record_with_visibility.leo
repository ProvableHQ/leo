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
        constant flag: bool,
    }

    transition main(a: u8, b:u8) -> u8 {
        return a + b;
    }
}
