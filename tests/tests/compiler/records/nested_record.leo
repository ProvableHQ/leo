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
        // The token amount.
        amount: Amount,
    }
    
    transition mint(r0: address, r1: u64) -> Token {
        return Token {
            owner: r0,
            amount: Amount { amount: r1, amt: r1 },
        };
    }
    
    transition main(x: address) -> u64 {
        let c: u64 = 1u64;
        let t: Token = Token { owner: x, amount: Amount { amount: c, amt: c } };

        return c + c;
    }
}
