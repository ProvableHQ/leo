/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token {
        owner: address,
        amount: u128,
    }
    
    mapping amounts: address => u128;
    mapping tokens: address => Token;
    
    transition decrease_self(amount: u128) {
        return then finalize(self.caller, amount);
    }
    
    finalize decrease_self(addr: address, amount: u128) {
        Mapping::set(tokens, addr, amount);
        tokens.set(addr, amount);
        Mapping::set(amounts, 1u8, amount);
        amounts.set(1u8, amount);
        Mapping::set(amounts, addr, 1u8);
        amounts.set(addr, 1u8);
        Mapping::set(tokens, addr, Token { owner: addr, amount: 1u8 });
        tokens.set(addr, Token { owner: addr, amount: 1u8 });
        Mapping::set(foo, addr, amount);
        foo.set(addr, amount);
    }
}
