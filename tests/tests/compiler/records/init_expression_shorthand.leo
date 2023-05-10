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
    
    transition mint(owner: address, amount: u64) -> Token {
        return Token {
            owner,
            amount,
        };
    }
    
    transition main(x: address) -> u64 {
        let c: u64 = 1u64;
        let t: Token = Token { owner: x, amount: c};

        return c + c;
    }}
