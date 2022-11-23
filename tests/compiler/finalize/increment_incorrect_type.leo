/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token {
        owner: address,
        gates: u64,
        amount: u128,
    }
    
    mapping amounts: address => u128;
    mapping tokens: address => Token;
    
    transition increase_self(amount: u128) {
        return then finalize(self.caller, amount);
    }
    
    finalize increase_self(addr: address, amount: u128) {
        increment(tokens, addr, amount);
        increment(amounts, 1u8, amount);
        increment(amounts, addr, 1u8);
        increment(amounts, addr, Token { owner: addr, gates: 1u64, amount: amount });
        increment(foo, addr, amount);
    }
}
