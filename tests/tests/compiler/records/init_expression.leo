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
    
    transition mint(r0: address, r1: u64) -> Token {
        return Token {
            owner: r0,
            amount: r1,
        };
    }
    
    transition main(x: address) -> u64 {
        let c: u64 = 1u64;
        let t: Token = Token { owner: x, amount: c};

        return c * c;
    }}
