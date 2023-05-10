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

    transition main(a: u8, b:u8) -> u8 {
        return a + b;
    }
}
