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
        Mapping::get_or_init(tokens, addr, amount);
        tokens.get_or_init(addr, amount);
        Mapping::get_or_init(amounts, 1u8, amount);
        amounts.get_or_init(1u8, amount);
        Mapping::get_or_init(amounts, addr, 1u8);
        amounts.get_or_init(addr, 1u8);
        Mapping::get_or_init(tokens, addr, Token { owner: addr, gates: 1u8, amount: 1u8 });
        tokens.get_or_init(addr, Token { owner: addr, gates: 1u8, amount: 1u8 });
        Mapping::get_or_init(foo, addr, amount);
        foo.get_or_init(addr, amount);
    }
}
