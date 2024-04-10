/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping amounts: address => u128;
    
    async transition increase_self(amount: u128) -> Future {
        return finalize_increase_self(self.caller, amount);
    }
    
    async function finalize_increase_self(addr: address, amount: u128) {
        increment(amounts, addr, amount);
    }
}
