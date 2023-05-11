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
        Mapping::set(tokens, addr, amount, 1u128);
        tokens.set(addr, amount, 1u128);
        Mapping::set(amounts, 1u8);
        amounts.set(1u8);
        Mapping::set(amounts);
        amounts.set();
    }
}
