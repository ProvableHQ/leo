/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    mapping amounts: address => u128;
    
    transition decrease_self(amount: u128) {
        async finalize(self.caller, amount);
    }
    
    finalize decrease_self(addr: address, amount: u128) {
        decrement(amounts, addr, amount);
    }
}
