/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    record Token {
        // The token owner.
        owner: address,
        // The Aleo balance (in gates).
        gates: u64,
        // The token amount.
        amount: u64,
    }
    
    transition mint(owner: address, amount: u64) -> Token {
        return Token {
            owner,
            gates: 0u64,
            amount,
        };
    }
    
    transition main(x: address) -> u64 {
        const c: u64 = 1u64;
        let t: Token = Token { owner: x, gates: 0u64, amount: c};
    
        return t.gates;
    }}
