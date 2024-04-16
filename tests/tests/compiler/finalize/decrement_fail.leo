/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping amounts: address => u128;
    
    async transition decrease_self(amount: u128) -> Future {
        return finalize_dec(self.caller, amount);
    }
    
    async function finalize_dec(addr: address, amount: u128) {
        decrement(amounts, addr, amount);
    }
}
