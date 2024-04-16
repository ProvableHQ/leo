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
    
    async transition decrease_self(amount: u128) -> Future {
        return finalize_decrease_self(self.caller, amount);
    }
    
    async function finalize_decrease_self(addr: address, amount: u128) {
        Mapping::get_or_use(tokens, addr, amount);
        tokens.get_or_use(addr, amount);
        Mapping::get_or_use(amounts, 1u8, amount);
        amounts.get_or_use(1u8, amount);
        Mapping::get_or_use(amounts, addr, 1u8);
        amounts.get_or_use(addr, 1u8);
        Mapping::get_or_use(tokens, addr, Token { owner: addr, amount: 1u8 });
        tokens.get_or_use(addr, Token { owner: addr, amount: 1u8 });
        Mapping::get_or_use(foo, addr, amount);
        foo.get_or_use(addr, amount);
    }
}
