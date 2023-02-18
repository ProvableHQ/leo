/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    struct Amount {
        amount: u64,
        amt: u64,
    }
    
    record Token {
        // The token owner.
        owner: address,
        // The Aleo balance (in gates).
        gates: u64,
        // The token amount.
        amount: Amount,
    }
    
    transition mint(r0: address, r1: u64) -> Token {
        return Token {
            owner: r0,
            gates: 0u64,
            amount: Amount { amount: r1, amt: r1 },
        };
    }
    
    transition main(x: address) -> u64 {
        let c: u64 = 1u64;
        let t: Token = Token { owner: x, gates: 0u64, amount: Amount { amount: c, amt: c } };
    
        return t.gates;
    }
}
