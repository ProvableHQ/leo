/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping amounts: address => u128;
    
    transition decrease_self(amount: u128) {
        return then finalize(self.caller, amount);
    }
    
    finalize decrease_self(addr: address, amount: u128) {
        decrement(amounts, addr, amount);
    }
}
