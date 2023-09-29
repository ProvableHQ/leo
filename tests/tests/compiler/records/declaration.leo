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
    }
    
    transition main(a: u8, b:u8) -> u8 {
        return a + b;
    }
}
