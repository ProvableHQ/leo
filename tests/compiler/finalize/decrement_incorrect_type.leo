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
    
    transition decrease_self(amount: u128) {
        return then finalize(self.caller, amount);
    }
    
    finalize decrease_self(addr: address, amount: u128) {
        decrement(tokens, addr, amount);
        decrement(amounts, 1u8, amount);
        decrement(amounts, addr, 1u8);
        decrement(tokens, addr, Token { owner: addr, gates: 1u8, amount: 1u8 });
        decrement(foo, addr, amount);
    }
}
