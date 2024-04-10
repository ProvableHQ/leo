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
        Mapping::set(tokens, addr, amount, 1u128);
        tokens.set(addr, amount, 1u128);
        Mapping::set(amounts, 1u8);
        amounts.set(1u8);
        Mapping::set(amounts);
        amounts.set();
    }
}
